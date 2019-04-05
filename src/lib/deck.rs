use failure::{Error,err_msg};

pub struct Deck {
    keycards: (u8,u8),
    state: Vec<u8>,
}

impl Deck {
    pub fn new(passphrase: &str, keycards: (u8,u8)) -> Result<Deck,Error>{
        let mut state: Vec<u8> = vec![];
        for i in 0..53 {
            state.push(i as u8);
        }

        if keycards.0 == keycards.1 {
            return Err(format_err!("keycards can't be the same!"));
        }
        if keycards.0 < 0 || keycards.0 > 51 || keycards.1 < 0 || keycards.1 > 51 {
            return Err(format_err!("keycards must be between 0 and 51 inclusive!"));
        }

        let mut deck = Deck {
            keycards,
            state,
        };

        deck.key_deck(passphrase)?;
        Ok(deck)
    }

    pub fn get_key(&mut self) -> u8 {
        //push jokers
        self.push_card(53,1);
        self.push_card(54,2);

        self.triple_cut(53,54);
        self.count_cut(self.state[53]);

        self.push_card(self.keycards.0,1);
        self.push_card(self.keycards.1,2);

        self.triple_cut(self.keycards.0, self.keycards.1);
        self.count_cut(self.state[53]);

        self.state[self.state[0] + 1]
    }

    fn key_deck(&mut self, passphrase: &str) -> Result<(),Error> {
        if passphrase == "" {
            return Err(format_err!("passphrase can't be empty!"));
        }
    }

    fn push_card(&mut self, card: u8, places: u8) {
    
    }

    fn triple_cut(&mut self, first_card: u8, last_card: u8) {

    }

    fn count_cut(&mut self, count: u8) {

    }
}
