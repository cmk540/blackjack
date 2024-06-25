use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clubs => write!(f, "♣"),
            Self::Diamonds => write!(f, "♦"),
            Self::Hearts => write!(f, "♥"),
            Self::Spades => write!(f, "♠"),
        }
    }
}

impl TryFrom<u8> for Suit {
    type Error = CardError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Spades),
            _ => Err(CardError::ParseSuitError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ace => write!(f, "A"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "T"),
            Self::Jack => write!(f, "J"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = CardError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::Ace),
            1 => Ok(Rank::Two),
            2 => Ok(Rank::Three),
            3 => Ok(Rank::Four),
            4 => Ok(Rank::Five),
            5 => Ok(Rank::Six),
            6 => Ok(Rank::Seven),
            7 => Ok(Rank::Eight),
            8 => Ok(Rank::Nine),
            9 => Ok(Rank::Ten),
            10 => Ok(Rank::Jack),
            11 => Ok(Rank::Queen),
            12 => Ok(Rank::King),
            _ => Err(CardError::ParseRankError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suit(), self.rank())
    }
}

impl TryFrom<u8> for Card {
    type Error = CardError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (value & 0b1100_0000) != 0 {
            return Err(CardError::ParseCardError);
        }
        
        let suit: Suit = (value & 0b0000_0011).try_into()?;
        let rank: Rank = ((value & 0b0011_1100) >> 2).try_into()?;

        Ok( Self { suit, rank })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum CardError {
    ParseSuitError,
    ParseRankError,
    ParseCardError,
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseSuitError => write!(f, "failed to parse to Suit"),
            Self::ParseRankError => write!(f, "failed to parse to Rank"),
            Self::ParseCardError => write!(f, "failed to parse to Card"),
        }
    }
}

impl Error for CardError {}

#[cfg(test)]
mod tests {
    use crate::card::{Card, CardError, Rank, Suit};
    
    #[test]
    fn parse_suit_from_u8() {
        assert_eq!(Ok(Suit::Clubs),     Suit::try_from(0u8));
        assert_eq!(Ok(Suit::Diamonds),  Suit::try_from(1u8));
        assert_eq!(Ok(Suit::Hearts),    Suit::try_from(2u8));
        assert_eq!(Ok(Suit::Spades),    Suit::try_from(3u8));

        assert_eq!(Err(CardError::ParseSuitError), Suit::try_from(4u8));
    }

    #[test]
    fn parse_rank_from_u8() {
        assert_eq!(Ok(Rank::Ace),   Rank::try_from(0u8));
        assert_eq!(Ok(Rank::Two),   Rank::try_from(1u8));
        assert_eq!(Ok(Rank::Three), Rank::try_from(2u8));
        assert_eq!(Ok(Rank::Four),  Rank::try_from(3u8));
        assert_eq!(Ok(Rank::Five),  Rank::try_from(4u8));
        assert_eq!(Ok(Rank::Six),   Rank::try_from(5u8));
        assert_eq!(Ok(Rank::Seven), Rank::try_from(6u8));
        assert_eq!(Ok(Rank::Eight), Rank::try_from(7u8));
        assert_eq!(Ok(Rank::Nine),  Rank::try_from(8u8));
        assert_eq!(Ok(Rank::Ten),   Rank::try_from(9u8));
        assert_eq!(Ok(Rank::Jack),  Rank::try_from(10u8));
        assert_eq!(Ok(Rank::Queen), Rank::try_from(11u8));
        assert_eq!(Ok(Rank::King),  Rank::try_from(12u8));

        assert_eq!(Err(CardError::ParseRankError), Rank::try_from(13u8))
    }

    #[test]
    fn basic_card_methods() {
        let ace_of_spades: Card = Card::new(Suit::Spades, Rank::Ace);

        assert_eq!(Suit::Spades, ace_of_spades.suit());
        assert_eq!(Rank::Ace, ace_of_spades.rank());
    }

    #[test]
    fn parse_card_from_u8() {
        let king_of_spades: Card = Card::new(Suit::Spades, Rank::King);
        assert_eq!(Ok(king_of_spades), Card::try_from(0b0011_0011));

        // impossible to obtain `CardError::ParseSuitError` when converting `u8` to `Card`

        assert_eq!(Err(CardError::ParseRankError), Card::try_from(0b0011_0100));

        assert_eq!(Err(CardError::ParseCardError), Card::try_from(0b1100_0000));
    }
}
