use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

mod derive_aksiyon;
mod derive_into_plot;
mod derive_ogeye_donus;

/// icon_name! makrosu için girdi: EnumName, "yol", [isteğe bağlı derive listesi]
struct SimgeAdiGirdisi {
    enum_name: syn::Ident,
    _comma: syn::Token![,],
    path: syn::LitStr,
    derives: Option<(
        syn::Token![,],
        syn::punctuated::Punctuated<syn::Path, syn::Token![,]>,
    )>,
}

impl Parse for SimgeAdiGirdisi {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let enum_name = input.parse()?;
        let _comma = input.parse()?;
        let path = input.parse()?;

        // Check if there's an optional derives list
        let derives = if input.peek(syn::Token![,]) {
            let comma = input.parse()?;
            let content;
            syn::bracketed!(content in input);
            let derives = content.parse_terminated(syn::Path::parse, syn::Token![,])?;
            Some((comma, derives))
        } else {
            None
        };

        Ok(SimgeAdiGirdisi {
            enum_name,
            _comma,
            path,
            derives,
        })
    }
}

#[proc_macro_derive(IntoPlot)]
pub fn derive_into_plot(input: TokenStream) -> TokenStream {
    derive_into_plot::derive_into_plot(input)
}

#[proc_macro_derive(Aksiyon, attributes(action, aksiyon))]
pub fn derive_aksiyon(input: TokenStream) -> TokenStream {
    derive_aksiyon::derive_aksiyon(input)
}

#[proc_macro_derive(OgeyeDonus)]
pub fn derive_ogeye_donus(input: TokenStream) -> TokenStream {
    derive_ogeye_donus::derive_ogeye_donus(input)
}

/// Bir SVG dosya adını PascalCase tanımlayıcıya dönüştürür.
///
/// `.svg` uzantısını kaldırır, ayırıcılara (`-`, `_`, `.`) göre böler,
/// ve Rust adlandırma kurallarına göre her kelimeyi büyük harfle başlatır.
///
/// # Örnekler
///
/// ```ignore
/// assert_eq!(pascal_case("arrow-right.svg"), "ArrowRight");
/// assert_eq!(pascal_case("some_icon_name.svg"), "SomeIconName");
/// assert_eq!(pascal_case("icon-123.svg"), "Icon123");
/// ```
fn pascal_case(filename: &str) -> String {
    filename
        .strip_suffix(".svg")
        .unwrap_or(filename)
        .split(|c: char| c == '-' || c == '_' || c == '.')
        .filter(|part| !part.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) if first.is_ascii_digit() => word.to_string(),
                Some(first) => {
                    let mut result = String::with_capacity(word.len());
                    result.extend(first.to_uppercase());
                    result.push_str(&chars.as_str().to_lowercase());
                    result
                }
            }
        })
        .collect()
}

/// Bir SVG dosya dizinini tarayarak özel bir simge enumu ve onun `AdliSimge` uygulamasını üretir.
///
/// Bir enum adı, çağıran crate'içinde `CARGO_MANIFEST_DIR` değerine göre göreli bir yol,
/// ve isteğe bağlı olarak ek derive özellik listesini kabul eder.
///
/// # Örnek
///
/// ```ignore
/// // Basic usage (derives IntoElement, Clone by default)
/// simge_adli!(SimgeAdi, "../assets/assets/icons");
///
/// // With custom derives
/// simge_adli!(SimgeAdi, "../assets/assets/icons", [Debug, Copy, PartialEq, Eq]);
/// ```
#[proc_macro]
pub fn simge_adli(input: TokenStream) -> TokenStream {
    let SimgeAdiGirdisi {
        enum_name,
        path,
        derives,
        ..
    } = syn::parse_macro_input!(input as SimgeAdiGirdisi);

    let relative_path = path.value();

    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR ayarlanmamış");
    let icons_dir = std::path::Path::new(&manifest_dir).join(&relative_path);

    let mut entries: Vec<(String, String)> = Vec::new();

    let dir = std::fs::read_dir(&icons_dir).unwrap_or_else(|e| {
        panic!(
            "generate_icon_enum: failed to read '{}': {}",
            icons_dir.display(),
            e
        )
    });

    for entry in dir {
        let entry = entry.expect("dizin girdisi okunamadı");
        let filename = entry.file_name().to_string_lossy().to_string();
        if filename.ends_with(".svg") {
            let variant_name = pascal_case(&filename);
            let path = format!("icons/{}", filename);
            entries.push((variant_name, path));
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let variants: Vec<proc_macro2::Ident> = entries
        .iter()
        .map(|(name, _)| proc_macro2::Ident::new(name, proc_macro2::Span::call_site()))
        .collect();
    let paths: Vec<&str> = entries.iter().map(|(_, p)| p.as_str()).collect();

    // Build derive list: always include IntoElement and Clone, then add custom derives
    let derive_attrs = if let Some((_, custom_derives)) = derives {
        let derives_vec: Vec<_> = custom_derives.iter().collect();
        quote! {
            #[derive(IntoElement, Clone, #(#derives_vec),*)]
        }
    } else {
        quote! {
            #[derive(IntoElement, Clone)]
        }
    };

    let expanded = quote! {
        #derive_attrs
        pub enum #enum_name {
            #(#variants,)*
        }

        impl AdliSimge for #enum_name {
            fn path(self) -> SharedString {
                match self {
                    #(Self::#variants => #paths,)*
                }
                .into()
            }
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_case_basic() {
        assert_eq!(pascal_case("arrow-right.svg"), "ArrowRight");
        assert_eq!(pascal_case("home.svg"), "Home");
        assert_eq!(pascal_case("x-circle.svg"), "XCircle");

        assert_eq!(pascal_case("some_icon_name.svg"), "SomeIconName");
        assert_eq!(pascal_case("arrow_up_down.svg"), "ArrowUpDown");

        assert_eq!(pascal_case("kebab-case_mixed.svg"), "KebabCaseMixed");
        assert_eq!(pascal_case("icon-with_under.svg"), "IconWithUnder");

        assert_eq!(pascal_case("icon-123.svg"), "Icon123");
        assert_eq!(pascal_case("arrow-2x.svg"), "Arrow2x");
        assert_eq!(pascal_case("24-hour.svg"), "24Hour");

        assert_eq!(pascal_case("arrow--right.svg"), "ArrowRight");
        assert_eq!(pascal_case("icon__name.svg"), "IconName");
        assert_eq!(pascal_case("multiple---dash.svg"), "MultipleDash");

        assert_eq!(pascal_case("a.svg"), "A");
        assert_eq!(pascal_case("-leading.svg"), "Leading");
        assert_eq!(pascal_case("trailing-.svg"), "Trailing");
        assert_eq!(pascal_case("-.svg"), "");

        assert_eq!(pascal_case("arrow-right"), "ArrowRight");
        assert_eq!(pascal_case("home"), "Home");

        assert_eq!(pascal_case("hello.svg"), "Hello");
        assert_eq!(pascal_case("WORLD.svg"), "World");
        assert_eq!(pascal_case("iOS-icon.svg"), "IosIcon");
        assert_eq!(pascal_case("API-key.svg"), "ApiKey");
    }
}
