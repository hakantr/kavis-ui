use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_into_plot(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics kavis_ui::IntoElement for #type_name #type_generics #where_clause {
            type Element = Self;

            fn into_element(self) -> Self::Element {
                self
            }
        }

        impl #impl_generics kavis_ui::Element for #type_name #type_generics #where_clause {
            type RequestLayoutState = ();
            type PrepaintState = ();

            fn id(&self) -> Option<kavis_ui::ElementId> {
                None
            }

            fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
                None
            }

            fn request_layout(
                &mut self,
                _: Option<&kavis_ui::ham_gpui::GlobalElementId>,
                _: Option<&kavis_ui::ham_gpui::InspectorElementId>,
                window: &mut kavis_ui::Window,
                cx: &mut kavis_ui::App,
            ) -> (kavis_ui::ham_gpui::LayoutId, Self::RequestLayoutState) {
                let style = kavis_ui::ham_gpui::Style {
                    size: kavis_ui::ham_gpui::Size::full(),
                    ..Default::default()
                };

                (window.request_layout(style, None, cx), ())
            }

            fn prepaint(
                &mut self,
                _: Option<&kavis_ui::ham_gpui::GlobalElementId>,
                _: Option<&kavis_ui::ham_gpui::InspectorElementId>,
                _: kavis_ui::Bounds<kavis_ui::Pixels>,
                _: &mut Self::RequestLayoutState,
                _: &mut kavis_ui::Window,
                _: &mut kavis_ui::App,
            ) -> Self::PrepaintState {
            }

            fn paint(
                &mut self,
                _: Option<&kavis_ui::ham_gpui::GlobalElementId>,
                _: Option<&kavis_ui::ham_gpui::InspectorElementId>,
                bounds: kavis_ui::Bounds<kavis_ui::Pixels>,
                _: &mut Self::RequestLayoutState,
                _: &mut Self::PrepaintState,
                window: &mut kavis_ui::Window,
                cx: &mut kavis_ui::App,
            ) {
                <Self as Plot>::paint(self, bounds, window, cx)
            }
        }
    };

    TokenStream::from(expanded)
}
