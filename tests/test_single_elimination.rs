use tournaments::single_elimination_tournament;

#[test]
fn four_contestants() {
    let names = vec![
        "John".to_string(),
        "Paul".to_string(),
        "Ringo".to_string(),
        "George".to_string(),
    ];

    let mut tournament = single_elimination_tournament(&names).unwrap();
    assert_eq!(tournament.bracket().current_matches().len(), 2);

    tournament.set_winner(&0, "John".to_string()).unwrap();
    tournament.set_winner(&1, "Ringo".to_string()).unwrap();
    tournament.set_winner(&2, "Ringo".to_string()).unwrap();

    assert!(tournament.is_finished());

    let rankings = tournament.rankings();
    let winner = rankings.first().unwrap().first().unwrap();

    assert_eq!(winner, &"Ringo".to_string())
}
