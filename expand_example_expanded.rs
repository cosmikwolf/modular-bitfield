#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use modular_bitfield::prelude::*;
pub enum TwoBit {
    A,
    B,
    C,
    D,
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::A as usize],
> for TwoBit {
    type CheckType = [(); ((Self::A as usize) < (0x01_usize << 2usize)) as usize];
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::B as usize],
> for TwoBit {
    type CheckType = [(); ((Self::B as usize) < (0x01_usize << 2usize)) as usize];
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::C as usize],
> for TwoBit {
    type CheckType = [(); ((Self::C as usize) < (0x01_usize << 2usize)) as usize];
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::D as usize],
> for TwoBit {
    type CheckType = [(); ((Self::D as usize) < (0x01_usize << 2usize)) as usize];
}
impl ::modular_bitfield::Specifier for TwoBit {
    const BITS: usize = 2usize;
    type Bytes = <[(); 2usize] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
    type InOut = Self;
    #[inline]
    fn into_bytes(
        input: Self::InOut,
    ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
        ::core::result::Result::Ok(input as Self::Bytes)
    }
    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> ::core::result::Result<
        Self::InOut,
        ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>,
    > {
        match bytes {
            __bitfield_binding if __bitfield_binding
                == Self::A as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::A)
            }
            __bitfield_binding if __bitfield_binding
                == Self::B as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::B)
            }
            __bitfield_binding if __bitfield_binding
                == Self::C as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::C)
            }
            __bitfield_binding if __bitfield_binding
                == Self::D as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::D)
            }
            invalid_bytes => {
                ::core::result::Result::Err(
                    <::modular_bitfield::error::InvalidBitPattern<
                        Self::Bytes,
                    >>::new(invalid_bytes),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TwoBit {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TwoBit::A => "A",
                TwoBit::B => "B",
                TwoBit::C => "C",
                TwoBit::D => "D",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TwoBit {
    #[inline]
    fn clone(&self) -> TwoBit {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for TwoBit {}
#[bits = 4]
pub enum FourBit {
    Small = 0,
    Large = 15,
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::Small as usize],
> for FourBit {
    type CheckType = [(); ((Self::Small as usize) < (0x01_usize << 4usize)) as usize];
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::Large as usize],
> for FourBit {
    type CheckType = [(); ((Self::Large as usize) < (0x01_usize << 4usize)) as usize];
}
impl ::modular_bitfield::Specifier for FourBit {
    const BITS: usize = 4usize;
    type Bytes = <[(); 4usize] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
    type InOut = Self;
    #[inline]
    fn into_bytes(
        input: Self::InOut,
    ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
        ::core::result::Result::Ok(input as Self::Bytes)
    }
    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> ::core::result::Result<
        Self::InOut,
        ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>,
    > {
        match bytes {
            __bitfield_binding if __bitfield_binding
                == Self::Small as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::Small)
            }
            __bitfield_binding if __bitfield_binding
                == Self::Large as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::Large)
            }
            invalid_bytes => {
                ::core::result::Result::Err(
                    <::modular_bitfield::error::InvalidBitPattern<
                        Self::Bytes,
                    >>::new(invalid_bytes),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FourBit {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                FourBit::Small => "Small",
                FourBit::Large => "Large",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FourBit {
    #[inline]
    fn clone(&self) -> FourBit {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for FourBit {}
pub enum MinimalAdt {
    Small(TwoBit),
    Large(FourBit),
    Unit,
    Extra,
}
enum __Bf_MinimalAdt_Tag {
    Small,
    Large,
    Unit,
    Extra,
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::Small as usize],
> for __Bf_MinimalAdt_Tag {
    type CheckType = [(); ((Self::Small as usize) < (0x01_usize << 2usize)) as usize];
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::Large as usize],
> for __Bf_MinimalAdt_Tag {
    type CheckType = [(); ((Self::Large as usize) < (0x01_usize << 2usize)) as usize];
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::Unit as usize],
> for __Bf_MinimalAdt_Tag {
    type CheckType = [(); ((Self::Unit as usize) < (0x01_usize << 2usize)) as usize];
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::Extra as usize],
> for __Bf_MinimalAdt_Tag {
    type CheckType = [(); ((Self::Extra as usize) < (0x01_usize << 2usize)) as usize];
}
impl ::modular_bitfield::Specifier for __Bf_MinimalAdt_Tag {
    const BITS: usize = 2usize;
    type Bytes = <[(); 2usize] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
    type InOut = Self;
    #[inline]
    fn into_bytes(
        input: Self::InOut,
    ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
        ::core::result::Result::Ok(input as Self::Bytes)
    }
    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> ::core::result::Result<
        Self::InOut,
        ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>,
    > {
        match bytes {
            __bitfield_binding if __bitfield_binding
                == Self::Small as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::Small)
            }
            __bitfield_binding if __bitfield_binding
                == Self::Large as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::Large)
            }
            __bitfield_binding if __bitfield_binding
                == Self::Unit as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::Unit)
            }
            __bitfield_binding if __bitfield_binding
                == Self::Extra as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::Extra)
            }
            invalid_bytes => {
                ::core::result::Result::Err(
                    <::modular_bitfield::error::InvalidBitPattern<
                        Self::Bytes,
                    >>::new(invalid_bytes),
                )
            }
        }
    }
}
#[allow(clippy::identity_op)]
const _: () = {
    impl ::modular_bitfield::private::checks::CheckSpecifierHasAtMost128Bits
    for MinimalAdt {
        type CheckType = [(); (::modular_bitfield::private::const_max(
            <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                + <TwoBit as ::modular_bitfield::Specifier>::BITS,
            <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                + <FourBit as ::modular_bitfield::Specifier>::BITS,
        ) <= 128) as ::core::primitive::usize];
    }
};
impl ::modular_bitfield::Specifier for MinimalAdt {
    const BITS: usize = ::modular_bitfield::private::const_max(
        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
            + <FourBit as ::modular_bitfield::Specifier>::BITS,
    );
    type Bytes = <[(); ::modular_bitfield::private::const_max(
        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
            + <FourBit as ::modular_bitfield::Specifier>::BITS,
    )] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
    type InOut = Self;
    #[inline]
    fn into_bytes(
        input: Self::InOut,
    ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
        match input {
            MinimalAdt::Small(__bf_temp_0) => {
                let mut __bf_bytes = [0u8; (((::modular_bitfield::private::const_max(
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                        + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                        + <FourBit as ::modular_bitfield::Specifier>::BITS,
                ) - 1) / 8) + 1) * 8 / 8usize];
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::into_bytes(
                        __Bf_MinimalAdt_Tag::Small,
                    )
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    __Bf_MinimalAdt_Tag,
                >(&mut __bf_bytes, 0usize, __bf_raw_val);
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <TwoBit as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <TwoBit as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <TwoBit as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <TwoBit as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <TwoBit as ::modular_bitfield::Specifier>::Bytes = {
                    <TwoBit as ::modular_bitfield::Specifier>::into_bytes(__bf_temp_0)
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    TwoBit,
                >(
                    &mut __bf_bytes,
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS,
                    __bf_raw_val,
                );
                ::core::result::Result::Ok(
                    <[(); (((::modular_bitfield::private::const_max(
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                            + <FourBit as ::modular_bitfield::Specifier>::BITS,
                    ) - 1) / 8) + 1)
                        * 8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(
                        __bf_bytes,
                    ),
                )
            }
            MinimalAdt::Large(__bf_temp_0) => {
                let mut __bf_bytes = [0u8; (((::modular_bitfield::private::const_max(
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                        + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                        + <FourBit as ::modular_bitfield::Specifier>::BITS,
                ) - 1) / 8) + 1) * 8 / 8usize];
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::into_bytes(
                        __Bf_MinimalAdt_Tag::Large,
                    )
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    __Bf_MinimalAdt_Tag,
                >(&mut __bf_bytes, 0usize, __bf_raw_val);
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <FourBit as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <FourBit as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <FourBit as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <FourBit as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <FourBit as ::modular_bitfield::Specifier>::Bytes = {
                    <FourBit as ::modular_bitfield::Specifier>::into_bytes(__bf_temp_0)
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    FourBit,
                >(
                    &mut __bf_bytes,
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS,
                    __bf_raw_val,
                );
                ::core::result::Result::Ok(
                    <[(); (((::modular_bitfield::private::const_max(
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                            + <FourBit as ::modular_bitfield::Specifier>::BITS,
                    ) - 1) / 8) + 1)
                        * 8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(
                        __bf_bytes,
                    ),
                )
            }
            MinimalAdt::Unit => {
                let mut __bf_bytes = [0u8; (((::modular_bitfield::private::const_max(
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                        + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                        + <FourBit as ::modular_bitfield::Specifier>::BITS,
                ) - 1) / 8) + 1) * 8 / 8usize];
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::into_bytes(
                        __Bf_MinimalAdt_Tag::Unit,
                    )
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    __Bf_MinimalAdt_Tag,
                >(&mut __bf_bytes, 0usize, __bf_raw_val);
                ::core::result::Result::Ok(
                    <[(); (((::modular_bitfield::private::const_max(
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                            + <FourBit as ::modular_bitfield::Specifier>::BITS,
                    ) - 1) / 8) + 1)
                        * 8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(
                        __bf_bytes,
                    ),
                )
            }
            MinimalAdt::Extra => {
                let mut __bf_bytes = [0u8; (((::modular_bitfield::private::const_max(
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                        + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                        + <FourBit as ::modular_bitfield::Specifier>::BITS,
                ) - 1) / 8) + 1) * 8 / 8usize];
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::into_bytes(
                        __Bf_MinimalAdt_Tag::Extra,
                    )
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    __Bf_MinimalAdt_Tag,
                >(&mut __bf_bytes, 0usize, __bf_raw_val);
                ::core::result::Result::Ok(
                    <[(); (((::modular_bitfield::private::const_max(
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                            + <FourBit as ::modular_bitfield::Specifier>::BITS,
                    ) - 1) / 8) + 1)
                        * 8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(
                        __bf_bytes,
                    ),
                )
            }
        }
    }
    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> ::core::result::Result<
        Self::InOut,
        ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>,
    > {
        let array = <[(); (((::modular_bitfield::private::const_max(
            <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                + <TwoBit as ::modular_bitfield::Specifier>::BITS,
            <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
                + <FourBit as ::modular_bitfield::Specifier>::BITS,
        ) - 1) / 8) + 1)
            * 8] as ::modular_bitfield::private::ArrayBytesConversion>::bytes_into_array(
            bytes,
        );
        let __tag_read: <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::Bytes = {
            ::modular_bitfield::private::read_specifier::<
                __Bf_MinimalAdt_Tag,
            >(&array[..], 0)
        };
        let tag = <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::from_bytes(
                __tag_read,
            )
            .map_err(|_| ::modular_bitfield::error::InvalidBitPattern::new(bytes))?;
        match tag {
            __Bf_MinimalAdt_Tag::Small => {
                let __bf_read: <TwoBit as ::modular_bitfield::Specifier>::Bytes = {
                    ::modular_bitfield::private::read_specifier::<
                        TwoBit,
                    >(
                        &array[..],
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS,
                    )
                };
                let __bf_temp_0 = <TwoBit as ::modular_bitfield::Specifier>::from_bytes(
                        __bf_read,
                    )
                    .map_err(|_| ::modular_bitfield::error::InvalidBitPattern::new(
                        bytes,
                    ))?;
                Ok(MinimalAdt::Small(__bf_temp_0))
            }
            __Bf_MinimalAdt_Tag::Large => {
                let __bf_read: <FourBit as ::modular_bitfield::Specifier>::Bytes = {
                    ::modular_bitfield::private::read_specifier::<
                        FourBit,
                    >(
                        &array[..],
                        <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS,
                    )
                };
                let __bf_temp_0 = <FourBit as ::modular_bitfield::Specifier>::from_bytes(
                        __bf_read,
                    )
                    .map_err(|_| ::modular_bitfield::error::InvalidBitPattern::new(
                        bytes,
                    ))?;
                Ok(MinimalAdt::Large(__bf_temp_0))
            }
            __Bf_MinimalAdt_Tag::Unit => Ok(MinimalAdt::Unit),
            __Bf_MinimalAdt_Tag::Extra => Ok(MinimalAdt::Extra),
            #[allow(unreachable_patterns)]
            _ => {
                ::core::result::Result::Err(
                    ::modular_bitfield::error::InvalidBitPattern::new(bytes),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MinimalAdt {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MinimalAdt::Small(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Small", &__self_0)
            }
            MinimalAdt::Large(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Large", &__self_0)
            }
            MinimalAdt::Unit => ::core::fmt::Formatter::write_str(f, "Unit"),
            MinimalAdt::Extra => ::core::fmt::Formatter::write_str(f, "Extra"),
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MinimalAdt {
    #[inline]
    fn clone(&self) -> MinimalAdt {
        let _: ::core::clone::AssertParamIsClone<TwoBit>;
        let _: ::core::clone::AssertParamIsClone<FourBit>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for MinimalAdt {}
pub enum NestedMinimal {
    Inner(MinimalAdt),
    Just(TwoBit),
}
enum __Bf_NestedMinimal_Tag {
    Inner,
    Just,
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::Inner as usize],
> for __Bf_NestedMinimal_Tag {
    type CheckType = [(); ((Self::Inner as usize) < (0x01_usize << 1usize)) as usize];
}
impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<
    [(); Self::Just as usize],
> for __Bf_NestedMinimal_Tag {
    type CheckType = [(); ((Self::Just as usize) < (0x01_usize << 1usize)) as usize];
}
impl ::modular_bitfield::Specifier for __Bf_NestedMinimal_Tag {
    const BITS: usize = 1usize;
    type Bytes = <[(); 1usize] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
    type InOut = Self;
    #[inline]
    fn into_bytes(
        input: Self::InOut,
    ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
        ::core::result::Result::Ok(input as Self::Bytes)
    }
    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> ::core::result::Result<
        Self::InOut,
        ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>,
    > {
        match bytes {
            __bitfield_binding if __bitfield_binding
                == Self::Inner as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::Inner)
            }
            __bitfield_binding if __bitfield_binding
                == Self::Just as <Self as ::modular_bitfield::Specifier>::Bytes => {
                ::core::result::Result::Ok(Self::Just)
            }
            invalid_bytes => {
                ::core::result::Result::Err(
                    <::modular_bitfield::error::InvalidBitPattern<
                        Self::Bytes,
                    >>::new(invalid_bytes),
                )
            }
        }
    }
}
#[allow(clippy::identity_op)]
const _: () = {
    impl ::modular_bitfield::private::checks::CheckSpecifierHasAtMost128Bits
    for NestedMinimal {
        type CheckType = [(); (::modular_bitfield::private::const_max(
            <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,
            <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                + <TwoBit as ::modular_bitfield::Specifier>::BITS,
        ) <= 128) as ::core::primitive::usize];
    }
};
impl ::modular_bitfield::Specifier for NestedMinimal {
    const BITS: usize = ::modular_bitfield::private::const_max(
        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
            + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,
        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
    );
    type Bytes = <[(); ::modular_bitfield::private::const_max(
        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
            + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,
        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
    )] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
    type InOut = Self;
    #[inline]
    fn into_bytes(
        input: Self::InOut,
    ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
        match input {
            NestedMinimal::Inner(__bf_temp_0) => {
                let mut __bf_bytes = [0u8; (((::modular_bitfield::private::const_max(
                    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                        + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,
                    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                        + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                ) - 1) / 8) + 1) * 8 / 8usize];
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::into_bytes(
                        __Bf_NestedMinimal_Tag::Inner,
                    )
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    __Bf_NestedMinimal_Tag,
                >(&mut __bf_bytes, 0usize, __bf_raw_val);
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <MinimalAdt as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <MinimalAdt as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <MinimalAdt as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <MinimalAdt as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <MinimalAdt as ::modular_bitfield::Specifier>::Bytes = {
                    <MinimalAdt as ::modular_bitfield::Specifier>::into_bytes(
                        __bf_temp_0,
                    )
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    MinimalAdt,
                >(
                    &mut __bf_bytes,
                    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS,
                    __bf_raw_val,
                );
                ::core::result::Result::Ok(
                    <[(); (((::modular_bitfield::private::const_max(
                        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                            + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,
                        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                    ) - 1) / 8) + 1)
                        * 8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(
                        __bf_bytes,
                    ),
                )
            }
            NestedMinimal::Just(__bf_temp_0) => {
                let mut __bf_bytes = [0u8; (((::modular_bitfield::private::const_max(
                    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                        + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,
                    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                        + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                ) - 1) / 8) + 1) * 8 / 8usize];
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::Bytes = {
                    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::into_bytes(
                        __Bf_NestedMinimal_Tag::Just,
                    )
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    __Bf_NestedMinimal_Tag,
                >(&mut __bf_bytes, 0usize, __bf_raw_val);
                let __bf_base_bits: ::core::primitive::usize = 8usize
                    * ::core::mem::size_of::<
                        <TwoBit as ::modular_bitfield::Specifier>::Bytes,
                    >();
                let __bf_max_value: <TwoBit as ::modular_bitfield::Specifier>::Bytes = {
                    !0
                        >> (__bf_base_bits
                            - <TwoBit as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <TwoBit as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <TwoBit as ::modular_bitfield::Specifier>::Bytes = {
                    <TwoBit as ::modular_bitfield::Specifier>::into_bytes(__bf_temp_0)
                }?;
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value)
                {
                    return ::core::result::Result::Err(
                        ::modular_bitfield::error::OutOfBounds,
                    );
                }
                ::modular_bitfield::private::write_specifier::<
                    TwoBit,
                >(
                    &mut __bf_bytes,
                    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS,
                    __bf_raw_val,
                );
                ::core::result::Result::Ok(
                    <[(); (((::modular_bitfield::private::const_max(
                        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                            + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,
                        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                            + <TwoBit as ::modular_bitfield::Specifier>::BITS,
                    ) - 1) / 8) + 1)
                        * 8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(
                        __bf_bytes,
                    ),
                )
            }
        }
    }
    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> ::core::result::Result<
        Self::InOut,
        ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>,
    > {
        let array = <[(); (((::modular_bitfield::private::const_max(
            <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,
            <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
                + <TwoBit as ::modular_bitfield::Specifier>::BITS,
        ) - 1) / 8) + 1)
            * 8] as ::modular_bitfield::private::ArrayBytesConversion>::bytes_into_array(
            bytes,
        );
        let __tag_read: <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::Bytes = {
            ::modular_bitfield::private::read_specifier::<
                __Bf_NestedMinimal_Tag,
            >(&array[..], 0)
        };
        let tag = <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::from_bytes(
                __tag_read,
            )
            .map_err(|_| ::modular_bitfield::error::InvalidBitPattern::new(bytes))?;
        match tag {
            __Bf_NestedMinimal_Tag::Inner => {
                let __bf_read: <MinimalAdt as ::modular_bitfield::Specifier>::Bytes = {
                    ::modular_bitfield::private::read_specifier::<
                        MinimalAdt,
                    >(
                        &array[..],
                        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS,
                    )
                };
                let __bf_temp_0 = <MinimalAdt as ::modular_bitfield::Specifier>::from_bytes(
                        __bf_read,
                    )
                    .map_err(|_| ::modular_bitfield::error::InvalidBitPattern::new(
                        bytes,
                    ))?;
                Ok(NestedMinimal::Inner(__bf_temp_0))
            }
            __Bf_NestedMinimal_Tag::Just => {
                let __bf_read: <TwoBit as ::modular_bitfield::Specifier>::Bytes = {
                    ::modular_bitfield::private::read_specifier::<
                        TwoBit,
                    >(
                        &array[..],
                        <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS,
                    )
                };
                let __bf_temp_0 = <TwoBit as ::modular_bitfield::Specifier>::from_bytes(
                        __bf_read,
                    )
                    .map_err(|_| ::modular_bitfield::error::InvalidBitPattern::new(
                        bytes,
                    ))?;
                Ok(NestedMinimal::Just(__bf_temp_0))
            }
            #[allow(unreachable_patterns)]
            _ => {
                ::core::result::Result::Err(
                    ::modular_bitfield::error::InvalidBitPattern::new(bytes),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for NestedMinimal {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            NestedMinimal::Inner(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Inner", &__self_0)
            }
            NestedMinimal::Just(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Just", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for NestedMinimal {
    #[inline]
    fn clone(&self) -> NestedMinimal {
        let _: ::core::clone::AssertParamIsClone<MinimalAdt>;
        let _: ::core::clone::AssertParamIsClone<TwoBit>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for NestedMinimal {}
fn main() {
    let adt = MinimalAdt::Small(TwoBit::A);
    let bytes = MinimalAdt::into_bytes(adt).unwrap();
    {
        ::std::io::_print(format_args!("Generated code executed: {0:?}\n", bytes));
    };
}
