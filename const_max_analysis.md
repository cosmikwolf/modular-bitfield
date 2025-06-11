# `const_max()` Performance Issue Analysis

## Minimal Example

```rust
#[derive(BitfieldSpecifier)]
pub enum MinimalAdt {
    Small(TwoBit),        // 2 + 2 = 4 bits total
    Large(FourBit),       // 4 + 2 = 6 bits total 
    Unit,                 // 0 + 2 = 2 bits total
    Extra,                // 0 + 2 = 2 bits total
}

#[derive(BitfieldSpecifier)]  
pub enum NestedMinimal {
    Inner(MinimalAdt),    // 6 + 1 = 7 bits total
    Just(TwoBit),         // 2 + 1 = 3 bits total  
}
```

## Generated Code Analysis

### 1. Simple ADT Size Calculation

**Generated `const_max()` for MinimalAdt:**
```rust
const BITS: usize = ::modular_bitfield::private::const_max(
    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
        + <TwoBit as ::modular_bitfield::Specifier>::BITS,    // 2 + 2 = 4
    <__Bf_MinimalAdt_Tag as ::modular_bitfield::Specifier>::BITS
        + <FourBit as ::modular_bitfield::Specifier>::BITS,  // 2 + 4 = 6
);
// Result: const_max(4, 6) = 6 bits
```

### 2. Nested ADT Size Calculation

**Generated `const_max()` for NestedMinimal:**
```rust
const BITS: usize = ::modular_bitfield::private::const_max(
    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
        + <MinimalAdt as ::modular_bitfield::Specifier>::BITS,  // 1 + 6 = 7
    <__Bf_NestedMinimal_Tag as ::modular_bitfield::Specifier>::BITS
        + <TwoBit as ::modular_bitfield::Specifier>::BITS,     // 1 + 2 = 3
);
// Result: const_max(7, 3) = 7 bits
```

### 3. Performance-Killing Array Allocation

**Every variant allocates the maximum size:**
```rust
fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
    match input {
        MinimalAdt::Small(__bf_temp_0) => {
            // PROBLEM: Even Small variant allocates max size (6 bits = 8 bytes)
            let mut __bf_bytes = [0u8; (((const_max(4, 6) - 1) / 8) + 1) * 8 / 8usize];
            //                           ^^^^^^^^^^^^^^^^^^
            //                           Always uses MAXIMUM size!
            
            // Write tag (2 bits)
            write_specifier(&mut __bf_bytes, 0, Tag::Small);
            
            // Write data (2 bits) 
            write_specifier(&mut __bf_bytes, 2, __bf_temp_0);
            
            // Convert array to bytes (expensive!)
            array_into_bytes(__bf_bytes)
        }
        MinimalAdt::Large(__bf_temp_0) => {
            // Same oversized allocation for Large variant
            let mut __bf_bytes = [0u8; (((const_max(4, 6) - 1) / 8) + 1) * 8 / 8usize];
            // ... similar code
        }
    }
}
```

## The Performance Problem

### 1. **Oversized Allocations**
- `Small(TwoBit)` needs only 4 bits but allocates 8 bytes (64 bits)
- `Unit` needs only 2 bits but allocates 8 bytes (64 bits)  
- **32x memory waste** for unit variants!

### 2. **Expensive Array Operations**
```rust
// Every call does this expensive conversion:
array_into_bytes(__bf_bytes)  // from array_bytes_conv.rs
```

### 3. **Nested Compounding**
For `NestedMinimal::Inner(MinimalAdt::Small(TwoBit::A))`:
1. `TwoBit` conversion: 2 bits → efficient
2. `MinimalAdt` conversion: allocates 6 bits (3x waste)
3. `NestedMinimal` conversion: allocates 7 bits (3.5x waste)
4. **Multiple array conversions** at each level

### 4. **Flamegraph Evidence**
- `core::num::<impl u16>::to_ne_bytes` (32.44%) - array conversions
- `core::ptr::read_volatile` (17.46%) - memory access overhead
- Tag operations only 13.39% - NOT the main bottleneck!

## Benchmark Results Explained

| Operation | Time (ns) | Overhead |
|-----------|-----------|----------|
| Handwritten | 3.90 | 1x (baseline) |
| SimpleAdt roundtrip | 11.65 | 3x |
| NestedAdt roundtrip | 30.78 | **8x** |

The 8x slowdown for nested ADTs is directly caused by:
1. Recursive `const_max()` calculations
2. Multiple oversized array allocations 
3. Compound array conversion overhead

## Solution Direction

Instead of `const_max()` approach, use:
1. **Fixed-size allocations** per variant
2. **External discrimination** (no internal tags)
3. **Direct byte manipulation** (no array conversions)

This would bring performance much closer to the handwritten baseline.