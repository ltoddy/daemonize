use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident, Result};

#[proc_macro_derive(Builder)]
pub fn derive(token: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token as DeriveInput);

    match derive_impl(&input) {
        Ok(token) => token.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_impl(ast: &DeriveInput) -> Result<TokenStream2> {
    let builder = generate_builder_struct(ast)?;

    Ok(quote! {
        #builder
    })
}

fn generate_builder_struct(ast: &DeriveInput) -> Result<TokenStream2> {
    let DeriveInput { vis, ident, .. } = ast;
    let fields = struct_fields(ast)?;

    let name = format!("{}Builder", ident.to_string());
    let ident = Ident::new(&name, ident.span());

    let field_idents = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let field_types = fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let ast = quote! {
        #vis struct #ident {
            #(#field_idents: std::option::Option<#field_types>),*
        }
    };

    Ok(ast)
}

fn struct_fields(ast: &DeriveInput) -> Result<&Fields> {
    let DeriveInput { data, .. } = ast;

    if let Data::Struct(DataStruct { fields, .. }) = data {
        return Ok(fields);
    }

    Err(syn::Error::new(ast.span(), "only derive struct"))
}
