use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum DealerOnSoft17 {
    H17,
    S17,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum SurrenderKind {
    Early, // before dealer checks for blackjack
    Late, // after dealer checks for blackjack
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RuleSet {
    // table setup
    decks: usize,
    players: usize,
    min_bet: f64,
    max_bet: f64,

    // dealer rules
    dealer_on_soft_17: DealerOnSoft17,

    // blackjack payout
    blackjack_payout: f64,

    // doubling down
    double_down_whitelist: Option<Vec<u64>>, // can DD on listed values, else always

    // splitting
    max_hands: u64,
    can_play_slit_aces: bool,
    das: bool, // can DD after splitting

    // surrendering
    surrender: Option<SurrenderKind>
}

impl RuleSet {
    pub fn new(
        decks: usize,
        players: usize,
        min_bet: f64,
        max_bet: f64,
        dealer_on_soft_17: DealerOnSoft17,
        blackjack_payout: f64,
        double_down_whitelist: Option<Vec<u64>>,
        max_hands: u64,
        can_play_slit_aces: bool,
        das: bool,
        surrender: Option<SurrenderKind>,
    ) -> Result<Self, RuleSetError> {
        if decks == 0 {
            return Err(RuleSetError::InvalidDeckNumer);
        }

        if players == 0 {
            return Err(RuleSetError::InvalidPlayerNumber);
        }

        if min_bet == 0.0 {
            return Err(RuleSetError::InvalidBetRange);
        }
        
        if min_bet > max_bet {
            return Err(RuleSetError::InvalidBetRange);
        }

        if max_hands < 2 {
            return Err(RuleSetError::InvalidMaxHands);
        }

        if let Some(values) = double_down_whitelist.clone() {
            for val in values {
                if val < 3 || val > 20 {
                    return Err(RuleSetError::InvalidDoubleDownWhitelist);
                }
            }
        }

        Ok( Self {
            decks,
            players,
            min_bet,
            max_bet,
            dealer_on_soft_17,
            blackjack_payout,
            double_down_whitelist,
            max_hands,
            can_play_slit_aces,
            das,
            surrender,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum RuleSetError {
    InvalidDeckNumer,
    InvalidPlayerNumber,
    InvalidBetRange,
    InvalidMaxHands,
    InvalidDoubleDownWhitelist,
}

impl fmt::Display for RuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeckNumer => write!(f, "must have at least 1 deck"),
            Self::InvalidPlayerNumber => write!(f, "must have at least 1 player"),
            Self::InvalidBetRange => write!(f, "min bet must be at least 1 and not exceed max bet"),
            Self::InvalidMaxHands => write!(f, "must have at least 2 max hands"),
            Self::InvalidDoubleDownWhitelist => write!(f, "double down whitelist must contain some values from 3 to 20 or be `None`"),
        }
    }
}

impl Error for RuleSetError {}

#[cfg(test)]
mod tests {
    use crate::rule::{DealerOnSoft17, RuleSet, RuleSetError};

    #[test]
    fn create_rulesets() {
        assert!( RuleSet::new(
            4,
            4,
            1.0,
            1.0,
            DealerOnSoft17::H17,
            1.5,
            None,
            3,
            false,
            false,
            None,
        ).is_ok() );

        let invalid_deck_number = RuleSet::new(
            0,
            4,
            1.0,
            1.0,
            DealerOnSoft17::H17,
            1.5,
            None,
            3,
            false,
            false,
            None,
        );
        assert_eq!(Err(RuleSetError::InvalidDeckNumer), invalid_deck_number);

        let invalid_player_number = RuleSet::new(
            4,
            0,
            1.0,
            1.0,
            DealerOnSoft17::H17,
            1.5,
            None,
            3,
            false,
            false,
            None,
        );
        assert_eq!(Err(RuleSetError::InvalidPlayerNumber), invalid_player_number);

        let invalid_bet_range = RuleSet::new(
            4,
            4,
            2.0,
            1.0,
            DealerOnSoft17::H17,
            1.5,
            None,
            3,
            false,
            false,
            None,
        );
        assert_eq!(Err(RuleSetError::InvalidBetRange), invalid_bet_range);

        let invalid_max_hands = RuleSet::new(
            4,
            4,
            1.0,
            1.0,
            DealerOnSoft17::H17,
            1.5,
            None,
            1,
            false,
            false,
            None,
        );
        assert_eq!(Err(RuleSetError::InvalidMaxHands), invalid_max_hands);

        let invalid_double_down_whitelist = RuleSet::new(
            4,
            4,
            1.0,
            1.0,
            DealerOnSoft17::H17,
            1.5,
            Some(vec![9, 10, 11, 21]),
            3,
            false,
            false,
            None,
        );
        assert_eq!(Err(RuleSetError::InvalidDoubleDownWhitelist), invalid_double_down_whitelist);
    }
}
