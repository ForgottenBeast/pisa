use failure::Error;

use crate::DECK_LENGTH;

pub struct Deck {
    keycards: (u8,u8),
    state: Vec<u8>,
}

impl Deck {
    pub fn new(passphrase: &str, keycards: (u8,u8)) -> Result<Deck,Error>{
        let mut state: Vec<u8> = vec![];
        for i in 0..DECK_LENGTH {
            state.push(i as u8);
        }

        if keycards.0 == keycards.1 {
            return Err(format_err!("keycards can't be the same!"));
        }
        if keycards.0 < 0 || keycards.0 > DECK_LENGTH - 2 || keycards.1 < 0 || keycards.1 > DECK_LENGTH - 2 {
            return Err(format_err!("keycards must be between 0 and 51 inclusive!"));
        }

        let mut deck = Deck {
            keycards,
            state,
        };

        key_deck(&mut deck.state,passphrase)?;
        Ok(deck)
    }

    pub fn get_key(&mut self) -> u8 {
        //push jokers
        push_card(&mut self.state,53,1);
        push_card(&mut self.state,54,1);

        triple_cut(&mut self.state,53,54);

        let count_pos = self.state[53];
        count_cut(&mut self.state, count_pos);

        push_card(&mut self.state, self.keycards.0,1);
        push_card(&mut self.state, self.keycards.1,2);

        triple_cut(&mut self.state, self.keycards.0, self.keycards.1);

        let count_pos = self.state[53];
        count_cut(&mut self.state, count_pos);

        self.state[(self.state[0] + 1) as usize]
    }

}

fn key_deck(deck: &mut Vec<u8>, passphrase: &str) -> Result<(),Error> {
    if passphrase == "" {
        return Err(format_err!("passphrase can't be empty!"));
    }
    unimplemented!();
}

fn triple_cut(deck: &mut Vec<u8>, first_card: u8, last_card: u8) {

}

//takes the first $count cards off the top and insert them just above the last card
fn count_cut(deck: &mut Vec<u8>, count: u8) -> Result<(),Error> {
    if count < 0 {
        return Err(format_err!("count must be > 0"));
    }
    if count as usize == deck.len() || count as usize == deck.len() - 1 {
        return Ok(());
    }

    let mut to_move: Vec<u8> = deck.drain(0..count as usize).collect();
    for item in to_move.drain(..) {
        deck.insert(deck.len() - 1,item);
    }
    Ok(())
}

fn push_card(deck: &mut Vec<u8>, card: u8, places: u8) -> Result<(),Error> {
    if !deck.contains(&card) {
        return Err(format_err!("card {} isn't in deck",card));
    }
    let starting_pos = deck.iter().position(|&s| s == card).unwrap() as u8;

    let target_index: u8 = if starting_pos + places > (deck.len() - 1) as u8 {
            (starting_pos + places) % deck.len() as u8
        }
        else {
            starting_pos + places
        };

    deck.remove(starting_pos as usize);
    deck.insert(target_index as usize, card);
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_push_card() {
        use crate::deck::push_card;

        let mut test: Vec<u8> = vec![0,1,2,3];
        let target: Vec<u8> = vec![1,0,2,3];
        push_card(&mut test,0,1);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3];
        let target: Vec<u8> = vec![0,2,1,3];
        push_card(&mut test,1,1);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3];
        let target: Vec<u8> = vec![1,2,0,3];
        push_card(&mut test,0,2);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3];
        let target: Vec<u8> = vec![3,0,1,2];
        push_card(&mut test,3,1);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3];
        let target: Vec<u8> = vec![0,2,3,1];
        push_card(&mut test,1,2);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3];
        let target: Vec<u8> = vec![1,0,2,3];
        push_card(&mut test,1,3);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3];
        let target: Vec<u8> = vec![0,1,2,3];
        push_card(&mut test,1,0);
        assert_eq!(test,target);

    }

    #[test]
    fn test_count_cut() {
        use crate::deck::count_cut;

        let mut test: Vec<u8> = vec![0,1,2,3,4,5];
        let target: Vec<u8> = vec![2,3,4,0,1,5];
        count_cut(&mut test,2);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3,4,5];
        let target: Vec<u8> = vec![0,1,2,3,4,5];
        count_cut(&mut test,5);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3,4,5];
        let target: Vec<u8> = vec![0,1,2,3,4,5];
        count_cut(&mut test,6);
        assert_eq!(test,target);

        let mut test: Vec<u8> = vec![0,1,2,3,4,5];
        let target: Vec<u8> = vec![4,0,1,2,3,5];
        count_cut(&mut test,4);
        assert_eq!(test,target);

    }
}

