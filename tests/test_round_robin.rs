use tournaments::round_robin_tournament;

#[test]
fn three_contestants() {
    let names = vec![
        "John".to_string(),
        "Paul".to_string(),
        "Ringo".to_string(),
        "George".to_string(),
    ];

    let mut tournament = round_robin_tournament(&names).unwrap();
    assert_eq!(tournament.bracket().current_matches().len(), 6);

    // John v George
    tournament.set_winner(&0, "John".to_string()).unwrap();
    // Paul v Ringo
    tournament.set_winner(&1, "Paul".to_string()).unwrap();
    // John v Ringo
    tournament.set_winner(&2, "Ringo".to_string()).unwrap();
    // George v Paul
    tournament.set_winner(&3, "Paul".to_string()).unwrap();
    // John v Paul
    tournament.set_winner(&4, "Paul".to_string()).unwrap();
    // Ringo v George
    tournament.set_winner(&5, "George".to_string()).unwrap();

    assert!(tournament.is_finished());

    let rankings = tournament.rankings();
    let winner = rankings.first().unwrap().first().unwrap();

    assert_eq!(winner, &"Paul".to_string())
}
