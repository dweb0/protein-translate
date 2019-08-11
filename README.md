# protein-translate

[![Build Status](https://travis-ci.com/dweb0/protein-translate.svg?token=EQz1tk6xqYMBC8vjUmyv&branch=master)](https://travis-ci.com/dweb0/protein-translate)
[![Cargo](https://img.shields.io/crates/v/protein-translate.svg)](https://crates.io/crates/protein-translate)
[![Documentation](https://docs.rs/protein-translate/badge.svg)](https://docs.rs/protein-translate)

Translate nucleotide sequence (dna or rna) to protein.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
protein-translate = "0.2.0"
```

## Example

```rust
use protein_translate::translate;

fn main() {
    let dna = b"GTGAGTCGTTGAGTCTGATTGCGTATC";
    let protein = translate(dna);
    assert_eq!("VSR*V*LRI", &protein);

    // To shift reading frame
    let protein_frame2 = translate(&dna[1..]);
    assert_eq!("*VVESDCV", &protein_frame2);

}
```

## Benchmarks

The current algorithm is inspired by [seqan's](https://github.com/seqan/seqan/blob/master/include/seqan/translation/translation_tables.h) implementation which uses array indexing. Here is how it performs vs other methods (tested on 2012 macbook pro).

| Method | 10 bp* | 100 bp | 1,000 bp | 10,000 bp | 100,000 bp | 1 million bp |
| ------ | ---- | ----- | ------- | -------- | --------- | ------- |
| protein_translate | **91 ns** | **0.29 μs** | **2.28 μs** | **23 μs** | **215 μs** | **2.25 ms** |
| fnv hashmap | 111 ns | 0.37 μs | 3.58 μs | 37 μs | 366 us | 3.86 ms |
| std hashmap | 160 ns | 1.03 μs | 9.65 μs | 100 μs | 943 μs | 9.40 ms |
| phf_map | 177 ns | 1.04 μs | 9.47 μs | 100 μs | 936 μs | 9.91 |
| match statement | 259 ns | 1.77 μs | 17.9 μs | 163 μs | 1941 μs | 19.1 ms |
| protein_translate (unchecked) | 90 ns | 0.26 μs | 2.02 μs | 20 μs | 197 μs | 1.92 ms |

> *bp = "base pairs"  

To benchmark yourself (have to use nightly because of phf_map macro).

```
cargo +nightly bench
```

### Thoughts

* [FNV](https://github.com/servo/rust-fnv) seems to be a great option, but I have chosen to use the current implementation due to being slightly faster and not required any dependencies.
* There was originally a function called `translate_unchecked` that did not validate each byte for valid ASCII, but since the performance gain was negligible, it was removed.

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
