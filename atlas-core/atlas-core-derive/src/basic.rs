use syn::{parse_quote, Attribute, DataStruct, Meta};

use crate::EMBED;

/// Finds and returns fields with the simple `#[embed]` attribute tag only.
/// This function filters fields within a `DataStruct` to identify those
/// explicitly tagged with the `#[embed]` attribute. It ensures that only
/// fields annotated for embedding are selected.
pub(crate) fn basic_embed_fields(data_struct: &DataStruct) -> impl Iterator<Item = &syn::Field> {
    data_struct.fields.iter().filter(|field| {
        field.attrs.iter().any(|attribute| matches!(attribute,
            Attribute {
                meta: Meta::Path(path),
                ..
            } if path.is_ident(EMBED)))
    })
}

/// Adds bounds to the `where` clause to ensure all fields tagged with `#[embed]` implement the `Embed` trait.
/// 
/// This enforces that types of fields marked with `#[embed]` comply with the `Embed` trait constraint,
/// enabling safer and more predictable behavior during embedding-related operations.
/// 
/// # Parameters:
/// - `generics`: Mutable reference to the generics of the struct.
/// - `field_type`: The type of the field to which the `Embed` trait should apply.
pub(crate) fn add_struct_bounds(generics: &mut syn::Generics, field_type: &syn::Type) {
    let where_clause = generics.make_where_clause();

    where_clause.predicates.push(parse_quote! {
        #field_type: Embed
    });
}
