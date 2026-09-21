use crate::deck::{Deck, DeckSize};

pub struct Player {
    pub hand: u64,
}

pub enum GameState {
    Init,
    Attack,
    Defense,
    ThrowIn,
}

pub struct Game<const MAX_PLAYERS: usize = 6 > {
    pub deck: Deck,
    pub players: [Player; MAX_PLAYERS],
    pub players_count: u8,
    pub attacker: u8,
    pub defender: u8,
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
            attacker: 0,
            defender: 0,
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

    pub fn turn_init(&mut self) {
        let mut lowest_trump: u8 = 255;
        let trump_suit = self.deck.trump_card % 4;
        let mut attacker_index: usize = 0;
        for i in 0..self.players_count as usize {
            for item in 0..52 {
                if (self.players[i].hand & (1u64 << item)) != 0 {
                    if (item % 4 == trump_suit) && (item / 4 < lowest_trump) {
                        lowest_trump = item / 4;
                        attacker_index = i;
                }
                }
            }
            
        }
        self.attacker = attacker_index as u8;
        self.defender = (attacker_index as u8 + 1) % self.players_count ;
    }
}