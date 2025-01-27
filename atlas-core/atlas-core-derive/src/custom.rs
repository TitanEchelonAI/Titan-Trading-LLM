use quote::ToTokens;
use syn::{meta::ParseNestedMeta, ExprPath};

use crate::EMBED;

const EMBED_WITH: &str = "embed_with";

/// Finds and returns fields with `#[embed(embed_with = "...")]` attribute tags only.
/// Additionally, it extracts and returns the "..." portion of the attribute tag (i.e., the custom function).
/// 
/// # Parameters:
/// - `data_struct`: A reference to the `DataStruct` to be parsed.
/// 
/// # Returns:
/// A `Result` containing a vector of tuples where each tuple consists of a field reference and its associated `ExprPath` (custom function), or an error.
pub(crate) fn custom_embed_fields(
    data_struct: &syn::DataStruct,
) -> syn::Result<Vec<(&syn::Field, syn::ExprPath)>> {
    data_struct
        .fields
        .iter()
        .filter_map(|field| {
            field
                .attrs
                .iter()
                .filter_map(|attribute| match attribute.is_custom() {
                    Ok(true) => match attribute.expand_tag() {
                        Ok(path) => Some(Ok((field, path))),
                        Err(e) => Some(Err(e)),
                    },
                    Ok(false) => None,
                    Err(e) => Some(Err(e)),
                })
                .next()
        })
        .collect::<Result<Vec<_>, _>>()
}

/// A trait that provides methods to parse and handle custom attributes on struct fields.
trait CustomAttributeParser {
    /// Determines whether the attribute is a custom `#[embed(embed_with = "...")]` tag.
    fn is_custom(&self) -> syn::Result<bool>;

    /// Extracts and returns the "..." portion of the `#[embed(embed_with = "...")]` tag.
    /// 
    /// # Returns:
    /// A `Result` containing the parsed `ExprPath` for the custom embedding function, or an error.
    fn expand_tag(&self) -> syn::Result<syn::ExprPath>;
}

impl CustomAttributeParser for syn::Attribute {
    fn is_custom(&self) -> syn::Result<bool> {
        // Check that the attribute is a list.
        match &self.meta {
            syn::Meta::List(meta) => {
                if meta.tokens.is_empty() {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        };

        // Verify that the first attribute path matches `EMBED`
        if !self.path().is_ident(EMBED) {
            return Ok(false);
        }

        self.parse_nested_meta(|meta| {
            // Parse the attribute meta as an expression.
            meta.value()?.parse::<syn::Expr>()?;

            if meta.path.is_ident(EMBED_WITH) {
                Ok(())
            } else {
                let path = meta.path.to_token_stream().to_string().replace(' ', "");
                Err(syn::Error::new_spanned(
                    meta.path,
                    format_args!("unknown embedding field attribute `{}`", path),
                ))
            }
        })?;

        Ok(true)
    }

    fn expand_tag(&self) -> syn::Result<syn::ExprPath> {
        fn function_path(meta: &ParseNestedMeta<'_>) -> syn::Result<ExprPath> {
            // Extract the value from `#[embed(embed_with = "...")]`
            let expr = meta.value()?.parse::<syn::Expr>().unwrap();
            let mut value = &expr;
            while let syn::Expr::Group(e) = value {
                value = &e.expr;
            }
            let string = if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = value
            {
                let suffix = lit_str.suffix();
                if !suffix.is_empty() {
                    return Err(syn::Error::new_spanned(
                        lit_str,
                        format!("unexpected suffix `{}` on string literal", suffix),
                    ));
                }
                lit_str.clone()
            } else {
                return Err(syn::Error::new_spanned(
                    value,
                    format!(
                        "expected {} attribute to be a string: `{} = \"...\"`",
                        EMBED_WITH, EMBED_WITH
                    ),
                ));
            };

            string.parse()
        }

        let mut custom_func_path = None;

        self.parse_nested_meta(|meta| match function_path(&meta) {
            Ok(path) => {
                custom_func_path = Some(path);
                Ok(())
            }
            Err(e) => Err(e),
        })?;

        Ok(custom_func_path.unwrap())
    }
}
