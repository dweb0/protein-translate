use protein_translate::translate;

fn main() {
    let dna = b"GTGAGTCGTTGAGTCTGATTGCGTATC";
    
    let protein = translate(dna);
    assert_eq!("VSR*V*LRI", &protein);

    // To shift reading frame
    let protein_frame2 = translate(&dna[1..]);
    assert_eq!("*VVESDCV", &protein_frame2);

}