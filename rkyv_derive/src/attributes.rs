use quote::ToTokens;
use syn::{
    meta::ParseNestedMeta, parenthesized, parse::Parse, parse_quote,
    punctuated::Punctuated, AttrStyle, DeriveInput, Error, Ident, LitStr, Meta,
    Path, Token, WherePredicate,
};

fn try_set_attribute<T: ToTokens>(
    attribute: &mut Option<T>,
    value: T,
    name: &'static str,
) -> Result<(), Error> {
    if attribute.is_none() {
        *attribute = Some(value);
        Ok(())
    } else {
        Err(Error::new_spanned(
            value,
            format!("{} already specified", name),
        ))
    }
}

#[derive(Default)]
pub struct Attributes {
    pub archive_as: Option<LitStr>,
    pub archived: Option<Ident>,
    pub resolver: Option<Ident>,
    pub attrs: Vec<Meta>,
    pub compares: Option<Punctuated<Path, Token![,]>>,
    pub archive_bounds: Option<Punctuated<WherePredicate, Token![,]>>,
    pub serialize_bounds: Option<Punctuated<WherePredicate, Token![,]>>,
    pub deserialize_bounds: Option<Punctuated<WherePredicate, Token![,]>>,
    pub check_bytes: Option<Path>,
    pub copy_safe: Option<Path>,
    rkyv_path: Option<Path>,
}

impl Attributes {
    fn parse_meta(&mut self, meta: ParseNestedMeta<'_>) -> Result<(), Error> {
        if meta.path.is_ident("check_bytes") {
            if !meta.input.is_empty() && !meta.input.peek(Token![,]) {
                return Err(meta.error("check_bytes argument must be a path"));
            }

            try_set_attribute(&mut self.check_bytes, meta.path, "check_bytes")
        } else if meta.path.is_ident("copy_safe") {
            if !meta.input.is_empty() && !meta.input.peek(Token![,]) {
                return Err(meta.error("copy_safe argument must be a path"));
            }

            try_set_attribute(&mut self.copy_safe, meta.path, "copy_safe")
        } else if meta.path.is_ident("compare") {
            let traits;
            parenthesized!(traits in meta.input);
            let traits = traits.parse_terminated(Path::parse, Token![,])?;
            try_set_attribute(&mut self.compares, traits, "compare")
        } else if meta.path.is_ident("archive_bounds") {
            let bounds;
            parenthesized!(bounds in meta.input);
            let clauses =
                bounds.parse_terminated(WherePredicate::parse, Token![,])?;
            try_set_attribute(
                &mut self.archive_bounds,
                clauses,
                "archive_bounds",
            )
        } else if meta.path.is_ident("serialize_bounds") {
            let bounds;
            parenthesized!(bounds in meta.input);
            let clauses =
                bounds.parse_terminated(WherePredicate::parse, Token![,])?;
            try_set_attribute(
                &mut self.serialize_bounds,
                clauses,
                "serialize_bounds",
            )
        } else if meta.path.is_ident("deserialize_bounds") {
            let bounds;
            parenthesized!(bounds in meta.input);
            let clauses =
                bounds.parse_terminated(WherePredicate::parse, Token![,])?;
            try_set_attribute(
                &mut self.deserialize_bounds,
                clauses,
                "deserialize_bounds",
            )
        } else if meta.path.is_ident("archived") {
            try_set_attribute(
                &mut self.archived,
                meta.value()?.parse()?,
                "archived",
            )
        } else if meta.path.is_ident("resolver") {
            try_set_attribute(
                &mut self.resolver,
                meta.value()?.parse()?,
                "resolver",
            )
        } else if meta.path.is_ident("as") {
            try_set_attribute(
                &mut self.archive_as,
                meta.value()?.parse()?,
                "as",
            )
        } else if meta.path.is_ident("crate") {
            if meta.input.parse::<Token![=]>().is_ok() {
                let path = meta.input.parse::<Path>()?;
                try_set_attribute(&mut self.rkyv_path, path, "crate")
            } else if meta.input.is_empty() {
                try_set_attribute(
                    &mut self.rkyv_path,
                    parse_quote! { crate },
                    "crate",
                )
            } else {
                Err(meta.error("expected `crate` or `crate = ...`"))
            }
        } else {
            Err(meta.error("unrecognized archive argument"))
        }
    }

    pub fn parse(input: &DeriveInput) -> Result<Attributes, Error> {
        let mut result = Attributes::default();
        for attr in input.attrs.iter() {
            if !matches!(attr.style, AttrStyle::Outer) {
                continue;
            }

            if attr.path().is_ident("archive") {
                attr.parse_nested_meta(|meta| result.parse_meta(meta))?;
            } else if attr.path().is_ident("archive_attr") {
                result.attrs.extend(
                    attr.parse_args_with(
                        Punctuated::<Meta, Token![,]>::parse_terminated,
                    )?
                    .into_iter(),
                );
            }
        }

        Ok(result)
    }

    pub fn rkyv_path(&self) -> Path {
        self.rkyv_path
            .clone()
            .unwrap_or_else(|| parse_quote! { ::rkyv })
    }
}
