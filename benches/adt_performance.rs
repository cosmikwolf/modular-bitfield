#![allow(dead_code)]

mod utils;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use modular_bitfield::{
    bitfield,
    prelude::*,
};
use utils::repeat;

// Basic unit enums for ADT construction
#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
pub enum Two {
    Zero,
    One,
    Two,
    Three,
}

#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
#[bits = 4]
pub enum Four {
    Zero = 0,
    Fifteen = 15,
}

#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
#[bits = 5]
pub enum Five {
    Zero = 0,
    ThirtyOne = 31,
}

// Daniel's ADT approach: Internal tag + variable data
#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
#[bits = 2]
pub enum SimpleAdt {
    First(Two, Four), // 2 + 4 = 6 bits data + 2 bits tag
    Second(Two),      // 2 bits data + 2 bits tag
    Third,            // 0 bits data + 2 bits tag
    Fourth,           // 0 bits data + 2 bits tag
}

#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
pub enum NestedAdt {
    This(SimpleAdt, Two),         // SimpleAdt + 2 bits + tag
    That { foo: Two, bar: Four }, // 2 + 4 = 6 bits + tag
}

// Larger ADT for testing more complex cases
#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
#[bits = 3]
pub enum LargeAdt {
    Small(Two),        // 2 bits data + 3 bits tag
    Medium(Four),      // 4 bits data + 3 bits tag
    Large(Five),       // 5 bits data + 3 bits tag
    Nested(SimpleAdt), // Variable bits (ADT within ADT) + 3 bits tag
    Unit,              // 0 bits data + 3 bits tag
}

// Custom tag type for testing explicit tag control
#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
#[bits = 3]
pub enum CustomTag {
    TypeA = 0,
    TypeB = 1,
    TypeC = 2,
    TypeD = 3,
    TypeE = 4,
    TypeF = 5,
    TypeG = 6,
    TypeH = 7,
}

// ADT with custom tag
#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
#[tag(CustomTag)]
pub enum TaggedAdt {
    TypeA(Two),
    TypeB(Four),
    TypeC(Five),
    TypeD,
    TypeE,
    TypeF,
    TypeG,
    TypeH,
}

// Handwritten equivalent for comparison
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HandwrittenAdt {
    data: u16,
}

impl HandwrittenAdt {
    fn new_first(two_val: u8, four_val: u8) -> Self {
        // Tag = 0 (2 bits), Two value (2 bits), Four value (4 bits)
        let data = (0u16 << 6) | ((two_val as u16 & 0x3) << 4) | (four_val as u16 & 0xF);
        Self { data }
    }

    fn new_second(two_val: u8) -> Self {
        // Tag = 1 (2 bits), Two value (2 bits), padding
        let data = (1u16 << 6) | ((two_val as u16 & 0x3) << 4);
        Self { data }
    }

    fn new_third() -> Self {
        // Tag = 2 (2 bits), no data
        Self { data: 2u16 << 6 }
    }

    fn new_fourth() -> Self {
        // Tag = 3 (2 bits), no data
        Self { data: 3u16 << 6 }
    }

    fn into_bytes(self) -> u16 {
        self.data
    }
    fn from_bytes(data: u16) -> Self {
        Self { data }
    }

    fn get_tag(&self) -> u8 {
        ((self.data >> 6) & 0x3) as u8
    }
}

fn bench_simple_adt_into_bytes(c: &mut Criterion) {
    let mut g = c.benchmark_group("SimpleAdt_into_bytes");

    g.bench_function("First", |b| {
        let input = SimpleAdt::First(Two::Two, Four::Fifteen);
        b.iter(|| {
            repeat(|| {
                black_box(SimpleAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Second", |b| {
        let input = SimpleAdt::Second(Two::Three);
        b.iter(|| {
            repeat(|| {
                black_box(SimpleAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Third", |b| {
        let input = SimpleAdt::Third;
        b.iter(|| {
            repeat(|| {
                black_box(SimpleAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Fourth", |b| {
        let input = SimpleAdt::Fourth;
        b.iter(|| {
            repeat(|| {
                black_box(SimpleAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });
}

fn bench_simple_adt_from_bytes(c: &mut Criterion) {
    let mut g = c.benchmark_group("SimpleAdt_from_bytes");

    g.bench_function("First", |b| {
        let input = 0b00000000u8; // Safe test pattern
        b.iter(|| {
            repeat(|| {
                black_box(SimpleAdt::from_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Second", |b| {
        let input = 0b00000001u8; // Safe test pattern
        b.iter(|| {
            repeat(|| {
                black_box(SimpleAdt::from_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Third", |b| {
        let input = 0b00000010u8; // Safe test pattern
        b.iter(|| {
            repeat(|| {
                black_box(SimpleAdt::from_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Fourth", |b| {
        let input = 0b00000011u8; // Safe test pattern
        b.iter(|| {
            repeat(|| {
                black_box(SimpleAdt::from_bytes(black_box(input)).unwrap());
            });
        });
    });
}

fn bench_simple_adt_roundtrip(c: &mut Criterion) {
    let mut g = c.benchmark_group("SimpleAdt_roundtrip");

    g.bench_function("First", |b| {
        let input = SimpleAdt::First(Two::One, Four::Zero);
        b.iter(|| {
            repeat(|| {
                let bytes = SimpleAdt::into_bytes(black_box(input)).unwrap();
                black_box(SimpleAdt::from_bytes(bytes).unwrap());
            });
        });
    });

    g.bench_function("Second", |b| {
        let input = SimpleAdt::Second(Two::Two);
        b.iter(|| {
            repeat(|| {
                let bytes = SimpleAdt::into_bytes(black_box(input)).unwrap();
                black_box(SimpleAdt::from_bytes(bytes).unwrap());
            });
        });
    });

    g.bench_function("Unit", |b| {
        let input = SimpleAdt::Third;
        b.iter(|| {
            repeat(|| {
                let bytes = SimpleAdt::into_bytes(black_box(input)).unwrap();
                black_box(SimpleAdt::from_bytes(bytes).unwrap());
            });
        });
    });
}

fn bench_nested_adt(c: &mut Criterion) {
    let mut g = c.benchmark_group("NestedAdt");

    g.bench_function("into_bytes_this", |b| {
        let input = NestedAdt::This(SimpleAdt::First(Two::Two, Four::Fifteen), Two::One);
        b.iter(|| {
            repeat(|| {
                black_box(NestedAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("into_bytes_that", |b| {
        let input = NestedAdt::That {
            foo: Two::Three,
            bar: Four::Zero,
        };
        b.iter(|| {
            repeat(|| {
                black_box(NestedAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("from_bytes", |b| {
        let input = 0b0000000000000000u16; // Safe test pattern
        b.iter(|| {
            repeat(|| {
                black_box(NestedAdt::from_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("roundtrip", |b| {
        let input = NestedAdt::This(SimpleAdt::Second(Two::Zero), Two::Three);
        b.iter(|| {
            repeat(|| {
                let bytes = NestedAdt::into_bytes(black_box(input)).unwrap();
                black_box(NestedAdt::from_bytes(bytes).unwrap());
            });
        });
    });
}

fn bench_large_adt(c: &mut Criterion) {
    let mut g = c.benchmark_group("LargeAdt");

    g.bench_function("Small", |b| {
        let input = LargeAdt::Small(Two::Three);
        b.iter(|| {
            repeat(|| {
                black_box(LargeAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Medium", |b| {
        let input = LargeAdt::Medium(Four::Fifteen);
        b.iter(|| {
            repeat(|| {
                black_box(LargeAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Large", |b| {
        let input = LargeAdt::Large(Five::ThirtyOne);
        b.iter(|| {
            repeat(|| {
                black_box(LargeAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Nested", |b| {
        let input = LargeAdt::Nested(SimpleAdt::First(Two::One, Four::Zero));
        b.iter(|| {
            repeat(|| {
                black_box(LargeAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("Unit", |b| {
        let input = LargeAdt::Unit;
        b.iter(|| {
            repeat(|| {
                black_box(LargeAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });
}

fn bench_tagged_adt(c: &mut Criterion) {
    let mut g = c.benchmark_group("TaggedAdt");

    g.bench_function("TypeA", |b| {
        let input = TaggedAdt::TypeA(Two::Two);
        b.iter(|| {
            repeat(|| {
                black_box(TaggedAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("TypeB", |b| {
        let input = TaggedAdt::TypeB(Four::Fifteen);
        b.iter(|| {
            repeat(|| {
                black_box(TaggedAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("TypeC", |b| {
        let input = TaggedAdt::TypeC(Five::ThirtyOne);
        b.iter(|| {
            repeat(|| {
                black_box(TaggedAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("TypeD_unit", |b| {
        let input = TaggedAdt::TypeD;
        b.iter(|| {
            repeat(|| {
                black_box(TaggedAdt::into_bytes(black_box(input)).unwrap());
            });
        });
    });

    g.bench_function("roundtrip", |b| {
        let input = TaggedAdt::TypeB(Four::Zero);
        b.iter(|| {
            repeat(|| {
                let bytes = TaggedAdt::into_bytes(black_box(input)).unwrap();
                black_box(TaggedAdt::from_bytes(bytes).unwrap());
            });
        });
    });
}

fn bench_handwritten_comparison(c: &mut Criterion) {
    let mut g = c.benchmark_group("Handwritten_ADT");

    g.bench_function("into_bytes_first", |b| {
        let input = HandwrittenAdt::new_first(2, 15);
        b.iter(|| {
            repeat(|| {
                black_box(black_box(input).into_bytes());
            });
        });
    });

    g.bench_function("into_bytes_second", |b| {
        let input = HandwrittenAdt::new_second(3);
        b.iter(|| {
            repeat(|| {
                black_box(black_box(input).into_bytes());
            });
        });
    });

    g.bench_function("into_bytes_unit", |b| {
        let input = HandwrittenAdt::new_third();
        b.iter(|| {
            repeat(|| {
                black_box(black_box(input).into_bytes());
            });
        });
    });

    g.bench_function("from_bytes", |b| {
        let input = 0b0000000000000000u16; // Safe test pattern
        b.iter(|| {
            repeat(|| {
                black_box(HandwrittenAdt::from_bytes(black_box(input)));
            });
        });
    });

    g.bench_function("roundtrip", |b| {
        let input = HandwrittenAdt::new_first(1, 0);
        b.iter(|| {
            repeat(|| {
                let bytes = black_box(input).into_bytes();
                black_box(HandwrittenAdt::from_bytes(bytes));
            });
        });
    });
}

fn bench_bitfield_usage(c: &mut Criterion) {
    #[bitfield(filled = false)]
    #[derive(Clone, Copy)]
    struct AdtPacket {
        header: B8,
        adt: SimpleAdt,
        footer: B6,
    }

    #[bitfield(filled = false)]
    #[derive(Clone, Copy)]
    struct NestedAdtPacket {
        header: B4,
        nested: NestedAdt,
        padding: B4,
    }

    let mut g = c.benchmark_group("Bitfield_Usage");

    g.bench_function("adt_packet_get", |b| {
        let input = AdtPacket::new()
            .with_header(0xFF)
            .with_adt(SimpleAdt::First(Two::Two, Four::Fifteen))
            .with_footer(0x3F);
        b.iter(|| {
            repeat(|| {
                black_box(black_box(&input).adt());
            });
        });
    });

    g.bench_function("adt_packet_set", |b| {
        let mut input = AdtPacket::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_adt(SimpleAdt::Second(Two::One));
            });
        });
    });

    g.bench_function("nested_adt_packet_get", |b| {
        let input = NestedAdtPacket::new().with_nested(NestedAdt::That {
            foo: Two::Three,
            bar: Four::Zero,
        });
        b.iter(|| {
            repeat(|| {
                black_box(black_box(&input).nested());
            });
        });
    });
}

criterion_group!(
    adt_benchmarks,
    bench_simple_adt_into_bytes,
    bench_simple_adt_from_bytes,
    bench_simple_adt_roundtrip,
    bench_nested_adt,
    bench_large_adt,
    bench_tagged_adt,
    bench_handwritten_comparison,
    bench_bitfield_usage
);
criterion_main!(adt_benchmarks);

