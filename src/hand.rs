use crate::{card::{Card, Rank}, rule::RuleSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandValue {
    Hard(u64),
    Soft {
        lower: u64,
        upper: u64,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandState {
    Fresh,
    Split,
    SpltA,
    Stand,
    Busts,
    DDown,
    Srndr,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    bet: f64,
    state: HandState,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bet: f64, state: HandState) -> Self {
        Self {
            cards,
            bet,
            state,
        }
    }

    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn bet(&self) -> f64 {
        self.bet
    }

    pub fn state(&self) -> HandState {
        self.state
    }

    pub fn is_terminal(&self) -> bool {
        match self.state {
            HandState::Fresh => false,
            HandState::Split => false,
            HandState::SpltA => true,
            HandState::Stand => true,
            HandState::Busts => true,
            HandState::DDown => true,
            HandState::Srndr => true,
        }
    }

    pub fn is_hard(&self) -> bool {
        for c in &self.cards {
            if c.rank() == Rank::Ace {
                return false;
            }
        }

        true
    }

    pub fn value(&self) -> HandValue {
        let mut value: u64 = 0;

        if self.is_hard() {
            for c in &self.cards {
                match c.rank() {
                    Rank::Ace => panic!("ace found in hard hand"),
                    Rank::Two => value += 2,
                    Rank::Three => value += 3,
                    Rank::Four => value += 4,
                    Rank::Five => value += 5,
                    Rank::Six => value += 6,
                    Rank::Seven => value += 7,
                    Rank::Eight => value += 8,
                    Rank::Nine => value += 9,
                    Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => value += 10,
                }
            }

            return HandValue::Hard(value);
        } else {
            for c in &self.cards {
                match c.rank() {
                    Rank::Ace => value += 1,
                    Rank::Two => value += 2,
                    Rank::Three => value += 3,
                    Rank::Four => value += 4,
                    Rank::Five => value += 5,
                    Rank::Six => value += 6,
                    Rank::Seven => value += 7,
                    Rank::Eight => value += 8,
                    Rank::Nine => value += 9,
                    Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => value += 10,
                }
            }

            return HandValue::Soft { lower: value, upper: value + 10 };
        }
    }

    pub fn is_bust(&self) -> bool {
        match self.value() {
            HandValue::Hard(v) => {
                if v > 21 {
                    return true;
                }

                false
            },
            HandValue::Soft { lower, .. } => {
                if lower > 21 {
                    return true;
                }

                false
            },
        }
    }

    pub fn can_hit(&self) -> bool {
        if self.is_terminal() {
            return false;
        }

        true
    }

    pub fn hit(&mut self, added_card: Card) {
        assert!(self.can_hit(), "tried to hit when not allowed");

        self.cards.push(added_card);

        if self.is_bust() {
            self.state = HandState::Busts;
        }
    }

    pub fn can_stand(&self) -> bool {
        if self.is_terminal() {
            return false;
        }

        true
    }

    pub fn stand(&mut self) {
        assert!(self.can_hit(), "tried to stand when not allowed");

        self.state = HandState::Stand;
    }

    pub fn can_double_down(&self, rules: RuleSet) -> bool {
        if self.is_terminal() {
            return false;
        }

        if !rules.can_dd_after_split() && self.state() == HandState::Split {
            return false;
        }

        let hand_values: Vec<u64> = {
            match self.value() {
                HandValue::Hard(n) => vec![n],
                HandValue::Soft { lower, upper } => vec![lower, upper],
            }
        };

        for hand_value in hand_values {
            if rules.double_down_whitelist().contains(&hand_value) {
                return true;
            }
        }

        false
    }

    pub fn double_down(&mut self) {
        assert!(!self.is_terminal(), "tried to double down on terminal hand");
        
        self.bet = self.bet() * 2.0;
        self.state = HandState::DDown;
    }

    pub fn split(&mut self, card1: Card, card2: Card, bet: f64, lock_aces: bool) -> Self {
        let cards2 = vec![
            self.cards.pop().unwrap(),
            card2,
        ];

        self.cards.push(card1);

        if lock_aces {
            self.state = HandState::SpltA;
            return Self {
                cards: cards2,
                bet,
                state: HandState::SpltA,
            };
        } else {
            self.state = HandState::Split;
            return Self {
                cards: cards2,
                bet,
                state: HandState::Split,
            };
        }
    }

    pub fn can_surrender(&self, rules: RuleSet) -> bool {
        if self.is_terminal() {
            return false;
        }

        if rules.can_surrender() {
            return true;
        } else {
            return false;
        }
    }

    pub fn surrender(&mut self) {
        self.state = HandState::Srndr;
        self.bet = self.bet / 2.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::{card::{Card, Rank, Suit}, hand::{Hand, HandState, HandValue}, rule::{DealerOnSoft17, RuleSet, ShuffleKind}};
    
    #[test]
    fn hitting() {
        // hard hand
        let mut hand = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::Ten),
                Card::new(Suit::Clubs, Rank::Two),
            ],
            1.0,
            HandState::Fresh,
        );

        assert_eq!(HandValue::Hard(12), hand.value());
        assert!(hand.can_hit());

        hand.hit(Card::new(Suit::Hearts, Rank::Ten));

        assert_eq!(HandState::Busts, hand.state());
        assert!(hand.is_terminal());
        assert!(!hand.can_hit());

        assert_eq!(HandValue::Hard(22), hand.value());

        // soft hand
        let mut hand_s = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::Two),
                Card::new(Suit::Clubs, Rank::Ace),
            ],
            1.0,
            HandState::Fresh,
        );

        assert_eq!(HandValue::Soft { lower: 3, upper: 13 }, hand_s.value());
        assert!(hand_s.can_hit());

        hand_s.hit(Card::new(Suit::Clubs, Rank::Ten));

        assert_eq!(HandValue::Soft { lower: 13, upper: 23 }, hand_s.value());
        assert!(hand_s.can_hit());

        hand_s.hit(Card::new(Suit::Clubs, Rank::Ten));

        assert_eq!(HandState::Busts, hand.state());
        assert!(hand_s.is_terminal());
        assert!(!hand_s.can_hit());

        assert_eq!(HandValue::Soft { lower: 23, upper: 33 }, hand_s.value());
    }

    #[test]
    fn standing() {
        let mut hand = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::Ten),
                Card::new(Suit::Clubs, Rank::Two),
            ],
            1.0,
            HandState::Fresh,
        );

        hand.hit(Card::new(Suit::Hearts, Rank::Six));
        assert_eq!(HandValue::Hard(18), hand.value());

        hand.stand();

        assert_eq!(HandState::Stand, hand.state());
        assert!(hand.is_terminal());
        assert!(!hand.can_hit());
        assert!(!hand.can_stand());
    }

    #[test]
    fn d_downing() {
        let rules_no_can_dd_after_split: RuleSet = RuleSet::new(
            4,
            4,
            1.0,
            1.0,
            ShuffleKind::Continuous,
            DealerOnSoft17::H17,
            1.5,
            vec![9, 10, 11],
            3,
            false,
            false,
            false,
        ).unwrap();

        let mut hand = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::Five),
                Card::new(Suit::Clubs, Rank::Six),
            ],
            1.0,
            HandState::Fresh,
        );

        assert!(hand.can_double_down(rules_no_can_dd_after_split));

        hand.double_down();

        assert_eq!(HandState::DDown, hand.state());
        assert!(hand.is_terminal());
        assert_eq!(2.0, hand.bet());
    }

    #[test]
    fn splitting() {
        let mut hand1 = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::Six),
                Card::new(Suit::Clubs, Rank::Six),
            ],
            1.0,
            HandState::Fresh,
        );

        let hand2 = hand1.split(
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Clubs, Rank::Two),
            1.0,
            false
        );

        assert_eq!(hand1, hand2);
        assert_eq!(HandState::Split, hand1.state());
    }

    #[test]
    fn surrendering() {
        let mut hand = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::Six),
                Card::new(Suit::Clubs, Rank::Six),
            ],
            1.0,
            HandState::Fresh,
        );

        hand.surrender();

        assert_eq!(HandState::Srndr, hand.state());
        assert_eq!(0.5, hand.bet());
    }

}
