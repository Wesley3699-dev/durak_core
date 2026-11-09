use super::shuffle::shuffle_deck;

#[allow(dead_code)]
pub struct Deck {
    pub cards: [u8; 52],
    pub trump_card: u8,
    pub len: u8,
}

impl Deck {
    pub fn new(size: DeckSize) -> Deck {
        let deck_size = size as u8;
        let mut cards = [0u8; 52];
        let offset = 52 - deck_size;

        for i in 0..deck_size {
            cards[i as usize] = i + offset
        }
        shuffle_deck(&mut cards[..deck_size as usize]);
        
        let trump_card = cards[0];

        Self { 
            cards: cards, 
            trump_card, 
            len: (deck_size)
        }
        
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let card = self.cards[self.len as usize];
            Some(card)
        }
    }
}

pub enum DeckSize {
    TwentyFour = 24,
    ThirtySix = 36,
    FiftyTwo = 52
}