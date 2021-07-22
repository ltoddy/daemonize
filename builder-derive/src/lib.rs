use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, AngleBracketedGenericArguments, Data, DataStruct,
    DeriveInput, Error, Field, Fields::Named, FieldsNamed, GenericArgument, Ident, Path, PathArguments, PathSegment,
    Result, Token, Type, TypePath,
};

#[proc_macro_derive(Builder)]
pub fn derive(token: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token as DeriveInput);

    match derive_impl(&input) {
        Ok(token) => token.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_impl(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let DeriveInput { ident, .. } = ast;
    let fields = struct_fields(ast)?;

    let name = format!("{}Builder", ident.to_string());
    let builder_ident = Ident::new(&name, ident.span());

    let field_idents = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let builder_field_types = fields.iter().map(|f| inner_type(&f.ty)).collect::<Vec<_>>();

    let ast = quote! {
        pub struct #builder_ident {
            #(#field_idents: std::option::Option<#builder_field_types>),*
        }

        impl #builder_ident {
            #(pub fn #field_idents(&mut self, #field_idents: #builder_field_types) -> &mut Self {
                self.#field_idents = std::option::Option::Some(#field_idents);
                self
            })*
        }

        // impl #builder_ident {
        //     pub fn build(self) -> #ident {
        //         #ident {
        //             #(#field_idents: self.#field_idents.unwrap()),*
        //         }
        //     }
        // }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#field_idents: std::option::Option::None),*
                }
            }
        }
    };

    Ok(ast)
}

fn struct_fields(ast: &DeriveInput) -> Result<&Punctuated<Field, Token![,]>> {
    let DeriveInput { data, .. } = ast;

    if let Data::Struct(DataStruct { fields: Named(FieldsNamed { named, .. }), .. }) = data {
        return Ok(named);
    }

    Err(Error::new(ast.span(), "only derive struct"))
}

// e.g. if ty is: Option<MyStruct>, return MyStruct
// if ty is MyStruct, return MyStruct
fn inner_type(ty: &Type) -> &Type {
    if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = ty {
        if let Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) = segments.last() {
            if let Some(GenericArgument::Type(ty)) = args.last() {
                return ty;
            }
        }
    }
    ty
}
