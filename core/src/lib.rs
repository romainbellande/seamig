use bae::FromAttributes;

#[derive(
    Debug,
    Eq,
    PartialEq,
    FromAttributes,
)]
pub struct Column {
    pub ty: syn::Ident,
}
