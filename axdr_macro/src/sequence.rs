use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_quote, spanned::Spanned, Attribute, Ident, Lifetime, Meta, WherePredicate, LitInt
};

fn get_attribute_meta(attr: &Attribute) -> Result<TokenStream, syn::Error> {
    if let Meta::List(meta) = &attr.meta {
        let content = &meta.tokens;
        Ok(quote! { #content })
    } else {
        Err(syn::Error::new(
            attr.span(),
            "Invalid error attribute format",
        ))
    }
}

fn get_tag_attribute(attrs: &[syn::Attribute]) -> Option<u8> {
    attrs.iter()
        .find(|attr| {
            eprintln!("{:?}", attr.path());
            attr.path().is_ident("tag")
        })
        .and_then(|attr| {
            let lit: LitInt = attr.parse_args().ok()?;
            lit.base10_parse::<u8>().ok()
        })
}

pub fn derive_axdr_sequence(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let ast = s.ast();

    let debug_derive = ast.attrs.iter().any(|attr| {
        attr.meta
            .path()
            .is_ident(&Ident::new("debug_derive", Span::call_site()))
    });

    // 只支持struct enum
    let lifetime = Lifetime::new("'axdr", Span::call_site());
    let lfts: Vec<_> = ast.generics.lifetimes().collect();
    let mut whs = Vec::new();
    if !lfts.is_empty() {
        let lft = Lifetime::new("'axdr", Span::call_site());
        let wh: WherePredicate = parse_quote! { #lft: #(#lfts)+* };
        whs.push(wh);
    };

    let error = ast
        .attrs
        .iter()
        .find(|attr| {
            attr.meta
                .path()
                .is_ident(&Ident::new("error", Span::call_site()))
        })
        .map_or(quote! { asn1_rs::Error }, |attr| {
            get_attribute_meta(attr).expect("Invalid error attribute format")
        });

    // 区分结构体和枚举
    let fn_content = match &ast.data {
        syn::Data::Struct(ds) => {
            let field_names = &ds.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let field_extractions = field_names.iter().map(|name| quote! {
                let (bytes, #name) = FromAxdr::from_axdr(bytes)?;
            }).collect::<Vec<_>>();
            
            quote! {
                #(#field_extractions)*
                Ok((bytes, Self { #(#field_names),* }))
            }
        }
        syn::Data::Enum(_) => {
            s.bind_with(|_| synstructure::BindStyle::Move);
            let variants = s.variants().iter().map(|variant| {
                let pat = variant.pat();
                let bindings = variant.bindings().iter().map(|b| {
                    let ty = b.ast().ty.clone();
                    let name = b.pat();
                    quote! {
                        let (bytes, #name) = <#ty>::from_axdr(bytes)?;
                    }
                }).collect::<Vec<_>>();

                let tag_value = get_tag_attribute(&variant.ast().attrs)
                        .unwrap_or_else(|| panic!("Missing #[tag(n)] attribute on variant {}", variant.ast().ident));

                quote! {
                    #tag_value => {
                        #(#bindings)*
                        Ok((bytes, #pat))
                    }
                }
            }).collect::<Vec<_>>();
            
            quote! {
                let (bytes, tag) = u8::from_axdr(bytes)?;
                    
                match tag {
                    #(#variants)*
                    _ => Err(asn1_rs::Error::InvalidTag.into())
                }
            }
        }
        _ => panic!("Only structs and enums are supported"),
    };

    let ts = s.gen_impl(quote! {
        use asn1_type::traits::FromAxdr;

        gen impl<#lifetime> FromAxdr<#lifetime, #error> for @Self where #(#whs)+* {
            fn from_axdr(bytes: &#lifetime [u8]) -> asn1_rs::ParseResult<#lifetime, Self, #error> {
                #fn_content
            }
        }
    });
    if debug_derive {
        eprintln!("{}", ts);
    }
    ts
}

fn gen_to_axdr_len(s: &mut synstructure::Structure) -> TokenStream {
    match &s.ast().data {
        syn::Data::Struct(ds) => {
            // 使用 fold 方法累积所有字段的长度
            let field_names = &ds.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let len_expr = field_names.iter().fold(quote!(0), |acc, name| {
                quote! { #acc + self.#name.to_axdr_len()? }
            });
            
            quote! {
                fn to_axdr_len(&self) -> asn1_rs::Result<usize> {
                    Ok(#len_expr)
                }
            }
        },
        syn::Data::Enum(_) => {
            let body = s.fold(quote! {1}, |acc, bi|
                quote!(#acc + #bi.to_axdr_len()?)
            );
            
            quote! {
                fn to_axdr_len(&self) -> asn1_rs::Result<usize> {
                    Ok(match self {
                        #body
                    })
                }
            }
        },
        _ => panic!("Only structs and enums are supported")
    }
}

fn gen_write_axdr_header(s: &mut synstructure::Structure) -> TokenStream {
    match &s.ast().data {
        syn::Data::Struct(_) => {
            // 结构体不需要写入头部
            quote! {
                fn write_axdr_header(&self, _writer: &mut dyn std::io::Write) -> asn1_rs::SerializeResult<usize> {
                    Ok(0)
                }
            }
        }
        syn::Data::Enum(_) => {
            let body = s.each_variant(|v| {
                let tag_value = get_tag_attribute(&v.ast().attrs)
                    .unwrap_or_else(|| panic!("Missing #[tag(n)] attribute on variant {}", v.ast().ident));
                
                quote! {
                    writer.write_all(&[#tag_value])?;
                }   
            });

            quote! {
                fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> asn1_rs::SerializeResult<usize> {
                    match self {
                        #body
                    }
                    Ok(1)
                }
            }
        }
        _ => panic!("Only structs and enums are supported"),
    }
}

fn gen_write_axdr_content(s: &mut synstructure::Structure) -> TokenStream {
    match &s.ast().data {
        syn::Data::Struct(ds) => {
            // 使用 each 方法遍历所有字段绑定
            let field_names = &ds.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let write_instructions = field_names.iter().fold(Vec::new(), |mut instrs, field| {
                instrs.push(quote! {num_bytes += self.#field.write_axdr_header(writer)?;});
                instrs.push(quote! {num_bytes += self.#field.write_axdr_content(writer)?;});
                instrs
            });
            
            quote! {
                fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> asn1_rs::SerializeResult<usize> {
                    let mut num_bytes = 0;
                    #(#write_instructions)*
                    Ok(num_bytes)
                }
            }
        },
        syn::Data::Enum(_) => {
            let body = s.each(|bi|
                quote! {
                    num_bytes += #bi.write_axdr_header(writer)?;
                    num_bytes += #bi.write_axdr_content(writer)?;
                }
            );
            
            quote! {
                fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> asn1_rs::SerializeResult<usize> {
                    let mut num_bytes = 0;
                    match self {
                        #body
                    }
                    Ok(num_bytes)
                }
            }
        },
        _ => panic!("Only structs and enums are supported")
    }
}

pub fn derive_toaxdr_sequence(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let ast = s.ast();

    let debug_derive = ast.attrs.iter().any(|attr| {
        attr.meta
            .path()
            .is_ident(&Ident::new("debug_derive", Span::call_site()))
    });

    let impl_to_axdr_len = gen_to_axdr_len(&mut s);
    let impl_write_axdr_header = gen_write_axdr_header(&mut s);
    let impl_write_axdr_content = gen_write_axdr_content(&mut s);

    let ts = s.gen_impl(quote! {
        use asn1_type::traits::ToAxdr;

        gen impl ToAxdr for @Self {
            #impl_to_axdr_len
            #impl_write_axdr_header
            #impl_write_axdr_content
        }
    });
    if debug_derive {
        eprintln!("{}", ts);
    }
    ts
}
