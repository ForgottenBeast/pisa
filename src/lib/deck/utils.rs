use failure::Error;

pub(super) fn key_deck(deck: &mut Vec<u8>, passphrase: &str) -> Result<(), Error> {
    if passphrase == "" {
        return Err(format_err!("passphrase can't be empty!"));
    }
    unimplemented!();
}

pub(super) fn triple_cut(deck: &mut Vec<u8>, first_card: u8, last_card: u8) {
    let mut c1_pos = get_card_position(deck, first_card).unwrap();
    let mut c2_pos = get_card_position(deck, last_card).unwrap();

    if c1_pos > c2_pos {
        let tmp = c1_pos;
        c1_pos = c2_pos;
        c2_pos = tmp;
    }

    let mut middle: Vec<u8> = deck.split_off(c1_pos);
    let mut last_part: Vec<u8> = middle.split_off(c2_pos - c1_pos + 1);

    middle.reverse();
    for i in middle {
        deck.insert(0, i);
    }

    last_part.reverse();
    for i in last_part {
        deck.insert(0, i);
    }
}

//takes the first $count cards off the top and insert them just above the last card
pub(super) fn count_cut(deck: &mut Vec<u8>, count: u8) -> Result<(), Error> {
    if count < 0 {
        return Err(format_err!("count must be > 0"));
    }
    if count as usize == deck.len() || count as usize == deck.len() - 1 {
        return Ok(());
    }

    let mut to_move: Vec<u8> = deck.drain(0..count as usize).collect();
    for item in to_move.drain(..) {
        deck.insert(deck.len() - 1, item);
    }
    Ok(())
}

pub(super) fn push_card(deck: &mut Vec<u8>, card: u8, places: u8) -> Result<(), Error> {
    if !deck.contains(&card) {
        return Err(format_err!("card {} isn't in deck", card));
    }
    let starting_pos = get_card_position(deck, card).unwrap() as u8;

    let target_index: u8 = if starting_pos + places > (deck.len() - 1) as u8 {
        (starting_pos + places) % deck.len() as u8
    } else {
        starting_pos + places
    };

    deck.remove(starting_pos as usize);
    deck.insert(target_index as usize, card);
    Ok(())
}

pub(super) fn get_card_position(deck: &mut Vec<u8>, card: u8) -> Option<usize> {
    deck.iter().position(|&s| s == card)
}

#[cfg(test)]
mod tests {
    #[test]
    pub(super) fn test_push_card() {
        use super::push_card;

        let mut test: Vec<u8> = vec![0, 1, 2, 3];
        let target: Vec<u8> = vec![1, 0, 2, 3];
        push_card(&mut test, 0, 1);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3];
        let target: Vec<u8> = vec![0, 2, 1, 3];
        push_card(&mut test, 1, 1);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3];
        let target: Vec<u8> = vec![1, 2, 0, 3];
        push_card(&mut test, 0, 2);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3];
        let target: Vec<u8> = vec![3, 0, 1, 2];
        push_card(&mut test, 3, 1);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3];
        let target: Vec<u8> = vec![0, 2, 3, 1];
        push_card(&mut test, 1, 2);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3];
        let target: Vec<u8> = vec![1, 0, 2, 3];
        push_card(&mut test, 1, 3);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3];
        let target: Vec<u8> = vec![0, 1, 2, 3];
        push_card(&mut test, 1, 0);
        assert_eq!(test, target);
    }

    #[test]
    pub(super) fn test_count_cut() {
        use super::count_cut;

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let target: Vec<u8> = vec![2, 3, 4, 0, 1, 5];
        count_cut(&mut test, 2);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let target: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        count_cut(&mut test, 5);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let target: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        count_cut(&mut test, 6);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let target: Vec<u8> = vec![4, 0, 1, 2, 3, 5];
        count_cut(&mut test, 4);
        assert_eq!(test, target);
    }

    #[test]
    pub(super) fn test_triple_cut() {
        use super::triple_cut;

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let mut target: Vec<u8> = vec![4, 5, 2, 3, 0, 1];
        triple_cut(&mut test, 2, 3);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let mut target: Vec<u8> = vec![4, 5, 2, 3, 0, 1];
        triple_cut(&mut test, 3, 2);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let mut target: Vec<u8> = vec![1, 2, 3, 4, 5, 0];
        triple_cut(&mut test, 1, 5);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let mut target: Vec<u8> = vec![5, 0, 1, 2, 3, 4];
        triple_cut(&mut test, 0, 4);
        assert_eq!(test, target);

        let mut test: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let mut target: Vec<u8> = vec![5, 1, 2, 3, 4, 0];
        triple_cut(&mut test, 1, 4);
        assert_eq!(test, target);
    }
}
