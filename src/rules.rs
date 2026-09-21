pub fn beats(attack_card: u8, defense_card: u8, trump_suit: u8) -> bool {
    let attack_suit = attack_card % 4;  
    let defense_suit = defense_card % 4;
    let attack_rank = attack_card / 4;
    let defense_rank = defense_card / 4;

    (attack_suit == defense_suit && defense_rank > attack_rank)
        || (attack_suit != trump_suit && defense_suit == trump_suit)
}