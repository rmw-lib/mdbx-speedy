extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(MdbxSpeedy)]
pub fn mdbx_speedy(ts: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(ts).unwrap();
  let name = &ast.ident;
  quote! {
    impl mdbx::prelude::FromMdbx for #name {
      fn from_mdbx(_: mdbx::prelude::PtrTx, val: mdbx::prelude::MDBX_val) -> Self {
        Self::read_from_buffer(val_bytes!(val)).unwrap()
      }
    }

    impl mdbx::prelude::ToAsRef<#name, Vec<u8>> for #name {
      fn to_as_ref(&self) -> Vec<u8> {
        self.write_to_vec().unwrap()
      }
    }

  }
  .into()
}
