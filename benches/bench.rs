#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate criterion;
use criterion::{Criterion, black_box};


mod using_lazy_static {

    use std::collections::HashMap;

    lazy_static! {
        static ref AA_TABLE: HashMap<&'static [u8], char> = {
            let mut m: HashMap<&'static [u8], char> = HashMap::new();
            m.insert(b"TAA", '*');
            m.insert(b"TAG", '*');
            m.insert(b"TGA", '*');
            m.insert(b"UAA", '*');
            m.insert(b"UAG", '*');
            m.insert(b"UGA", '*');
            m.insert(b"GCA", 'A');
            m.insert(b"GCC", 'A');
            m.insert(b"GCG", 'A');
            m.insert(b"GCT", 'A');
            m.insert(b"GCU", 'A');
            m.insert(b"TGC", 'C');
            m.insert(b"TGT", 'C');
            m.insert(b"UGC", 'C');
            m.insert(b"UGU", 'C');
            m.insert(b"GAC", 'D');
            m.insert(b"GAT", 'D');
            m.insert(b"GAU", 'D');
            m.insert(b"GAA", 'E');
            m.insert(b"GAG", 'E');
            m.insert(b"TTC", 'F');
            m.insert(b"TTT", 'F');
            m.insert(b"UUC", 'F');
            m.insert(b"UUU", 'F');
            m.insert(b"GGA", 'G');
            m.insert(b"GGC", 'G');
            m.insert(b"GGG", 'G');
            m.insert(b"GGT", 'G');
            m.insert(b"GGU", 'G');
            m.insert(b"CAC", 'H');
            m.insert(b"CAT", 'H');
            m.insert(b"CAU", 'H');
            m.insert(b"ATA", 'I');
            m.insert(b"ATC", 'I');
            m.insert(b"ATT", 'I');
            m.insert(b"AUA", 'I');
            m.insert(b"AUC", 'I');
            m.insert(b"AUU", 'I');
            m.insert(b"AAA", 'K');
            m.insert(b"AAG", 'K');
            m.insert(b"CTA", 'L');
            m.insert(b"CTC", 'L');
            m.insert(b"CTG", 'L');
            m.insert(b"CTT", 'L');
            m.insert(b"CUA", 'L');
            m.insert(b"CUC", 'L');
            m.insert(b"CUG", 'L');
            m.insert(b"CUU", 'L');
            m.insert(b"TTA", 'L');
            m.insert(b"TTG", 'L');
            m.insert(b"UUA", 'L');
            m.insert(b"UUG", 'L');
            m.insert(b"ATG", 'M');
            m.insert(b"AUG", 'M');
            m.insert(b"AAC", 'N');
            m.insert(b"AAT", 'N');
            m.insert(b"AAU", 'N');
            m.insert(b"CCA", 'P');
            m.insert(b"CCC", 'P');
            m.insert(b"CCG", 'P');
            m.insert(b"CCT", 'P');
            m.insert(b"CCU", 'P');
            m.insert(b"CAA", 'Q');
            m.insert(b"CAG", 'Q');
            m.insert(b"AGA", 'R');
            m.insert(b"AGG", 'R');
            m.insert(b"CGA", 'R');
            m.insert(b"CGC", 'R');
            m.insert(b"CGG", 'R');
            m.insert(b"CGT", 'R');
            m.insert(b"CGU", 'R');
            m.insert(b"AGC", 'S');
            m.insert(b"AGT", 'S');
            m.insert(b"AGU", 'S');
            m.insert(b"TCA", 'S');
            m.insert(b"TCC", 'S');
            m.insert(b"TCG", 'S');
            m.insert(b"TCT", 'S');
            m.insert(b"UCA", 'S');
            m.insert(b"UCC", 'S');
            m.insert(b"UCG", 'S');
            m.insert(b"UCU", 'S');
            m.insert(b"ACA", 'T');
            m.insert(b"ACC", 'T');
            m.insert(b"ACG", 'T');
            m.insert(b"ACT", 'T');
            m.insert(b"ACU", 'T');
            m.insert(b"GTA", 'V');
            m.insert(b"GTC", 'V');
            m.insert(b"GTG", 'V');
            m.insert(b"GTT", 'V');
            m.insert(b"GUA", 'V');
            m.insert(b"GUC", 'V');
            m.insert(b"GUG", 'V');
            m.insert(b"GUU", 'V');
            m.insert(b"TGG", 'W');
            m.insert(b"UGG", 'W');
            m.insert(b"UAC", 'Y');
            m.insert(b"TAC", 'Y');
            m.insert(b"TAT", 'Y');
            m.insert(b"UAU", 'Y');
            m
        };
    }

    pub fn translate(seq: &str) -> String {

        let len = seq.len();
        let bytes = seq.as_bytes();
        let mut peptide = String::with_capacity(len);
        
        for i in (0..len - 2 - (len % 3)).step_by(3) {
            let amino_acid = AA_TABLE.get(&bytes[i..i+3]).unwrap_or(&'X');
            peptide.push(*amino_acid);
        }
        peptide
    }
}


mod using_match {
    fn triplet_to_char(triplet: &[u8]) -> char {
        match triplet {
            b"TAA" | b"TAG" | b"TGA" | b"UAA" | b"UAG" | b"UGA" => '*',
            b"GCA" | b"GCC" | b"GCG" | b"GCT" | b"GCU"  => 'A',
            b"TGC" | b"TGT" | b"UGC" | b"UGU" => 'C',
            b"GAC" | b"GAT" | b"GAU" => 'D',
            b"GAA" | b"GAG" => 'E',
            b"TTC" | b"TTT" | b"UUC" | b"UUU" => 'F',
            b"GGA" | b"GGC" | b"GGG" | b"GGT" | b"GGU" => 'G',
            b"CAC" | b"CAT" | b"CAU" => 'H',
            b"ATA" | b"ATC" | b"ATT" | b"AUA" | b"AUC" | b"AUU" => 'I',
            b"AAA" | b"AAG" => 'K',
            b"CTA" | b"CTC" | b"CTG" | b"CTT" | b"CUA" | b"CUC" | b"CUG" | b"CUU" | b"TTA" | b"TTG" | b"UUA" | b"UUG" => 'L',
            b"ATG" | b"AUG" => 'M',
            b"AAC" | b"AAT" | b"AAU" => 'N',
            b"CCA" | b"CCC" | b"CCG" | b"CCT" | b"CCU" => 'P',
            b"CAA" | b"CAG" => 'Q',
            b"AGA" | b"AGG" | b"CGA" | b"CGC" | b"CGG" | b"CGT" | b"CGU" => 'R',
            b"AGC" | b"AGT" | b"AGU" | b"TCA" | b"TCC" | b"TCG" | b"TCT" | b"UCA" | b"UCC" | b"UCG" | b"UCU" => 'S',
            b"ACA" | b"ACC" | b"ACG" | b"ACT" | b"ACU" => 'T',
            b"GTA" | b"GTC" | b"GTG" | b"GTT" | b"GUA" | b"GUC" | b"GUG" | b"GUU" => 'V',
            b"TGG" | b"UGG" => 'W',
            b"UAC" | b"TAC" | b"TAT" | b"UAU" => 'Y',
            _ => 'X'
        }
    }

    pub fn translate(seq: &str) -> String {

        let len = seq.len();
        let bytes = seq.as_bytes();
        let mut peptide = String::with_capacity(len);
        
        for i in (0..len - 2 - (len % 3)).step_by(3) {
            let amino_acid = triplet_to_char(&bytes[i..i+3]);
            peptide.push(amino_acid);
        }
        peptide
    }
}

mod using_phf_map {
    use phf::phf_map;

    static AA_TABLE: phf::Map<&'static [u8], char> = phf_map! {
        b"TAA" => '*',
        b"TAG" => '*',
        b"TGA" => '*',
        b"UAA" => '*',
        b"UAG" => '*',
        b"UGA" => '*',
        b"GCA" => 'A',
        b"GCC" => 'A',
        b"GCG" => 'A',
        b"GCT" => 'A',
        b"GCU" => 'A',
        b"TGC" => 'C',
        b"TGT" => 'C',
        b"UGC" => 'C',
        b"UGU" => 'C',
        b"GAC" => 'D',
        b"GAT" => 'D',
        b"GAU" => 'D',
        b"GAA" => 'E',
        b"GAG" => 'E',
        b"TTC" => 'F',
        b"TTT" => 'F',
        b"UUC" => 'F',
        b"UUU" => 'F',
        b"GGA" => 'G',
        b"GGC" => 'G',
        b"GGG" => 'G',
        b"GGT" => 'G',
        b"GGU" => 'G',
        b"CAC" => 'H',
        b"CAT" => 'H',
        b"CAU" => 'H',
        b"ATA" => 'I',
        b"ATC" => 'I',
        b"ATT" => 'I',
        b"AUA" => 'I',
        b"AUC" => 'I',
        b"AUU" => 'I',
        b"AAA" => 'K',
        b"AAG" => 'K',
        b"CTA" => 'L',
        b"CTC" => 'L',
        b"CTG" => 'L',
        b"CTT" => 'L',
        b"CUA" => 'L',
        b"CUC" => 'L',
        b"CUG" => 'L',
        b"CUU" => 'L',
        b"TTA" => 'L',
        b"TTG" => 'L',
        b"UUA" => 'L',
        b"UUG" => 'L',
        b"ATG" => 'M',
        b"AUG" => 'M',
        b"AAC" => 'N',
        b"AAT" => 'N',
        b"AAU" => 'N',
        b"CCA" => 'P',
        b"CCC" => 'P',
        b"CCG" => 'P',
        b"CCT" => 'P',
        b"CCU" => 'P',
        b"CAA" => 'Q',
        b"CAG" => 'Q',
        b"AGA" => 'R',
        b"AGG" => 'R',
        b"CGA" => 'R',
        b"CGC" => 'R',
        b"CGG" => 'R',
        b"CGT" => 'R',
        b"CGU" => 'R',
        b"AGC" => 'S',
        b"AGT" => 'S',
        b"AGU" => 'S',
        b"TCA" => 'S',
        b"TCC" => 'S',
        b"TCG" => 'S',
        b"TCT" => 'S',
        b"UCA" => 'S',
        b"UCC" => 'S',
        b"UCG" => 'S',
        b"UCU" => 'S',
        b"ACA" => 'T',
        b"ACC" => 'T',
        b"ACG" => 'T',
        b"ACT" => 'T',
        b"ACU" => 'T',
        b"GTA" => 'V',
        b"GTC" => 'V',
        b"GTG" => 'V',
        b"GTT" => 'V',
        b"GUA" => 'V',
        b"GUC" => 'V',
        b"GUG" => 'V',
        b"GUU" => 'V',
        b"TGG" => 'W',
        b"UGG" => 'W',
        b"UAC" => 'Y',
        b"TAC" => 'Y',
        b"TAT" => 'Y',
        b"UAU" => 'Y',
    };


    pub fn translate(seq: &str) -> String {
        let len = seq.len();
        let bytes = seq.as_bytes();
        let mut peptide = String::with_capacity(len);
        
        for i in (0..len - 2 - (len % 3)).step_by(3) {
            let amino_acid = AA_TABLE.get(&bytes[i..i+3]).unwrap_or(&'X');
            peptide.push(*amino_acid);
        }
        peptide
    }
}


// 100,000 base pair sequence
static TEST_SEQ: &'static str = include_str!("test_seq.txt");

fn bench_current(c: &mut Criterion) {

    c.bench_function("current implementation", |b| {
        b.iter(|| protein_translate::translate(black_box(TEST_SEQ)))
    });
}

fn bench_lazy_static(c: &mut Criterion) {
    c.bench_function("lazy static", |b| {
        b.iter(|| using_lazy_static::translate(black_box(TEST_SEQ)))
    });
}

fn bench_match(c: &mut Criterion) {
    c.bench_function("match statement", |b| {
        b.iter(|| using_match::translate(black_box(TEST_SEQ)))
    });
}

fn bench_phf_map(c: &mut Criterion) {
    c.bench_function("phf map", |b| {
        b.iter(|| using_phf_map::translate(black_box(TEST_SEQ)))
    });
}

criterion_group!(
    benches,
    bench_current,
    bench_lazy_static,
    bench_match,
    bench_phf_map
);
criterion_main!(benches);