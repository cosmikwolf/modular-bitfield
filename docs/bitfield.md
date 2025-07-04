Applicable to structs to turn their fields into compact bitfields.

# Generated API

By default this generates the following API:

- **Constructors:**

    1. `new()`: When any field has a `#[default(...)]` attribute, this applies the specified 
       default values. When no defaults are specified, initializes all bits to 0.
       Note that invalid bit patterns are supported in that getters and setters will
       be protecting accesses. This constructor is always `const fn`.
    2. `new_zeroed()`: Generated when any field has a `#[default(...)]` attribute.
       Always initializes all bits to 0, ignoring any default values.
       This constructor is always `const fn`.

- **Getters:**

    For every field `f` we generate the following getters:

    1. `f()`: Returns the value of `f` and might panic
       if the value contains an invalid bit pattern.
    2. `f_or_err()`: Returns the value of `f` or an error
       if the value contains an invalid bit pattern.

- **Setters:**

    For every field `f` we generate the following setters:

    1. `set_f(new_value)`: Sets the value of `f` to `new_value` and might panic
       if `new_value` is out of bounds for the bit width of `f`.
    2. `set_f_checked(new_value)`: Sets the value of `f` to `new` or returns an error
       if `new_value` if out of bounds for the bit width of `f`.
    3. `with_f(new_value)`: Similar to `set_f` but consumes and returns `Self`.
       Primarily useful for method chaining.
    4. `with_f_checked(new_value)`: Similar to `set_f_checked` but consumes and returns `Self`.
       Primarily useful for method chaining.

- **Conversions:**

    - `from_bytes(bytes)`: Allows to constructor the bitfield type from a fixed array of bytes.
    - `into_bytes()`: Allows to convert the bitfield into its underlying byte representation.

# Parameters

The following parameters for the `#[bitfield]` macro are supported:

## Parameter: `bytes = N`

This ensures at compilation time that the resulting `#[bitfield]` struct consists of
exactly `N` bytes. Yield a compilation error if this does not hold true.

### Example

```
# use modular_bitfield::prelude::*;
#[bitfield(bytes = 2)]
pub struct SingedInt {
    sign: bool, //  1 bit
    value: B15, // 15 bits
}
```

## Parameter: `filled: bool`

If `filled` is `true` ensures that the `#[bitfield]` struct defines all bits and
therefore has a bitwidth that is divisible by 8. If `filled` is `false` ensures the
exact opposite.

The default value is: `true`

### Example

```
# use modular_bitfield::prelude::*;
#[bitfield(filled = false)]
pub struct Package {
    is_received: bool, // 1 bit
    is_alive: bool,    // 1 bit
    status: B2,        // 2 bits
}
```

## Parameter: `bits = N`

With the `bits: int` parameter it is possible to control the targeted bit width of
a `#[bitfield]` annoated struct. Using `bits = N` guarantees that the resulting bitfield
struct will have a bit width of exactly `N`.

### Example 1

```
# use modular_bitfield::prelude::*;
#[bitfield(bits = 16)]
pub struct Package {
    is_received: bool, // 1 bit
    is_alive: bool,    // 1 bit
    status: B14,       // 14 bits
}
```

### Example 2

The `bits: int` parameter is especially useful when using this in conjunction with
`#[derive(Specifier)]` and `filled = false` as shown in the below example.

```
# use modular_bitfield::prelude::*;
#[bitfield(bits = 5)]
#[derive(Specifier)]
pub struct Package {
    is_received: bool, // 1 bit
    is_alive: bool,    // 1 bit
    status: B3,        // 3 bits
}
```

## Field Parameter: `#[bits = N]`

To ensure at compile time that a field of a `#[bitfield]` struct has a bit width of exactly
`N` a user may add `#[bits = N]` to the field in question.

### Example

```
# use modular_bitfield::prelude::*;
# #[bitfield(filled = false)]
# #[derive(Specifier)]
# pub struct Header {
#     is_received: bool, // 1 bit
#     is_alive: bool,    // 1 bit
#     status: B2,        // 2 bits
# }
#[bitfield]
pub struct Base {
    #[bits = 4]
    header: Header, //  4 bits
    content: B28,   // 28 bits
}
```

## Field Parameter: `#[skip(..)]`

It is possible to skip the entire code generation for getters or setters with the `#[skip]`
field attribute.
This is useful if a field just needs to be read or written exclusively. Skipping both
setters and getters is useful if you want to have undefined blocks within your bitfields.

### Example

```
# use modular_bitfield::prelude::*;
#[bitfield]
pub struct Sparse {
    #[skip(getters)]
    no_getters: B4,
    #[skip(setters)]
    no_setters: B4,
    #[skip]
    skipped_entirely: B4,
    #[skip(getters, setters)]
    skipped_entirely_2: B2,
    #[skip(getters)] #[skip(setters)]
    skipped_entirely_2: B2,
}
```

### Trick: Wildcards

If you are completely uninterested in a field of a bitfield, for example when specifying
some undefined bits in your bitfield you can use double wildcards as their names:

```
# use modular_bitfield::prelude::*;
#[bitfield]
pub struct Sparse {
    #[skip] __: B10,
    a: bool,
    #[skip] __: B10,
    b: bool,
    #[skip] __: B10,
}
```

## Field Parameter: `#[default(...)]`

The `#[default(...)]` attribute allows you to specify a default value for a field.
When using this attribute, the `new()` constructor applies the specified default values,
and an additional `new_zeroed()` constructor is generated for explicit zero initialization.

### Example

```
# use modular_bitfield::prelude::*;
#[bitfield]
pub struct Config {
    enabled: bool,
    #[default(true)]
    auto_restart: bool,
    #[default(5)]
    retry_count: B6,
    #[default(0xFF)]
    flags: B8,
}

// Create with default values applied
let config1 = Config::new();
assert_eq!(config1.auto_restart(), true);
assert_eq!(config1.retry_count(), 5);
assert_eq!(config1.flags(), 0xFF);

// Create with all zeros
let config2 = Config::new_zeroed();
assert_eq!(config2.auto_restart(), false);
assert_eq!(config2.retry_count(), 0);
assert_eq!(config2.flags(), 0);
```

### Limitations

- The `#[default(...)]` attribute cannot be used on fields that skip setter generation
  (i.e., fields marked with `#[skip]` or `#[skip(setters)]`).
- Default values must be valid for the field's type and bit width.
- Default expressions must be const-evaluable (literals, const variables, enum variants, simple const expressions).
- Both `new()` and `new_zeroed()` constructors are `const fn` and work in const contexts.

# Features

## Support: `#[derive(Specifier)]`

If a `#[bitfield]` struct is annotated with a `#[derive(Specifier)]` attribute
an implementation of the `Specifier` trait will be generated for it. This has the effect
that the bitfield struct itself can be used as the type of a field of another bitfield type.

This feature is limited to bitfield types that have a total bit width of 128 bit or fewer.
This restriction is ensured at compile time.

### Example

```
# use modular_bitfield::prelude::*;
#[bitfield(filled = false)]
#[derive(Specifier)]
pub struct Header {
    is_received: bool, // 1 bit
    is_alive: bool,    // 1 bit
    status: B2,        // 2 bits
}
```

Now the above `Header` bitfield type can be used in yet another `#[bitfield]` annotated type:

```
# use modular_bitfield::prelude::*;
# #[bitfield(filled = false)]
# #[derive(Specifier)]
# pub struct Header {
#     is_received: bool, // 1 bit
#     is_alive: bool,    // 1 bit
#     status: B2,        // 2 bits
# }
#[bitfield]
pub struct Base {
    header: Header, //  4 bits
    content: B28,   // 28 bits
}
```

## Support: `#[derive(Default)]`

If a `#[derive(Default)]` is found by the `#[bitfield]` a `Default` implementation
is generated that calls `Self::new()`. This means that when fields have `#[default(...)]`
attributes, those default values will be applied. When no defaults are specified,
all fields are zero-initialized.

### Example

```
# use modular_bitfield::prelude::*;
#[bitfield]
#[derive(Default)]
pub struct Config {
    enabled: bool,
    #[default(true)]
    auto_restart: bool,
    #[default(5)]
    retry_count: B6,
    flags: B8, // no default, so zero-initialized
}

let config = Config::default();
assert_eq!(config.enabled(), false);     // zero-initialized
assert_eq!(config.auto_restart(), true); // uses default value
assert_eq!(config.retry_count(), 5);     // uses default value
assert_eq!(config.flags(), 0);           // zero-initialized
```

## Support: `#[derive(Debug)]`

If a `#[derive(Debug)]` is found by the `#[bitfield]` a naturally formatting implementation
is going to be generated that clearly displays all the fields and their values as the user
would expect.
Also invalid bit patterns for fields are clearly displayed under this implementation.

### Example

```
# use modular_bitfield::prelude::*;
#[bitfield]
#[derive(Debug)]
pub struct Package {
    is_received: bool, // 1 bit
    is_alive: bool,    // 1 bit
    status: B6,        // 6 bits
}

let package = Package::new()
    .with_is_received(false)
    .with_is_alive(true)
    .with_status(3);
println!("{:?}", package);
assert_eq!(
    format!("{:?}", package),
    "Package { is_received: false, is_alive: true, status: 3 }",
);
```

## Support: `#[repr(uN)]`

It is possible to additionally annotate a `#[bitfield]` annotated struct with `#[repr(uN)]`
where `uN` is one of `u8`, `u16`, `u32`, `u64` or `u128` in order to make it conveniently
interchangeable with such an unsigned integer value.

As an effect to the user this implements `From` implementations between the chosen primitive
and the bitfield as well as ensuring at compile time that the bit width of the bitfield struct
matches the bit width of the primitive.

### Example

```
# use modular_bitfield::prelude::*;
#[bitfield]
#[repr(u16)]
pub struct SignedU16 {
    sign: bool,     //  1 bit
    abs_value: B15, // 15 bits
}

let sint = SignedU16::from(0b0111_0001);
assert_eq!(sint.sign(), true);
assert_eq!(sint.abs_value(), 0b0011_1000);
assert_eq!(u16::from(sint), 0b0111_0001_u16);
```
