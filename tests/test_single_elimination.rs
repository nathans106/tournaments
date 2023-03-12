use tournaments::SingleElimination;

#[test]
fn four_contestants() {
    let names: Vec<String> = vec![
        "John".to_string(),
        "Paul".to_string(),
        "Ringo".to_string(),
        "George".to_string(),
    ];
    let tournament = SingleElimination::new(names.as_slice()).unwrap();
    assert_eq!(tournament.current_matches().len(), 2);
}
