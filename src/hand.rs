use crate::card::{Card, Rank};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandState {
    Fresh,
    FreshSplit,
    SplitAcesOpen,
    SplitAcesLocked,
    DoubledDown,
    Stood,
    Surrendered,
    Busted,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandValue {
    Hard(u64),
    Soft {
        lower: u64,
        upper: u64,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Hand {
    stack: Vec<Card>,
    state: HandState,
}

impl Hand {
    pub fn stack(&self) -> Vec<Card> {
        self.stack.clone()
    }

    pub fn state(&self) -> HandState {
        self.state
    }
    
    pub fn new(stack: [Card; 2]) -> Self {
        Self {
            stack: stack.to_vec(),
            state: HandState::Fresh
        }
    }

    pub fn is_terminal(&self) -> bool {
        match self.state {
            HandState::Fresh => false,
            HandState::FreshSplit => false,
            HandState::SplitAcesOpen => false,
            HandState::SplitAcesLocked => true,
            HandState::DoubledDown => true,
            HandState::Stood => true,
            HandState::Surrendered => true,
            HandState::Busted => true,
        }
    }

    fn has_ace(&self) -> bool {
        for card in &self.stack {
            if card.rank() == Rank::Ace {
                return true;
            }
        }

        false
    }

    pub fn value(&self) -> HandValue {
        let mut value: u64 = 0;

        if !self.has_ace() {
            for c in &self.stack {
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
            for c in &self.stack {
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
        }

        return HandValue::Soft { lower: value, upper: value + 10 };
    }

    pub fn is_21(&self) -> bool {
        match self.value() {
            HandValue::Hard(v) => {
                if v == 21 {
                    return true;
                }

                false
            },
            HandValue::Soft { lower, upper } => {
                if lower == 21 || upper == 21 {
                    return true;
                }

                false
            },
        }
    }

    fn is_pair(&self) -> bool {
        if self.stack.len() == 2 {
            if self.stack[0].rank() == self.stack[1].rank() {
                return true;
            }
        }

        false
    }

    pub fn is_natural(&self) -> bool {
        if self.is_21() && self.is_pair() {
            return true;
        }

        false
    }

    pub fn is_bust(&self) -> bool {
        match self.value() {
            HandValue::Hard(v) => {
                if v > 21 {
                    return true;
                }
            },
            HandValue::Soft { lower, .. } => {
                if lower > 21 {
                    return true;
                }
            }
        }

        false
    }

    // pub fn can_hit(&self, rule_set: RuleSet) -> bool {
    //     match self.state {
    //         HandState::Fresh => {
    //             if self.is_bust() ||
    //         }
    //     }
    // }
}
