pub struct Table { 
    pub attack_cards:  [Option<u8>; 6],
    pub defense_cards: [Option<u8>; 6],
}

impl Table {
    pub fn new() -> Table {
        let x: [Option<u8>; 6] = [None; 6];
        let y: [Option<u8>; 6] = [None; 6];
        
        Self {
            attack_cards: x,
            defense_cards: y,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.attack_cards.iter().all(|x| x.is_none()) &&
        self.defense_cards.iter().all(|x| x.is_none())
    }

    pub fn match_rank(&self, rank: u8) -> bool {
        for i in 0..6 {
            if let Some(card) = self.attack_cards[i] { if card / 4 == rank { return true; } }
        }

        for i in 0..6 {
            if let Some(card) = self.defense_cards[i] { if card / 4 == rank { return true; } }
        }
        return false
    }

    pub fn attack_not_full(&self) -> bool {
        self.attack_cards[5].is_none()
    }
}