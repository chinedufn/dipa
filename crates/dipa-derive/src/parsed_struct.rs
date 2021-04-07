use crate::multi_field_utils::StructOrTupleField;
use quote::__private::Span;
use syn::Ident;

mod validate_attributes;

/// Information into a struct stored in a format that is useful for generating our dipa
/// implementation regardless of whether or not it was a struct with named, unnamed or no fields.
pub struct ParsedStruct {
    pub name: Ident,
    pub fields: Vec<StructOrTupleField>,
    pub fields_span: Span,
}
