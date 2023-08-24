#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut hand_value: usize = 0;

        for c in &self.cards {
            match c {
                Card::Two => hand_value += 2,
                Card::Three => hand_value += 3,
                Card::Four => hand_value += 4,
                Card::Five => hand_value += 5,
                Card::Six => hand_value += 6,
                Card::Seven => hand_value += 7,
                Card::Eight => hand_value += 8,
                Card::Nine => hand_value += 9,
                Card::Jack | Card::Queen | Card::King => hand_value += 10,
                Card::Ace => {
                    if hand_value + 11 > 21 {
                        hand_value += 1
                    } else {
                        hand_value += 11
                    }
                }
            }
        }
        hand_value
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

/*fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
}  */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hand() {
        let hand = Hand::new();

        assert_eq!(hand.value(), 0);
    }

    #[test]
    fn strong_hand() {
        let mut hand = Hand::new();
        hand.add(Card::Queen);
        hand.add(Card::Ace);

        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn risky_hand() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Queen);
        hand.add(Card::Ace);

        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn oops() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Seven);
        hand.add(Card::Five);

        assert!(hand.is_loosing_hand());
        assert_eq!(hand.value(), 22);
    }
}
