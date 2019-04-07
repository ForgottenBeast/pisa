use failure::Error;

use crate::{DECK_LENGTH, JOKER_1, JOKER_2};

mod utils;
use utils::*;

enum EncodingState {
    Letters,
    Numbers,
}

pub struct Deck {
    keycards: (u8, u8),
    cards: Vec<u8>,
    state: EncodingState,
}

impl Deck {
    pub fn new(passphrase: &str, keycards: (u8, u8)) -> Result<Deck, Error> {
        let mut cards: Vec<u8> = vec![];
        for i in 0..DECK_LENGTH {
            cards.push(i as u8);
        }

        if keycards.0 == keycards.1 {
            return Err(format_err!("keycards can't be the same!"));
        }
        if keycards.0 > DECK_LENGTH - 2 || keycards.1 > DECK_LENGTH - 2 {
            return Err(format_err!("keycards must be between 0 and 51 inclusive!"));
        }

        let mut deck = Deck {
            keycards,
            cards,
            state: EncodingState::Letters,
        };

        key_deck(&mut deck.cards, passphrase, JOKER_1, JOKER_2)?;
        Ok(deck)
    }

    pub fn get_key(&mut self) -> Result<u8, Error> {
        //push jokers
        push_card(&mut self.cards, JOKER_1, 1)?;
        push_card(&mut self.cards, JOKER_2, 1)?;

        triple_cut(&mut self.cards, JOKER_1, JOKER_2)?;

        let count_pos = self.cards[53] + 1;
        count_cut(&mut self.cards, count_pos as usize)?;

        push_card(&mut self.cards, self.keycards.0, 1)?;
        push_card(&mut self.cards, self.keycards.1, 2)?;

        triple_cut(&mut self.cards, self.keycards.0, self.keycards.1)?;

        let count_pos = self.cards[53] + 1;
        count_cut(&mut self.cards, count_pos as usize)?;

        Ok(self.cards[(self.cards[0] + 1) as usize])
    }
}
