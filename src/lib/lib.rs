#[macro_use]
extern crate failure;

mod deck;
pub use deck::Deck;

const DECK_LENGTH: u8 = 54;
const JOKER_1: u8 = 53;
const JOKER_2: u8 = 54;
