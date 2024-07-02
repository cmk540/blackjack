use crate::{card::{Card, Rank}, rule::RuleSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Hand<S: HandState> {
    stack: Vec<Card>,
    marker: std::marker::PhantomData<S>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandValue {
    Hard(u64),
    Soft {
        lower: u64,
        upper: u64,
    }
}

impl<S> Hand<S>
    where S: HandState
{
    pub fn value(&self) -> HandValue {
        let aces = self.stack.iter().filter(|&c| c.rank() == Rank::Ace).count();

        let mut value: u64 = 0;

        if aces == 0 {
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

        return HandValue::Soft { lower: value, upper: value + 10 };
        }
    }
}

impl IsTerminal for Hand<Fresh> {
    fn is_terminal() -> bool {
        false
    }
}

// impl HandInfo for Hand<Fresh> {
//     fn is_21(&self) -> bool {
//         match self.value() {
//             HandValue::Hard(v) => {
//                 if v == 21 {
//                     return true;
//                 }

//                 false
//             },
//             HandValue::Soft { lower, upper } => {
//                 if lower == 21 || upper == 21 {
//                     return true;
//                 }

//                 false
//             },
//         }
//     }

//     fn is_natural(&self) -> bool {
//         if self.is_21() {
//             return true;
//         }

//         false
//     }

//     fn is_bust(&self) -> bool {
//         match self.value() {
//             HandValue::Hard(v) => {
//                 if v > 21 {
//                     return true;
//                 }

//                 false
//             },
//             HandValue::Soft { lower, upper } => {
//                 if lower > 21 {
//                     return true;
//                 }

//                 false
//             },
//         }
//     }

//     fn is_pair(&self) -> bool {
//         if self.stack.len() == 2 {
//             if self.stack[0].rank() == self.stack[0].rank() {
//                 return true;
//             }
//         }

//         false
//     }

//     fn can_hit(&self, rules: RuleSet) -> bool {
//         if !self.is_bust() {
//             return true;
//         }

//         false
//     }
// }

impl Hand<Fresh> {
    pub fn new(stack: Vec<Card>) -> Self {
        Self {
            stack,
            marker: std::marker::PhantomData::<Fresh>,
        }
    }
}

impl IsTerminal for Hand<Bust> {
    fn is_terminal() -> bool {
        true
    }
}

impl IsTerminal for Hand<DoubleDown> {
    fn is_terminal() -> bool {
        true
    }
}

impl IsTerminal for Hand<Split> {
    fn is_terminal() -> bool {
        false
    }
}

impl IsTerminal for Hand<SplitAcesLocked> {
    fn is_terminal() -> bool {
        true
    }
}

impl IsTerminal for Hand<Stand> {
    fn is_terminal() -> bool {
        true
    }
}

impl IsTerminal for Hand<Surrender> {
    fn is_terminal() -> bool {
        true
    }
}

pub struct Fresh;
pub struct Bust;
pub struct DoubleDown;
pub struct Split;
pub struct SplitAcesLocked;
pub struct Stand;
pub struct Surrender;

pub trait HandState {}
impl HandState for Fresh {}
impl HandState for Bust {}
impl HandState for DoubleDown {}
impl HandState for Split {}
impl HandState for SplitAcesLocked {}
impl HandState for Stand {}
impl HandState for Surrender {}

pub trait IsTerminal {
    fn is_terminal() -> bool;
}

pub trait HandInfo {
    fn is_21(&self) -> bool;

    fn is_natural(&self) -> bool;

    fn is_bust(&self) -> bool;

    fn is_pair(&self) -> bool;

    fn can_hit(&self, rules: RuleSet) -> bool;

    fn can_split(&self, rules: RuleSet) -> bool;
    
    fn can_surrender(&self, rules: RuleSet) -> bool;

    fn can_double_down(&self, rules: RuleSet) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::{card::{Card, Rank, Suit}, hand::{Hand, HandValue}};

    #[test]
    fn value_of_hands() {
        let hard_hand = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::Two),
                Card::new(Suit::Clubs, Rank::Queen),
            ]
        );

        assert_eq!(HandValue::Hard(12), hard_hand.value());

        let soft_hand = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::King),
                Card::new(Suit::Clubs, Rank::Ace),
            ]
        );

        assert_eq!(HandValue::Soft { lower: 11, upper: 21 }, soft_hand.value());

        let soft_hand2 = Hand::new(
            vec![
                Card::new(Suit::Clubs, Rank::Ace),
                Card::new(Suit::Hearts, Rank::Ace),
            ]
        );

        assert_eq!(HandValue::Soft { lower: 2, upper: 12 }, soft_hand2.value());
    }
}
