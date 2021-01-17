#![recursion_limit="128"]

use synstructure::decl_derive;
use quote::quote;

decl_derive!([ProofSize] => derive_proofsize);

fn derive_proofsize(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.fold(0 as usize, |acc, bi| quote!{
         #acc + ::rug_binserial::ProofSize::proof_size(#bi)
//            0 as usize
    });

    s.bound_impl(quote!(rug_binserial::ProofSize), quote!{
        fn proof_size(&self) -> usize {
            match *self {
                #body
            }
        }
    })
}
