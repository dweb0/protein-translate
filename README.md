# `protein-translate`

Translate nucleotide sequence (dna or rna) to protein.

[![Build Status](https://travis-ci.com/dweb0/protein-translate.svg?token=EQz1tk6xqYMBC8vjUmyv&branch=master)](https://travis-ci.com/dweb0/protein-translate)
[![Cargo](https://img.shields.io/crates/v/protein-translate.svg)](https://crates.io/crates/protein-translate)
[![Documentation](https://docs.rs/protein-translate/badge.svg)](https://docs.rs/protein-translate)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
protein-translate = "0.1.0"
```

## Example

```rust
use protein_translate::translate;

fn main() {
    let dna = "GTGAGTCGTTGAGTCTGATTGCGTATC";
    let protein = translate(dna);
    assert_eq!("VSR*V*LRI", &protein);

    // To shift reading frame
    let protein_frame2 = translate(&dna[1..]);
    assert_eq!("*VVESDCV", &protein_frame2);

}
```

## Benchmarks

The current algorithm is inspired by [seqan's](https://github.com/seqan/seqan/blob/master/include/seqan/translation/translation_tables.h) implementation which uses array indexing. Here is how it performs vs other methods.


| Method | 10 bp* | 100 bp | 1,000 bp | 10,000 bp | 100,000 bp |
| ------ | ---- | ----- | ------- | -------- | --------- |
| protein_translate | **87 ns** | **0.22 μs** | **1.74 μs** | **15 μs** | **157 μs** |
| lazy_static | 159 ns | 1.02 μs | 9.69 μs | 100 μs | 966 μs |
| phf_map | 190 ns | 1.13 μs | 10.56 μs | 105 μs | 1021 μs |
| match statement | 270 ns | 1.74 μs | 18.4 μs | 173 μs | 1907 μs |

* bp = "base pairs"

You benchmark yourself (have to use nightly because of phf_map macro).

```
cargo +nightly bench
```

If you have a better implementation feel free to submit a merge request!

## Todo
* Add other Codon tables (e.g. Vertebrate Mitochondrial, Yeast Mitochondrial, Mold Mitochondrial, etc.)
* Add support for ambiguous nucleotides (right now, only supports A, U, T, C, G)

## Tests

To test

```
cargo test
```

To can also generate new test data (requires python3 and [biopython](https://github.com/biopython/biopython)).

```bash
# Generate 500 random sequences and their peptides
python3 tests/generate_test_data.py 500
```