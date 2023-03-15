use tournaments::SingleElimination;

#[test]
fn four_contestants() {
    let names: Vec<String> = vec![
        "John".to_string(),
        "Paul".to_string(),
        "Ringo".to_string(),
        "George".to_string(),
    ];
    let mut tournament = SingleElimination::new(names.as_slice()).unwrap();
    assert_eq!(tournament.current_matches().len(), 2);

    tournament.set_winner(&0, &"John".to_string()).unwrap();
    tournament.set_winner(&1, &"Ringo".to_string()).unwrap();
    let winner = tournament
        .set_winner(&2, &"Ringo".to_string())
        .unwrap()
        .unwrap();
    assert_eq!(winner, "Ringo".to_string())
}
