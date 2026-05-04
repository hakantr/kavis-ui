use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub(crate) fn derive_ogeye_donus(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics kavis_ui::IntoElement for #type_name #type_generics
        #where_clause
        {
            type Element = kavis_ui::ham_gpui::Component<Self>;

            #[track_caller]
            fn into_element(self) -> Self::Element {
                kavis_ui::ham_gpui::Component::new(self)
            }
        }
    };

    expanded.into()
}
