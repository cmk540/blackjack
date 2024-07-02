use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum DealerOnSoft17 {
    H17,
    S17,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum ShuffleKind {
    Continuous,
    Threshold(u64),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RuleSet {
    // table setup
    decks: usize,
    players: usize,
    min_bet: f64,
    max_bet: f64,
    shuffle_kind: ShuffleKind,

    // dealer rules
    dealer_on_soft_17: DealerOnSoft17,

    // blackjack payout
    blackjack_payout: f64,

    // doubling down
    double_down_whitelist: Vec<u64>,

    // splitting
    max_hands: u64,
    can_play_slit_aces: bool,
    das: bool, // can DD after splitting

    // surrendering (always late (after dealer checks for bj))
    can_surrender: bool,
}

impl RuleSet {
    pub fn new(
        decks: usize,
        players: usize,
        min_bet: f64,
        max_bet: f64,
        shuffle_kind: ShuffleKind,
        dealer_on_soft_17: DealerOnSoft17,
        blackjack_payout: f64,
        double_down_whitelist: Vec<u64>,
        max_hands: u64,
        can_play_slit_aces: bool,
        das: bool,
        can_surrender: bool,
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

        for val in double_down_whitelist.clone() {
            if val < 3 || val > 20 {
                return Err(RuleSetError::InvalidDoubleDownWhitelist);
            }
        }

        Ok( Self {
            decks,
            players,
            min_bet,
            max_bet,
            shuffle_kind,
            dealer_on_soft_17,
            blackjack_payout,
            double_down_whitelist,
            max_hands,
            can_play_slit_aces,
            das,
            can_surrender,
        })
    }

    pub fn decks(&self) -> usize {
        self.decks
    }

    pub fn players(&self) -> usize {
        self.players
    }

    pub fn min_bet(&self) -> f64 {
        self.min_bet
    }

    pub fn max_bet(&self) -> f64 {
        self.max_bet
    }

    pub fn shuffle_kind(&self) -> ShuffleKind {
        self.shuffle_kind
    }

    pub fn dealer_on_soft_17(&self) -> DealerOnSoft17 {
        self.dealer_on_soft_17
    }

    pub fn blackjack_payout(&self) -> f64 {
        self.blackjack_payout
    }

    pub fn double_down_whitelist(&self) -> Vec<u64> {
        self.double_down_whitelist.clone()
    }

    pub fn max_hands(&self) -> u64 {
        self.max_hands
    }

    pub fn can_play_slit_aces(&self) -> bool {
        self.can_play_slit_aces
    }

    pub fn das(&self) -> bool {
        self.das
    }

    pub fn can_surrender(&self) -> bool {
        self.can_surrender
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
            Self::InvalidDoubleDownWhitelist => write!(f, "double down whitelist must contain some values from 3 to 20"),
        }
    }
}

impl Error for RuleSetError {}

#[cfg(test)]
mod tests {
    use crate::rule::{DealerOnSoft17, RuleSet, RuleSetError, ShuffleKind};

    #[test]
    fn create_rulesets() {
        assert!( RuleSet::new(
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
        ).is_ok() );

        let invalid_deck_number = RuleSet::new(
            0,
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
        );
        assert_eq!(Err(RuleSetError::InvalidDeckNumer), invalid_deck_number);

        let invalid_player_number = RuleSet::new(
            4,
            0,
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
        );
        assert_eq!(Err(RuleSetError::InvalidPlayerNumber), invalid_player_number);

        let invalid_bet_range = RuleSet::new(
            4,
            4,
            2.0,
            1.0,
            ShuffleKind::Continuous,
            DealerOnSoft17::H17,
            1.5,
            vec![9, 10, 11],
            3,
            false,
            false,
            false,
        );
        assert_eq!(Err(RuleSetError::InvalidBetRange), invalid_bet_range);

        let invalid_max_hands = RuleSet::new(
            4,
            4,
            1.0,
            1.0,
            ShuffleKind::Continuous,
            DealerOnSoft17::H17,
            1.5,
            vec![9, 10, 11],
            1,
            false,
            false,
            false,
        );
        assert_eq!(Err(RuleSetError::InvalidMaxHands), invalid_max_hands);

        let invalid_double_down_whitelist = RuleSet::new(
            4,
            4,
            1.0,
            1.0,
            ShuffleKind::Continuous,
            DealerOnSoft17::H17,
            1.5,
            vec![9, 10, 11, 21],
            3,
            false,
            false,
            false,
        );
        assert_eq!(Err(RuleSetError::InvalidDoubleDownWhitelist), invalid_double_down_whitelist);
    }
}
