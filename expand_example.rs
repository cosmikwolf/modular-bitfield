// Minimal example to expand and show const_max usage
use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug, Clone, Copy)]
pub enum TwoBit {
    A, B, C, D,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy)]
#[bits = 4] 
pub enum FourBit {
    Small = 0,
    Large = 15,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy)]
pub enum MinimalAdt {
    Small(TwoBit),        // 2 + 2 = 4 bits total
    Large(FourBit),       // 4 + 2 = 6 bits total 
    Unit,                 // 0 + 2 = 2 bits total
    Extra,                // 0 + 2 = 2 bits total
}
// const_max(const_max(4, 6), const_max(2, 2)) = const_max(6, 2) = 6 bits

#[derive(BitfieldSpecifier, Debug, Clone, Copy)]  
pub enum NestedMinimal {
    Inner(MinimalAdt),    // 6 + 1 = 7 bits total
    Just(TwoBit),         // 2 + 1 = 3 bits total  
}
// const_max(7, 3) = 7 bits

fn main() {
    let adt = MinimalAdt::Small(TwoBit::A);
    let bytes = MinimalAdt::into_bytes(adt).unwrap();
    println!("Generated code executed: {:?}", bytes);
}