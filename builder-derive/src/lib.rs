use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Builder)]
pub fn derive(token: TokenStream) -> TokenStream {
    TokenStream2::new().into()
}
