use super::{
    config::{Config, ReprKind},
    field_config::{FieldConfig, SkipWhich},
    raise_skip_error, BitfieldStruct,
};
use core::convert::TryFrom;
use quote::quote;
use syn::{self, parse::Result, spanned::Spanned as _};

impl TryFrom<(&mut Config, syn::ItemStruct)> for BitfieldStruct {
    type Error = syn::Error;

    fn try_from((config, item_struct): (&mut Config, syn::ItemStruct)) -> Result<Self> {
        Self::ensure_has_fields(&item_struct)?;
        Self::ensure_valid_generics(&item_struct)?;
        Self::extract_attributes(&item_struct.attrs, config)?;
        Self::analyse_config_for_fields(&item_struct, config)?;
        config.ensure_no_conflicts()?;
        Ok(Self { item_struct })
    }
}

impl BitfieldStruct {
    /// Returns an error if the input struct does not have any fields.
    fn ensure_has_fields(item_struct: &syn::ItemStruct) -> Result<()> {
        if matches!(&item_struct.fields, syn::Fields::Unit)
            || matches!(&item_struct.fields, syn::Fields::Unnamed(f) if f.unnamed.is_empty())
            || matches!(&item_struct.fields, syn::Fields::Named(f) if f.named.is_empty())
        {
            return Err(format_err_spanned!(
                item_struct,
                "encountered invalid bitfield struct without fields"
            ));
        }
        Ok(())
    }

    /// Returns an error if the input struct contains generics that cannot be
    /// used in a const expression.
    fn ensure_valid_generics(item_struct: &syn::ItemStruct) -> Result<()> {
        if item_struct.generics.type_params().next().is_some()
            || item_struct.generics.lifetimes().next().is_some()
        {
            return Err(format_err_spanned!(
                item_struct.generics,
                "bitfield structs can only use const generics"
            ));
        }
        Ok(())
    }

    /// Extracts the `#[repr(uN)]` annotations from the given `#[bitfield]` struct.
    fn extract_repr_attribute(attr: &syn::Attribute, config: &mut Config) -> Result<()> {
        let list = attr.meta.require_list()?;
        let mut retained_reprs = vec![];
        attr.parse_nested_meta(|meta| {
            let path = &meta.path;
            let repr_kind = if path.is_ident("u8") {
                Some(ReprKind::U8)
            } else if path.is_ident("u16") {
                Some(ReprKind::U16)
            } else if path.is_ident("u32") {
                Some(ReprKind::U32)
            } else if path.is_ident("u64") {
                Some(ReprKind::U64)
            } else if path.is_ident("u128") {
                Some(ReprKind::U128)
            } else {
                // If other repr such as `transparent` or `C` have been found we
                // are going to re-expand them into a new `#[repr(..)]` that is
                // ignored by the rest of this macro.
                retained_reprs.push(path.clone());
                None
            };
            if let Some(repr_kind) = repr_kind {
                config.repr(repr_kind, path.span())?;
            }
            Ok(())
        })?;
        if !retained_reprs.is_empty() {
            // We only push back another re-generated `#[repr(..)]` if its contents
            // contained some non-bitfield representations and thus is not empty.
            let retained_reprs_tokens = quote! {
                #( #retained_reprs ),*
            };
            config.push_retained_attribute(syn::Attribute {
                pound_token: attr.pound_token,
                style: attr.style,
                bracket_token: attr.bracket_token,
                meta: syn::Meta::List(syn::MetaList {
                    path: list.path.clone(),
                    delimiter: list.delimiter.clone(),
                    tokens: retained_reprs_tokens,
                }),
            });
        }
        Ok(())
    }

    /// Extracts the `#[derive(Debug)]` annotations from the given `#[bitfield]` struct.
    fn extract_derive_debug_attribute(attr: &syn::Attribute, config: &mut Config) -> Result<()> {
        let list = attr.meta.require_list()?;
        let mut retained_derives = vec![];
        attr.parse_nested_meta(|meta| {
            let path = &meta.path;
            if path.is_ident("Debug") {
                config.derive_debug(path.span())?;
            } else if path.is_ident("Default") {
                config.derive_default(path.span())?;
            } else if path.is_ident("BitfieldSpecifier") {
                config.deprecated_specifier(path.span());
                config.derive_specifier(path.span())?;
            } else if path.is_ident("Specifier") {
                config.derive_specifier(path.span())?;
            } else {
                // Other derives are going to be re-expanded them into a new
                // `#[derive(..)]` that is ignored by the rest of this macro.
                retained_derives.push(path.clone());
            }
            Ok(())
        })?;
        if !retained_derives.is_empty() {
            // We only push back another re-generated `#[derive(..)]` if its contents
            // contain some remaining derives and thus is not empty.
            let retained_derives_tokens = quote! {
                #( #retained_derives ),*
            };
            config.push_retained_attribute(syn::Attribute {
                pound_token: attr.pound_token,
                style: attr.style,
                bracket_token: attr.bracket_token,
                meta: syn::Meta::List(syn::MetaList {
                    path: list.path.clone(),
                    delimiter: list.delimiter.clone(),
                    tokens: retained_derives_tokens,
                }),
            });
        }
        Ok(())
    }

    /// Analyses and extracts the `#[repr(uN)]` or other annotations from the given struct.
    fn extract_attributes(attributes: &[syn::Attribute], config: &mut Config) -> Result<()> {
        for attr in attributes {
            if attr.path().is_ident("repr") {
                Self::extract_repr_attribute(attr, config)?;
            } else if attr.path().is_ident("derive") {
                Self::extract_derive_debug_attribute(attr, config)?;
            } else {
                config.push_retained_attribute(attr.clone());
            }
        }
        Ok(())
    }

    /// Analyses and extracts the configuration for all bitfield fields.
    fn analyse_config_for_fields(item_struct: &syn::ItemStruct, config: &mut Config) -> Result<()> {
        for (index, field) in Self::fields(item_struct) {
            let span = field.span();
            let field_config = Self::extract_field_config(field)?;
            config.field_config(index, span, field_config)?;
        }
        Ok(())
    }

    /// Extracts the `#[bits = N]` and `#[skip(..)]` attributes for a given field.
    fn extract_field_config(field: &syn::Field) -> Result<FieldConfig> {
        let mut config = FieldConfig::default();
        for attr in &field.attrs {
            if attr.path().is_ident("bits") {
                let name_value = attr.meta.require_name_value()?;
                let span = name_value.span();
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(lit_int),
                        ..
                    }) => {
                        config.bits(lit_int.base10_parse::<usize>()?, span)?;
                    }
                    value => {
                        return Err(format_err!(
                            value.span(),
                            "encountered invalid value type for #[bits = N]"
                        ))
                    }
                }
            } else if attr.path().is_ident("skip") {
                match &attr.meta {
                    syn::Meta::Path(path) => {
                        assert!(path.is_ident("skip"));
                        config.skip(SkipWhich::All, path.span())?;
                    }
                    syn::Meta::List(meta_list) => {
                        let (mut getters, mut setters) = (None, None);
                        meta_list.parse_nested_meta(|meta| {
                            let path = &meta.path;
                            if path.is_ident("getters") {
                                if let Some(previous) = getters {
                                    return raise_skip_error("(getters)", path.span(), previous);
                                }
                                getters = Some(path.span());
                            } else if path.is_ident("setters") {
                                if let Some(previous) = setters {
                                    return raise_skip_error("(setters)", path.span(), previous);
                                }
                                setters = Some(path.span());
                            } else {
                                return Err(meta.error(
                                    "encountered unknown or unsupported #[skip(..)] specifier",
                                ));
                            }
                            Ok(())
                        })?;
                        if getters.is_some() == setters.is_some() {
                            config.skip(SkipWhich::All, meta_list.path.span())?;
                        } else if getters.is_some() {
                            config.skip(SkipWhich::Getters, meta_list.path.span())?;
                        } else {
                            config.skip(SkipWhich::Setters, meta_list.path.span())?;
                        }
                    }
                    meta @ syn::Meta::NameValue(..) => {
                        return Err(format_err!(
                            meta.span(),
                            "encountered invalid format for #[skip] field attribute"
                        ))
                    }
                }
            } else if attr.path().is_ident("default") {
                match &attr.meta {
                    syn::Meta::List(meta_list) => {
                        // Parse the default value expression inside the parentheses
                        let default_value: syn::Expr = meta_list.parse_args()?;
                        config.set_default(default_value, meta_list.span())?;
                    }
                    meta => {
                        return Err(format_err!(
                            meta.span(),
                            "encountered invalid format for #[default(...)] field attribute, expected #[default(expression)]"
                        ))
                    }
                }
            } else {
                config.retain_attr(attr.clone());
            }
        }
        Ok(config)
    }
}
