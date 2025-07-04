use super::{
    config::{Config, ReprKind},
    field_info::FieldInfo,
    BitfieldStruct,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::{self, punctuated::Punctuated, spanned::Spanned as _, Token};

impl BitfieldStruct {
    /// Expands the given `#[bitfield]` struct into an actual bitfield definition.
    pub fn expand(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let check_filled = self.generate_check_for_filled(config);
        let struct_definition = self.generate_struct(config);
        let constructor_definition = self.generate_constructor(config);
        let specifier_impl = self.generate_specifier_impl(config);

        let byte_conversion_impls = self.expand_byte_conversion_impls(config);
        let getters_and_setters = self.expand_getters_and_setters(config);
        let bytes_check = self.expand_optional_bytes_check(config);
        let repr_impls_and_checks = self.expand_repr_from_impls_and_checks(config);
        let debug_impl = self.generate_debug_impl(config);
        let default_impl = self.generate_default_impl(config);

        quote_spanned!(span=>
            #struct_definition
            #check_filled
            #constructor_definition
            #byte_conversion_impls
            #getters_and_setters
            #specifier_impl
            #bytes_check
            #repr_impls_and_checks
            #debug_impl
            #default_impl
        )
    }

    /// Expands to the `Specifier` impl for the `#[bitfield]` struct if the
    /// `#[derive(Specifier)]` attribute is applied to it as well.
    ///
    /// Otherwise returns `None`.
    pub fn generate_specifier_impl(&self, config: &Config) -> Option<TokenStream2> {
        config.derive_specifier.as_ref()?;

        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();
        let bits = self.generate_target_or_actual_bitfield_size(config);
        let next_divisible_by_8 = Self::next_divisible_by_8(&bits);

        let deprecation_warning = config.deprecated_specifier.map(|span| {
            quote_spanned!(span=> const _: () = {
                #[derive(::modular_bitfield::BitfieldSpecifier)]
                enum #ident { A = 0, B = 1 }
            };)
        });

        Some(quote_spanned!(span=>
            #deprecation_warning

            #[allow(clippy::identity_op)]
            const _: () = {
                impl #impl_generics ::modular_bitfield::private::checks::CheckSpecifierHasAtMost128Bits for #ident #ty_generics #where_clause {
                    type CheckType = [(); (#bits <= 128) as ::core::primitive::usize];
                }
            };

            #[allow(clippy::identity_op)]
            impl #impl_generics ::modular_bitfield::Specifier for #ident #ty_generics #where_clause {
                const BITS: usize = #bits;

                type Bytes = <[(); if #bits > 128 { 128 } else { #bits }] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
                type InOut = Self;

                #[inline]
                fn into_bytes(
                    value: Self::InOut,
                ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
                    ::core::result::Result::Ok(
                        <[(); #next_divisible_by_8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(
                            value.bytes
                        )
                    )
                }

                #[inline]
                fn from_bytes(
                    bytes: Self::Bytes,
                ) -> ::core::result::Result<Self::InOut, ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>>
                {
                    use ::core::convert::TryFrom;
                    let __bf_max_value: Self::Bytes = (0x01 as Self::Bytes)
                        .checked_shl(u32::try_from(Self::BITS).unwrap())
                        .unwrap_or(<Self::Bytes>::MAX);
                    if bytes <= __bf_max_value {
                        let __bf_bytes = bytes.to_le_bytes();
                        ::core::result::Result::Ok(Self {
                            bytes: <[(); #next_divisible_by_8] as ::modular_bitfield::private::ArrayBytesConversion>::bytes_into_array(bytes)
                        })
                    } else {
                        ::core::result::Result::Err(::modular_bitfield::error::InvalidBitPattern::new(bytes))
                    }
                }
            }
        ))
    }

    /// Generates the `core::fmt::Debug` impl if `#[derive(Debug)]` is included.
    pub fn generate_debug_impl(&self, config: &Config) -> Option<TokenStream2> {
        config.derive_debug.as_ref()?;
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();
        let is_tuple = matches!(self.item_struct.fields, syn::Fields::Unnamed(_));
        let builder_name = if is_tuple {
            quote_spanned!(span=> debug_tuple)
        } else {
            quote_spanned!(span=> debug_struct)
        };
        let fields = self.field_infos(config).map(|info| {
            let FieldInfo {
                index: _,
                field,
                config,
            } = &info;
            if config.skip_getters() {
                return None;
            }
            let field_span = field.span();
            let field_name = if field.ident.is_some() {
                let field_name = info.name();
                quote_spanned!(field_span=> #field_name,)
            } else {
                <_>::default()
            };
            let field_ident = info.ident_frag();
            let field_getter = field.ident.as_ref().map_or_else(
                || format_ident!("get_{}_or_err", field_ident),
                |_| format_ident!("{}_or_err", field_ident),
            );
            Some(quote_spanned!(field_span=>
                .field(
                    #field_name
                    self.#field_getter()
                        .as_ref()
                        .map_or_else(
                            |__bf_err| __bf_err as &dyn (::core::fmt::Debug),
                            |__bf_field| __bf_field as &dyn (::core::fmt::Debug)
                        )
                )
            ))
        });
        Some(quote_spanned!(span=>
            impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause {
                fn fmt(&self, __bf_f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    __bf_f.#builder_name(::core::stringify!(#ident))
                        #( #fields )*
                        .finish()
                }
            }
        ))
    }

    /// Generates the `Default` implementation for the `#[bitfield]` struct if the
    /// `#[derive(Default)]` attribute is applied to it.
    ///
    /// The implementation calls `Self::new()` which applies any field-level
    /// `#[default(...)]` attributes and zero-initializes fields without defaults.
    pub fn generate_default_impl(&self, config: &Config) -> Option<TokenStream2> {
        config.derive_default.as_ref()?;
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();

        Some(quote_spanned!(span=>
            impl #impl_generics ::core::default::Default for #ident #ty_generics #where_clause {
                fn default() -> Self {
                    Self::new()
                }
            }
        ))
    }

    /// Generates the expression denoting the sum of all field bit specifier sizes.
    ///
    /// # Example
    ///
    /// For the following struct:
    ///
    /// ```no_compile
    /// #[bitfield]
    /// pub struct Color {
    ///     r: B8,
    ///     g: B8,
    ///     b: B8,
    ///     a: bool,
    ///     rest: B7,
    /// }
    /// ```
    ///
    /// We generate the following tokens:
    ///
    /// ```no_compile
    /// 0usize +
    /// <B8 as ::modular_bitfield::Specifier>::BITS +
    /// <B8 as ::modular_bitfield::Specifier>::BITS +
    /// <B8 as ::modular_bitfield::Specifier>::BITS +
    /// <bool as ::modular_bitfield::Specifier>::BITS +
    /// <B7 as ::modular_bitfield::Specifier>::BITS
    /// ```
    ///
    /// Which is a compile time evaluatable expression.
    fn generate_bitfield_size(&self) -> TokenStream2 {
        let span = self.item_struct.span();
        self.item_struct
            .fields
            .iter()
            .map(|field| {
                let span = field.span();
                let ty = &field.ty;
                quote_spanned!(span=>
                    <#ty as ::modular_bitfield::Specifier>::BITS
                )
            })
            .fold(quote_spanned!(span=> 0usize), |lhs, rhs| {
                quote_spanned!(span =>
                    #lhs + #rhs
                )
            })
    }

    /// Generates the expression denoting the actual configured or implied bit width.
    fn generate_target_or_actual_bitfield_size(&self, config: &Config) -> TokenStream2 {
        config.bits.as_ref().map_or_else(
            || self.generate_bitfield_size(),
            |bits_config| {
                let span = bits_config.span;
                let value = bits_config.value;
                quote_spanned!(span=>
                    #value
                )
            },
        )
    }

    /// Generates a check in case `bits = N` is unset to verify that the actual amount of bits is either
    ///
    /// - ... equal to `N`, if `filled = true` or
    /// - ... smaller than `N`, if `filled = false`
    fn generate_filled_check_for_unaligned_bits(
        &self,
        config: &Config,
        required_bits: usize,
    ) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();
        let actual_bits = self.generate_bitfield_size();
        let check_ident = if config.filled_enabled() {
            quote_spanned!(span => CheckFillsUnalignedBits)
        } else {
            quote_spanned!(span => CheckDoesNotFillUnalignedBits)
        };
        let comparator = if config.filled_enabled() {
            quote! { == }
        } else {
            quote! { > }
        };
        quote_spanned!(span=>
            #[allow(clippy::identity_op)]
            const _: () = {
                impl #impl_generics ::modular_bitfield::private::checks::#check_ident for #ident #ty_generics #where_clause {
                    type CheckType = [(); (#required_bits #comparator #actual_bits) as usize];
                }
            };
        )
    }

    /// Generates a check in case `bits = N` is unset to verify that the actual amount of bits is either
    ///
    /// - ... divisible by 8, if `filled = true` or
    /// - ... not divisible by 8, if `filled = false`
    fn generate_filled_check_for_aligned_bits(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();
        let actual_bits = self.generate_bitfield_size();
        let check_ident = if config.filled_enabled() {
            quote_spanned!(span => CheckTotalSizeMultipleOf8)
        } else {
            quote_spanned!(span => CheckTotalSizeIsNotMultipleOf8)
        };
        quote_spanned!(span=>
            #[allow(clippy::identity_op)]
            const _: () = {
                impl #impl_generics ::modular_bitfield::private::checks::#check_ident for #ident #ty_generics #where_clause {
                    type Size = ::modular_bitfield::private::checks::TotalSize<[(); (#actual_bits) % 8usize]>;
                }
            };
        )
    }

    /// Generate check for either of the following two cases:
    ///
    /// - `filled = true`: Check if the total number of required bits is
    ///   - ... the same as `N` if `bits = N` was provided or
    ///   - ... a multiple of 8, otherwise
    /// - `filled = false`: Check if the total number of required bits is
    ///   - ... smaller than `N` if `bits = N` was provided or
    ///   - ... NOT a multiple of 8, otherwise
    fn generate_check_for_filled(&self, config: &Config) -> TokenStream2 {
        match config.bits.as_ref() {
            Some(bits_config) => {
                self.generate_filled_check_for_unaligned_bits(config, bits_config.value)
            }
            None => self.generate_filled_check_for_aligned_bits(config),
        }
    }

    /// Returns a token stream representing the next greater value divisible by 8.
    fn next_divisible_by_8(value: &TokenStream2) -> TokenStream2 {
        let span = value.span();
        quote_spanned!(span=>
            (((#value - 1) / 8) + 1) * 8
        )
    }

    /// Generates the actual item struct definition for the `#[bitfield]`.
    ///
    /// Internally it only contains a byte array equal to the minimum required
    /// amount of bytes to compactly store the information of all its bit fields.
    fn generate_struct(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let attrs = &config.retained_attributes;
        let vis = &self.item_struct.vis;
        let ident = &self.item_struct.ident;
        let generics = &self.item_struct.generics;
        let size = self.generate_target_or_actual_bitfield_size(config);
        let next_divisible_by_8 = Self::next_divisible_by_8(&size);
        quote_spanned!(span=>
            #( #attrs )*
            #[allow(clippy::identity_op)]
            #vis struct #ident #generics
            {
                bytes: [::core::primitive::u8; #next_divisible_by_8 / 8usize],
            }
        )
    }

    /// Generates the constructor for the bitfield that initializes all bytes to zero
    /// and applies any default values specified with #[default(...)].
    fn generate_constructor(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();
        let size = self.generate_target_or_actual_bitfield_size(config);
        let next_divisible_by_8 = Self::next_divisible_by_8(&size);

        // Check if any fields have default values
        let has_defaults = self
            .field_infos(config)
            .any(|info| info.config.default.is_some());

        if has_defaults {
            // Generate const new() with defaults using bit manipulation
            let byte_count = quote_spanned!(span=> #next_divisible_by_8 / 8usize);

            // Generate the const byte array initialization
            let const_bytes_init = self.generate_const_default_bytes(config, &byte_count);

            quote_spanned!(span=>
                impl #impl_generics #ident #ty_generics #where_clause
                {
                    /// Returns an instance with default values applied.
                    #[allow(clippy::identity_op)]
                    #[allow(clippy::new_without_default)]
                    #[must_use]
                    pub const fn new() -> Self {
                        Self {
                            bytes: #const_bytes_init,
                        }
                    }

                    /// Returns an instance with all bits initialized to zero.
                    #[allow(clippy::identity_op)]
                    #[must_use]
                    pub const fn new_zeroed() -> Self {
                        Self {
                            bytes: [0u8; #byte_count],
                        }
                    }
                }
            )
        } else {
            // Original const constructor when no defaults
            quote_spanned!(span=>
                impl #impl_generics #ident #ty_generics #where_clause
                {
                    /// Returns an instance with zero initialized data.
                    #[allow(clippy::identity_op)]
                    #[allow(clippy::new_without_default)]
                    #[must_use]
                    pub const fn new() -> Self {
                        Self {
                            bytes: [0u8; #next_divisible_by_8 / 8usize],
                        }
                    }
                }
            )
        }
    }

    /// Generates the compile-time assertion if the optional `byte` parameter has been set.
    fn expand_optional_bytes_check(&self, config: &Config) -> Option<TokenStream2> {
        let ident = &self.item_struct.ident;
        config.bytes.as_ref().map(|config| {
            let bytes = config.value;
            quote_spanned!(config.span=>
                const _: () = {
                    struct ExpectedBytes { __bf_unused: [::core::primitive::u8; #bytes] }

                    ::modular_bitfield::private::static_assertions::assert_eq_size!(
                        ExpectedBytes,
                        #ident
                    );
                };
            )
        })
    }

    /// Generates `From` impls for a `#[repr(uN)]` annotated #[bitfield] struct.
    fn expand_repr_from_impls_and_checks(&self, config: &Config) -> Option<TokenStream2> {
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();
        let where_predicates = where_clause.map(|w| &w.predicates);
        config.repr.as_ref().map(|repr| {
            let kind = &repr.value;
            let span = repr.span;
            let prim = match kind {
                ReprKind::U8 => quote! { ::core::primitive::u8 },
                ReprKind::U16 => quote! { ::core::primitive::u16 },
                ReprKind::U32 => quote! { ::core::primitive::u32 },
                ReprKind::U64 => quote! { ::core::primitive::u64 },
                ReprKind::U128 => quote! { ::core::primitive::u128 },
            };
            let actual_bits = self.generate_target_or_actual_bitfield_size(config);
            let trait_check_ident = match kind {
                ReprKind::U8 => quote! { IsU8Compatible },
                ReprKind::U16 => quote! { IsU16Compatible },
                ReprKind::U32 => quote! { IsU32Compatible },
                ReprKind::U64 => quote! { IsU64Compatible },
                ReprKind::U128 => quote! { IsU128Compatible },
            };
            quote_spanned!(span=>
                #[allow(clippy::identity_op)]
                impl #impl_generics ::core::convert::From<#prim> for #ident #ty_generics
                where
                    [(); #actual_bits]: ::modular_bitfield::private::#trait_check_ident,
                    #where_predicates
                {
                    #[inline]
                    fn from(__bf_prim: #prim) -> Self {
                        Self { bytes: <#prim>::to_le_bytes(__bf_prim) }
                    }
                }

                #[allow(clippy::identity_op)]
                impl #impl_generics ::core::convert::From<#ident #ty_generics> for #prim
                where
                    [(); #actual_bits]: ::modular_bitfield::private::#trait_check_ident,
                    #where_predicates
                {
                    #[inline]
                    fn from(__bf_bitfield: #ident #ty_generics) -> Self {
                        <Self>::from_le_bytes(__bf_bitfield.bytes)
                    }
                }
            )
        })
    }

    /// Generates routines to allow conversion from and to bytes for the `#[bitfield]` struct.
    fn expand_byte_conversion_impls(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();
        let size = self.generate_target_or_actual_bitfield_size(config);
        let next_divisible_by_8 = Self::next_divisible_by_8(&size);
        let from_bytes = if config.filled_enabled() {
            quote_spanned!(span=>
                /// Converts the given bytes directly into the bitfield struct.
                #[inline]
                #[allow(clippy::identity_op)]
                #[must_use]
                pub const fn from_bytes(bytes: [::core::primitive::u8; #next_divisible_by_8 / 8usize]) -> Self {
                    Self { bytes }
                }
            )
        } else {
            quote_spanned!(span=>
                /// Converts the given bytes directly into the bitfield struct.
                ///
                /// # Errors
                ///
                /// If the given bytes contain bits at positions that are undefined for `Self`.
                #[inline]
                #[allow(clippy::identity_op)]
                pub fn from_bytes(
                    bytes: [::core::primitive::u8; #next_divisible_by_8 / 8usize]
                ) -> ::core::result::Result<Self, ::modular_bitfield::error::OutOfBounds> {
                    if ::core::primitive::u16::from(bytes[(#next_divisible_by_8 / 8usize) - 1]) < (0x01 << (8 - (#next_divisible_by_8 - (#size)))) {
                        ::core::result::Result::Ok(Self { bytes })
                    } else {
                        ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds)
                    }
                }
            )
        };
        quote_spanned!(span=>
            impl #impl_generics #ident #ty_generics #where_clause {
                /// Returns the underlying bits.
                ///
                /// # Layout
                ///
                /// The returned byte array is layed out in the same way as described
                /// [here](https://docs.rs/modular-bitfield/#generated-structure).
                #[inline]
                #[allow(clippy::identity_op)]
                pub const fn into_bytes(self) -> [::core::primitive::u8; #next_divisible_by_8 / 8usize] {
                    self.bytes
                }

                #from_bytes
            }
        )
    }

    /// Generates code to check for the bit size arguments of bitfields.
    fn expand_bits_checks_for_field(field_info: FieldInfo<'_>) -> TokenStream2 {
        let FieldInfo {
            index: _,
            field,
            config,
        } = field_info;
        let span = field.span();
        let bits_check = match &config.bits {
            Some(bits) => {
                let ty = &field.ty;
                let expected_bits = bits.value;
                let span = bits.span;
                Some(quote_spanned!(span =>
                    let _: ::modular_bitfield::private::checks::BitsCheck::<[(); #expected_bits]> =
                        ::modular_bitfield::private::checks::BitsCheck::<[(); #expected_bits]>{
                            arr: [(); <#ty as ::modular_bitfield::Specifier>::BITS]
                        };
                ))
            }
            None => None,
        };
        quote_spanned!(span=>
            const _: () = {
                #bits_check
            };
        )
    }

    fn expand_getters_for_field(
        &self,
        offset: &Punctuated<syn::Expr, syn::Token![+]>,
        info: &FieldInfo<'_>,
    ) -> Option<TokenStream2> {
        let FieldInfo {
            index: _,
            field,
            config,
        } = info;
        if config.skip_getters() {
            return None;
        }
        let struct_ident = &self.item_struct.ident;
        let span = field.span();
        let ident = info.ident_frag();
        let name = info.name();

        let retained_attrs = &config.retained_attrs;
        let get_ident = field
            .ident
            .clone()
            .unwrap_or_else(|| format_ident!("get_{}", ident));
        let get_checked_ident = field.ident.as_ref().map_or_else(
            || format_ident!("get_{}_or_err", ident),
            |_| format_ident!("{}_or_err", ident),
        );
        let ty = &field.ty;
        let vis = &field.vis;
        let get_assert_msg =
            format!("value contains invalid bit pattern for field {struct_ident}.{name}");

        let getter_docs = format!("Returns the value of `{name}`.");
        let checked_getter_docs = format!(
            "Returns the value of `{name}`.\n\n\
             # Errors\n\n\
             If the returned value contains an invalid bit pattern for `{name}`.",
        );
        let getters = quote_spanned!(span=>
            #[doc = #getter_docs]
            #[inline]
            #[must_use]
            #( #retained_attrs )*
            #vis fn #get_ident(&self) -> <#ty as ::modular_bitfield::Specifier>::InOut {
                self.#get_checked_ident().expect(#get_assert_msg)
            }

            #[doc = #checked_getter_docs]
            #[inline]
            #[allow(dead_code)]
            #( #retained_attrs )*
            #vis fn #get_checked_ident(
                &self,
            ) -> ::core::result::Result<
                <#ty as ::modular_bitfield::Specifier>::InOut,
                ::modular_bitfield::error::InvalidBitPattern<<#ty as ::modular_bitfield::Specifier>::Bytes>
            > {
                let __bf_read: <#ty as ::modular_bitfield::Specifier>::Bytes = {
                    ::modular_bitfield::private::read_specifier::<#ty>(&self.bytes[..], #offset)
                };
                <#ty as ::modular_bitfield::Specifier>::from_bytes(__bf_read)
            }
        );
        Some(getters)
    }

    fn expand_setters_for_field(
        &self,
        offset: &Punctuated<syn::Expr, syn::Token![+]>,
        info: &FieldInfo<'_>,
    ) -> Option<TokenStream2> {
        let FieldInfo {
            index: _,
            field,
            config,
        } = info;
        if config.skip_setters() {
            return None;
        }
        let struct_ident = &self.item_struct.ident;
        let span = field.span();
        let retained_attrs = &config.retained_attrs;

        let ident = info.ident_frag();
        let name = info.name();
        let ty = &field.ty;
        let vis = &field.vis;

        let set_ident = format_ident!("set_{}", ident);
        let set_checked_ident = format_ident!("set_{}_checked", ident);
        let with_ident = format_ident!("with_{}", ident);
        let with_checked_ident = format_ident!("with_{}_checked", ident);

        let set_assert_msg = format!("value out of bounds for field {struct_ident}.{name}");
        let setter_docs = format!(
            "Sets the value of `{name}` to the given value.\n\n\
             # Panics\n\n\
             If the given value is out of bounds for `{name}`.",
        );
        let checked_setter_docs = format!(
            "Sets the value of `{name}` to the given value.\n\n\
             # Errors\n\n\
             If the given value is out of bounds for `{name}`.",
        );
        let with_docs = format!(
            "Returns a copy of the bitfield with the value of `{name}` \
             set to the given value.\n\n\
             # Panics\n\n\
             If the given value is out of bounds for `{name}`.",
        );
        let checked_with_docs = format!(
            "Returns a copy of the bitfield with the value of `{name}` \
             set to the given value.\n\n\
             # Errors\n\n\
             If the given value is out of bounds for `{name}`.",
        );
        let setters = quote_spanned!(span=>
            #[doc = #with_docs]
            #[inline]
            #[allow(dead_code)]
            #[must_use]
            #( #retained_attrs )*
            #vis fn #with_ident(
                mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut
            ) -> Self {
                self.#set_ident(new_val);
                self
            }

            #[doc = #checked_with_docs]
            #[inline]
            #[allow(dead_code)]
            #( #retained_attrs )*
            #vis fn #with_checked_ident(
                mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut,
            ) -> ::core::result::Result<Self, ::modular_bitfield::error::OutOfBounds> {
                self.#set_checked_ident(new_val)?;
                ::core::result::Result::Ok(self)
            }

            #[doc = #setter_docs]
            #[inline]
            #[allow(dead_code)]
            #( #retained_attrs )*
            #vis fn #set_ident(&mut self, new_val: <#ty as ::modular_bitfield::Specifier>::InOut) {
                self.#set_checked_ident(new_val).expect(#set_assert_msg)
            }

            #[doc = #checked_setter_docs]
            #[inline]
            #( #retained_attrs )*
            #vis fn #set_checked_ident(
                &mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut
            ) -> ::core::result::Result<(), ::modular_bitfield::error::OutOfBounds> {
                let __bf_base_bits: ::core::primitive::usize = 8usize * ::core::mem::size_of::<<#ty as ::modular_bitfield::Specifier>::Bytes>();
                let __bf_max_value: <#ty as ::modular_bitfield::Specifier>::Bytes = {
                    !0 >> (__bf_base_bits - <#ty as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <#ty as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <#ty as ::modular_bitfield::Specifier>::Bytes = {
                    <#ty as ::modular_bitfield::Specifier>::into_bytes(new_val)
                }?;
                // We compare base bits with spec bits to drop this condition
                // if there cannot be invalid inputs.
                if __bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value {
                    ::modular_bitfield::private::write_specifier::<#ty>(&mut self.bytes[..], #offset, __bf_raw_val);
                    ::core::result::Result::Ok(())
                } else {
                    ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds)
                }
            }
        );
        Some(setters)
    }

    fn expand_getters_and_setters_for_field(
        &self,
        offset: &mut Punctuated<syn::Expr, syn::Token![+]>,
        info: &FieldInfo<'_>,
    ) -> TokenStream2 {
        let field = info.field;
        let span = field.span();
        let ty = &field.ty;
        let getters = self.expand_getters_for_field(offset, info);
        let setters = self.expand_setters_for_field(offset, info);
        let getters_and_setters = quote_spanned!(span=>
            #getters
            #setters
        );
        offset.push(syn::parse_quote! { <#ty as ::modular_bitfield::Specifier>::BITS });
        getters_and_setters
    }

    fn expand_getters_and_setters(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();
        let mut offset = {
            let mut offset = Punctuated::<syn::Expr, Token![+]>::new();
            offset.push(syn::parse_quote! { 0usize });
            offset
        };
        let bits_checks = self
            .field_infos(config)
            .map(|field_info| Self::expand_bits_checks_for_field(field_info));
        let setters_and_getters = self
            .field_infos(config)
            .map(|field_info| self.expand_getters_and_setters_for_field(&mut offset, &field_info));
        quote_spanned!(span=>
            const _: () = {
                #( #bits_checks )*
            };

            impl #impl_generics #ident #ty_generics #where_clause {
                #( #setters_and_getters )*
            }
        )
    }

    /// Generates a const expression that creates a byte array with default values applied.
    /// This uses const-compatible bit manipulation to set default values at compile time.
    fn generate_const_default_bytes(
        &self,
        config: &Config,
        byte_count: &TokenStream2,
    ) -> TokenStream2 {
        let span = self.item_struct.span();

        let mut bit_manipulations = Vec::new();
        let mut current_offset = quote! { 0usize };

        for info in self.field_infos(config) {
            let field_config = &info.config;
            let field_type = &info.field.ty;
            let field_bits = quote! { <#field_type as ::modular_bitfield::Specifier>::BITS };

            // Add default value assignment if present
            if let Some(default_config) = &field_config.default {
                if !field_config.skip_setters() {
                    let default_value = &default_config.value;

                    // Generate const-compatible value conversion
                    let const_value = Self::generate_const_value_conversion(
                        field_type,
                        default_value,
                        default_config.span,
                    );

                    // Generate bit manipulation at current offset
                    let bit_manipulation = quote_spanned!(default_config.span=> {
                        // Set field value at bit offset
                        let field_offset = #current_offset;
                        let field_value = #const_value;
                        let field_bits = #field_bits;

                        // Simple bit manipulation that works in const context
                        let start_byte = field_offset / 8;
                        let start_bit = field_offset % 8;

                        // Handle fields that fit within a single byte
                        if start_bit + field_bits <= 8 {
                            if field_bits == 8 && start_bit == 0 {
                                // Full byte assignment
                                bytes[start_byte] = field_value as u8;
                            } else if field_bits < 8 {
                                // Partial byte assignment
                                let mask = ((1u8 << field_bits) - 1) << start_bit;
                                let shifted_value = (field_value as u8) << start_bit;
                                bytes[start_byte] = (bytes[start_byte] & !mask) | shifted_value;
                            }
                        } else if field_bits == 16 && start_bit == 0 {
                            // 16-bit aligned field
                            let value = field_value as u16;
                            bytes[start_byte] = value as u8;
                            bytes[start_byte + 1] = (value >> 8) as u8;
                        } else if field_bits == 32 && start_bit == 0 {
                            // 32-bit aligned field
                            let value = field_value as u32;
                            bytes[start_byte] = value as u8;
                            bytes[start_byte + 1] = (value >> 8) as u8;
                            bytes[start_byte + 2] = (value >> 16) as u8;
                            bytes[start_byte + 3] = (value >> 24) as u8;
                        } else {
                            // Multi-byte field with bit manipulation
                            let mut remaining_bits = field_bits;
                            let mut value = field_value as u64;
                            let mut byte_idx = start_byte;
                            let mut bit_pos = start_bit;

                            while remaining_bits > 0 {
                                let bits_in_this_byte = if bit_pos + remaining_bits <= 8 {
                                    remaining_bits
                                } else {
                                    8 - bit_pos
                                };

                                // Avoid shift overflow by checking bit count
                                if bits_in_this_byte > 0 && bits_in_this_byte < 8 {
                                    let mask = ((1u8 << bits_in_this_byte) - 1) << bit_pos;
                                    let byte_value = ((value & ((1u64 << bits_in_this_byte) - 1)) as u8) << bit_pos;
                                    bytes[byte_idx] = (bytes[byte_idx] & !mask) | byte_value;
                                } else if bits_in_this_byte == 8 && bit_pos == 0 {
                                    // Full byte write
                                    bytes[byte_idx] = value as u8;
                                }

                                value >>= bits_in_this_byte;
                                remaining_bits -= bits_in_this_byte;
                                byte_idx += 1;
                                bit_pos = 0;
                            }
                        }
                    });

                    bit_manipulations.push(bit_manipulation);
                }
            }

            // Advance offset for next field
            current_offset = quote! { #current_offset + #field_bits };
        }

        if bit_manipulations.is_empty() {
            // No defaults, just return zero array
            quote_spanned!(span=> [0u8; #byte_count])
        } else {
            // Generate const block that builds the byte array
            quote_spanned!(span=> {
                let mut bytes = [0u8; #byte_count];
                #( #bit_manipulations )*
                bytes
            })
        }
    }

    /// Generates const-compatible value conversion for different field types.
    fn generate_const_value_conversion(
        field_type: &syn::Type,
        default_value: &syn::Expr,
        span: proc_macro2::Span,
    ) -> TokenStream2 {
        // Generate code that handles the most common cases with const-compatible casting
        quote_spanned!(span=> {
            // Cast the default value to the appropriate byte type
            // This will work for:
            // - bool: true as u8, false as u8
            // - enum variants: Status::Green as u8
            // - integer literals: 42 as u8
            #default_value as <#field_type as ::modular_bitfield::Specifier>::Bytes
        })
    }
}
