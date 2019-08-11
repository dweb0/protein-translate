use protein_translate::translate;

#[test]
fn validate() {

    let test_data = include_bytes!("test_data.csv");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(&test_data[..]);
    
    for record in rdr.deserialize() {
        let (seq, peptide): (String, String) = record.unwrap();
        let my_peptide = translate(seq.as_bytes());
        assert_eq!(peptide, my_peptide);
    }
}
