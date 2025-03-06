mod sequence;

use sequence::derive_axdr_sequence;
use sequence::derive_toaxdr_sequence;

synstructure::decl_derive!([AxdrSequence, attributes(
    debug_derive,
    default,
    optional,
)] => derive_axdr_sequence);

synstructure::decl_derive!([ToAxdrSequence, attributes(
    debug_derive,
)] => derive_toaxdr_sequence);
