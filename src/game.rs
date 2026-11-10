use crate::{
    deck::{Deck, DeckSize},
    game::GameState::{Playing, Setup},
    rules::{attacker_allowance, beats},
    table::Table,
};

pub struct Player {
    pub hand: u64,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Setup,
    Playing,
    GameOver,
}

pub struct Game<const MAX_PLAYERS: usize = 6> {
    pub deck: Deck,
    pub players: [Player; MAX_PLAYERS],
    pub players_count: u8,
    pub attacker: u8,
    pub defender: u8,
    pub discard: u64,
    pub table: Table,
    pub defender_hand_at_bout_start: u8,
    state: GameState,
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
            discard: 0,
            table: Table::new(),
            defender_hand_at_bout_start: 0,
            state: Setup,
        }
    }

    pub fn deal(&mut self) {
        for _ in 0..6 {
            for i in 0..self.players_count as usize {
                if let Some(card) = self.deck.pop() {
                    self.players[i].hand |= 1u64 << card;
                }
            }
        }
    }

    pub fn draw(&mut self, player: u8) {
        let hand = self.players[player as usize].hand;
        let hand_size = hand.count_ones();

        if hand_size < 6 {
            for _ in 0..6 - hand_size {
                if let Some(card) = self.deck.pop() {
                    self.players[player as usize].hand |= 1u64 << card;
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
        self.defender = (attacker_index as u8 + 1) % self.players_count;
        self.defender_hand_at_bout_start = 5; //five card first bout rule
        self.state = Playing;
    }

    pub fn attack(&mut self, card: u8) {
        if self.state != Playing {
            return;
        }

        let attacker_have_card = (self.players[self.attacker as usize].hand & (1u64 << card)) != 0;
        if !attacker_have_card {
            return;
        }

        let mut max_attack: u8 = 6;

        if self.defender_hand_at_bout_start < 6 {
            max_attack = self.defender_hand_at_bout_start
        }

        if !attacker_allowance(&self.table, card, max_attack) {
            return;
        }

        for i in 0..max_attack as usize {
            if self.table.attack_cards[i] == None {
                self.table.attack_cards[i] = Some(card);
                self.players[self.attacker as usize].hand &= !(1u64 << card);
                break;
            }
        }
    }

    pub fn defend(&mut self, attack_slot: u8, card: u8) {
        if self.state != Playing {
            return;
        }
        if !(attack_slot <= 5) {
            return;
        }
        if self.table.attack_cards[attack_slot as usize].is_none() {
            return;
        }
        if self.table.defense_cards[attack_slot as usize].is_some() {
            return;
        }
        let defender_have_card = (self.players[self.defender as usize].hand & (1u64 << card)) != 0;
        if !defender_have_card {
            return;
        }

        if let Some(attack_card) = self.table.attack_cards[attack_slot as usize] {
            if beats(attack_card, card, self.deck.trump_card % 4) {
                self.table.defense_cards[attack_slot as usize] = Some(card);
                self.players[self.defender as usize].hand &= !(1u64 << card)
            }
        }
    }

    pub fn take(&mut self) {
        if self.state != Playing {
            return;
        }
        if self.table.is_empty() {
            return;
        }
        for i in 0..6 {
            if let Some(card) = self.table.attack_cards[i] {
                self.players[self.defender as usize].hand |= 1u64 << card;
                self.table.attack_cards[i] = None
            }
            if let Some(card) = self.table.defense_cards[i] {
                self.players[self.defender as usize].hand |= 1u64 << card;
                self.table.defense_cards[i] = None
            }
        }

        self.end_bout(false);
    }

    pub fn pass(&mut self) {
        if self.state != Playing {
            return;
        }
        if self.table.is_empty() {
            return;
        }
        for i in 0..6 {
            if let Some(_card_attack) = self.table.attack_cards[i] {
                if self.table.defense_cards[i].is_none() {
                    return;
                }
            }
        }

        for i in 0..6 {
            if let Some(card) = self.table.attack_cards[i] {
                self.discard |= 1u64 << card;
                self.table.attack_cards[i] = None
            }
            if let Some(card) = self.table.defense_cards[i] {
                self.discard |= 1u64 << card;
                self.table.defense_cards[i] = None
            }
        }
        self.end_bout(true);
    }

    fn end_bout(&mut self, is_defended: bool) {
        for step in 0..self.players_count {
            let chair = (self.attacker + step) % self.players_count;
            if chair != self.defender {
                self.draw(chair);
            }
        }

        self.draw(self.defender);

        if !is_defended {
            self.attacker = self.find_active(self.defender);
            self.defender = self.find_active(self.attacker);
        } else {
            self.attacker = self.find_active(self.attacker);
            self.defender = self.find_active(self.attacker);
        }

        self.defender_hand_at_bout_start =
            self.players[self.defender as usize].hand.count_ones() as u8;

        let active_players = (0..self.players_count)
            .filter(|&x| self.is_active(x))
            .count();

        if active_players == 1 {
            self.state = GameState::GameOver;
        }
    }

    pub fn is_active(&self, chair: u8) -> bool {
        if self.deck.len != 0 {
            return true;
        }
        self.players[chair as usize].hand != 0
    }

    pub fn find_active(&self, chair: u8) -> u8 {
        let mut x = chair;
        for i in 1..self.players_count {
            let next_player = (chair + i) % self.players_count;
            if self.is_active(next_player) {
                x = next_player;
                break;
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use crate::{deck::DeckSize::TwentyFour, game::GameState::GameOver};

    use super::*;

    #[test]
    fn test_find_active() {
        let mut game = Game::<6>::new(3, TwentyFour);

        game.deck.len = 0;
        game.players[0].hand = 0;
        game.players[1].hand = 0;
        game.players[2].hand = 1;

        let x = game.find_active(0);
        assert_eq!(x, 2);
    }

    #[test]
    fn test_end_bout() {
        let mut game = Game::<6>::new(3, TwentyFour);

        game.deck.len = 0;
        game.players[0].hand = 0;
        game.players[1].hand = 0;
        game.players[2].hand = 1;
        game.attacker = 1;
        game.defender = 2;
        game.table.attack_cards[0] = Some(1);
        game.table.defense_cards[0] = Some(5);
        game.state = Playing;
        game.pass();

        assert_eq!(game.state, GameOver)
    }
}
