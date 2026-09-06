use super::shuffle::shuffle_deck;

#[allow(dead_code)]
pub struct Deck {
    cards: [u8; 52],
    trump_card: u8,
    len: u8,
}

impl Deck {
    pub fn new(size: u8, ) -> Self {
        let mut cards = [0u8; 52];
        let offset = 52 - size;

        for i in 0..size {
            cards[i as usize] = i + offset
        }
        shuffle_deck(&mut cards[..size as usize]);
        
        let trump_card = cards[0];

        Self { 
            cards: cards, 
            trump_card, 
            len: (size)
        }
    }


}