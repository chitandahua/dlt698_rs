use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::ParseStream, parse_quote, spanned::Spanned, Attribute, BinOp, DataStruct, DeriveInput, Field, Fields, Ident, Lifetime, LitInt, Meta, Type, WherePredicate
};
use asn1_rs::Err::Error;

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

pub fn derive_axdr_sequence(s: synstructure::Structure) -> proc_macro2::TokenStream {
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
    let from_axdr_impl = match ast.data {
        syn::Data::Struct(ref ds) => {
            let field_names = ds.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let field_extractions = s.each(|bi| quote! {
                let (bytes, #bi) = FromAxdr::from_axdr(bytes)?;
            });
            
            quote! {
                fn from_axdr(bytes: &#lifetime [u8]) -> asn1_rs::ParseResult<#lifetime, Self, #error> {
                    let mut bytes = bytes;
                    #field_extractions
                    Ok((bytes, Self { #(#field_names),* }))
                }
            }
        }
        // TODO
        syn::Data::Enum(_) => {
            let arms = s.variants().iter().map(|v| {
                let pat = v.pat();
                let variant_name = &v.ast().ident;
                let body = v.bindings().iter().map(|bi| {
                    quote! {
                        let (bytes, #bi) = FromAxdr::from_axdr(bytes)?;
                    }
                });

                quote! {
                    tag if tag == <Self as Choice>::tag().0 as u8 => {
                        #(#body)*
                        Ok((bytes, #pat))
                    }
                }
            });

            quote! {
                fn from_axdr(bytes: &#lifetime [u8]) -> asn1_rs::ParseResult<#lifetime, Self, #error> {
                    if bytes.is_empty() {
                        return Err(asn1_rs::Error::NomError(nom::Err::Incomplete(nom::Needed::Size(1))).into());
                    }

                    let (bytes, tag) = nom::number::complete::be_u8(bytes)
                        .map_err(|_| asn1_rs::Error::InvalidTag)?;

                    match tag {
                        #(#arms),*
                        _ => Err(asn1_rs::Error::InvalidTag.into())
                    }
                }
            }
        }
        _ => panic!("Only structs and enums are supported"),
    };

    let ts = s.gen_impl(quote! {
        extern crate dlt698_rs;
        use dlt698_rs::apdu::data_type::traits::FromAxdr;
        use dlt698_rs::apdu::data_type::choice::Choice;

        gen impl<#lifetime> FromAxdr<#lifetime, #error> for @Self where #(#whs)+* {
            type Error = #error;

            #from_axdr_impl
        }
    });
    if debug_derive {
        eprintln!("{}", ts);
    }
    ts
}

fn gen_to_axdr_len(s: &synstructure::Structure) -> TokenStream {
    match &s.ast().data {
        syn::Data::Struct(_) => {
            // 使用 fold 方法累积所有字段的长度
            let len_expr = s.fold(quote!(0), |acc, bi| {
                quote! { #acc + ToAxdr::to_axdr_len(#bi)? }
            });
            
            quote! {
                fn to_axdr_len(&self) -> asn1_rs::Result<usize> {
                    Ok(#len_expr)
                }
            }
        },
        syn::Data::Enum(_) => {
            let variants = s.variants().iter().map(|variant| {
                let pat = variant.pat();
                let len_expr = variant.bindings().iter().fold(quote!(1), |acc, bi| {
                    quote! { #acc + ToAxdr::to_axdr_len(#bi)? }
                });
                
                quote! {
                    #pat => {
                        Ok(#len_expr)  // 包含1字节的标签
                    }
                }
            }).collect::<Vec<_>>();
            
            quote! {
                fn to_axdr_len(&self) -> asn1_rs::Result<usize> {
                    match self {
                        #(#variants),*
                    }
                }
            }
        },
        _ => panic!("Only structs and enums are supported")
    }
}

fn gen_write_axdr_header(s: &synstructure::Structure) -> TokenStream {
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
            let body = s.variants().iter().map(|variant| {
                let pat = variant.pat();
                let variant_name = &variant.ast().ident;

                quote! {
                    #pat => {
                        let tag = <Self as Choice>::tag();
                        writer.write_all(&[tag.0 as u8])?;
                        Ok(1)
                    }
                }
            });

            quote! {
                fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> asn1_rs::SerializeResult<usize> {
                    match self {
                        #(#body),*
                    }
                }
            }
        }
        _ => panic!("Only structs and enums are supported"),
    }
}

fn gen_write_axdr_content(s: &synstructure::Structure) -> TokenStream {
    match &s.ast().data {
        syn::Data::Struct(_) => {
            // 使用 each 方法遍历所有字段绑定
            let field_writes = s.each(|bi| quote! {
                num_bytes += ToAxdr::write_axdr(#bi, writer)?;
            });
            
            quote! {
                fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> asn1_rs::SerializeResult<usize> {
                    let mut num_bytes = 0;
                    #field_writes
                    Ok(num_bytes)
                }
            }
        },
        syn::Data::Enum(_) => {
            let variants = s.variants().iter().map(|variant| {
                let pat = variant.pat();
                let field_writes = variant.bindings().iter().map(|bi| {
                    quote! {
                        num_bytes += ToAxdr::write_axdr(#bi, writer)?;
                    }
                }).collect::<Vec<_>>();
                
                quote! {
                    #pat => {
                        let mut num_bytes = 0;
                        #(#field_writes)*
                        Ok(num_bytes)
                    }
                }
            }).collect::<Vec<_>>();
            
            quote! {
                fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> asn1_rs::SerializeResult<usize> {
                    match self {
                        #(#variants),*
                    }
                }
            }
        },
        _ => panic!("Only structs and enums are supported")
    }
}

pub fn derive_toaxdr_sequence(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let ast = s.ast();

    let debug_derive = ast.attrs.iter().any(|attr| {
        attr.meta
            .path()
            .is_ident(&Ident::new("debug_derive", Span::call_site()))
    });

    let impl_to_axdr_len = gen_to_axdr_len(&s);
    let impl_write_axdr_header = gen_write_axdr_header(&s);
    let impl_write_axdr_content = gen_write_axdr_content(&s);

    let ts = s.gen_impl(quote! {
        extern crate dlt698_rs;
        use dlt698_rs::apdu::data_type::traits::ToAxdr;
        use dlt698_rs::apdu::data_type::choice::Choice;

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
