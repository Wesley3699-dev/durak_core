use crate::deck::{Deck, DeckSize};

pub struct Player {
    pub hand: u64,
}

pub struct Game<const MAX_PLAYERS: usize = 6 > {
    pub deck: Deck,
    pub players: [Player; MAX_PLAYERS],
    pub players_count: u8,
}

impl Player {
    pub fn new() -> Self {
        Self { hand: 0 }
    }
}

impl<const MAX_PLAYERS: usize> Game<MAX_PLAYERS> {
    pub fn new(players_count: u8, size: DeckSize) -> Self {
        assert!(players_count as usize <= MAX_PLAYERS);

        Self {
            deck: Deck::new(size),
            players: std::array::from_fn(|_| Player::new()),
            players_count,
        }
    }

    pub fn deal(&mut self) {
        for _ in 0..6 {
            for i in 0..self.players_count as usize {
                if let Some(card) = self.deck.pop(){
                    self.players[i].hand |= 1u64 << card;
                }
            }
        }
    }
}