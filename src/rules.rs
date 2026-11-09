use crate::table::Table;

pub fn beats(attack_card: u8, defense_card: u8, trump_suit: u8) -> bool {
    let attack_suit = attack_card % 4;
    let defense_suit = defense_card % 4;
    let attack_rank = attack_card / 4;
    let defense_rank = defense_card / 4;

    (attack_suit == defense_suit && defense_rank > attack_rank)
        || (attack_suit != trump_suit && defense_suit == trump_suit)
}

pub fn attacker_allowance(table: &Table, card: u8, max_attack: u8) -> bool {
    if table.attack_cards.iter().filter(|c| c.is_some()).count() < max_attack as usize
        && (table.is_empty() || table.match_rank(card / 4))
    {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn not_beats_different_suit_not_trump() {
        let x = beats(1, 8, 2);
        assert!(!x);
    }

    #[test]
    fn beats_higher_rank() {
        let x = beats(0, 4, 3);
        assert!(x);
    }

    #[test]
    fn not_beats_higher_rank() {
        let x = beats(4, 0, 3);
        assert!(!x);
    }
    #[test]
    fn beats_lower_trump() {
        let x = beats(0, 4, 0);
        assert!(x);
    }
}
