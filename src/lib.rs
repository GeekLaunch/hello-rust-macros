use darling::{FromDeriveInput, FromVariant};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Path};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(display), supports(enum_any))]
struct EnumMeta {
    pub transform: Option<Path>,

    pub data: darling::ast::Data<VariantVisitor, ()>,
    pub generics: syn::Generics,
    pub ident: syn::Ident,
}

#[derive(Debug, FromVariant)]
struct VariantVisitor {
    pub ident: syn::Ident,
    pub fields: darling::ast::Fields<()>,
}

fn expand(meta: EnumMeta) -> Result<TokenStream2, darling::Error> {
    let EnumMeta {
        transform,
        data,
        generics,
        ident,
    } = meta;

    let variants = data.take_enum().unwrap();

    let match_arms = variants.iter().map(|variant| {
        let i = &variant.ident;
        let name = i.to_string();
        match variant.fields.style {
            darling::ast::Style::Tuple => {
                quote! { Self :: #i ( .. ) => #name , }
            }
            darling::ast::Style::Struct => {
                quote! { Self :: #i { .. } => #name , }
            }
            darling::ast::Style::Unit => {
                quote! { Self :: #i  => #name , }
            }
        }
    });

    // properly include generics in output
    let (imp, ty, wher) = generics.split_for_impl();

    Ok(quote! {
        impl #imp std::fmt::Display for #ident #ty #wher {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #transform (
                    match self { #(#match_arms)* }
                ))
            }
        }
    })
}

#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    FromDeriveInput::from_derive_input(&input)
        .and_then(expand)
        .map(Into::into)
        .unwrap_or_else(|e| e.write_errors().into())
}
