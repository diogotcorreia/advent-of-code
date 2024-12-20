use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Lit};

/// Derive macro to generate a TryFrom<char> implementation for
/// a given enum. Variants can be annotated with #[char_repr = '?'],
/// which will convert that character into that variant.
/// It is possible to annotate variants multiple times.
/// Variants that have fields cannot be annotated with #[char_repr = '?'].
/// If a character doesn't match any variant, it will return a
/// DayError::TryFromCharErr.
#[proc_macro_derive(TryFromChar, attributes(char_repr))]
pub fn try_from_char_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    if let syn::Data::Enum(data) = ast.data {
        let attr_name = format_ident!("char_repr");
        let input_type = &ast.ident;

        let cases = data.variants.iter().flat_map(|variant| {
            let variant_ident = &variant.ident;
            // Collect all char_repr attributes and generate match cases.
            variant
                .attrs
                .iter()
                .filter_map(|attr| {
                    let name_value = attr.meta.require_name_value().ok()?;

                    if name_value.path.is_ident(&attr_name) {
                        if let Expr::Lit(expr_lit) = &name_value.value {
                            if let Lit::Char(c) = &expr_lit.lit {
                                return Some(c);
                            }
                        }

                        panic!("Value of char_repr must be a char literal")
                    } else {
                        None
                    }
                })
                .inspect(|_| {
                    if !variant.fields.is_empty() {
                        panic!("Can't use char_repr on variants with fields")
                    }
                })
                .map(move |lit_char| {
                    quote! {
                        #lit_char => #input_type::#variant_ident,
                    }
                })
        });

        quote! {
            impl std::convert::TryFrom<char> for #input_type {
                type Error = aoc_common::DayError;

                #[inline]
                fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
                    std::result::Result::Ok(match value {
                        #(#cases)*
                        _ => return std::result::Result::Err(aoc_common::DayError::TryFromCharErr(stringify!(#input_type))),
                    })
                }
            }
        }.into()
    } else {
        panic!("Only enums can derive(TryFromChar)");
    }
}
