mod sequence;

use sequence::{derive_axdr_sequence, derive_into_data, derive_toaxdr_sequence};

synstructure::decl_derive!([AxdrSequence, attributes(
    debug_derive,
    default,
    optional,
    tag,
)] => derive_axdr_sequence);

synstructure::decl_derive!([ToAxdrSequence, attributes(
    debug_derive,
    tag,
)] => derive_toaxdr_sequence);

synstructure::decl_derive!([IntoData, attributes(
    debug_derive,
)] => derive_into_data);
