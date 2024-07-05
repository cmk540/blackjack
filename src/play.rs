// bot will have to remember to stand on blackjack

use crate::{card::{Card, Rank}, hand::{Hand, HandState, HandValue}, rule::RuleSet};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Action {
    Hit,
    Stand,
    Split{ second_bet: f64 },
    DoubleDown{ added_bet: f64 },
    Surrender,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Player {
    hands: Vec<Hand>,
    funds: f64,
    rules: RuleSet,
    ai: fn(&Hand) -> Action,
}

impl Player {
    pub fn new(hands: Vec<Hand>, funds: f64, rules: RuleSet, ai: fn(&Hand) -> Action) -> Self {
        Self { hands, funds, rules, ai }
    }

    pub fn hands(&self) -> Vec<Hand> {
        self.hands.clone()
    }

    pub fn funds(&self) -> f64 {
        self.funds
    }

    pub fn rules(&self) -> RuleSet {
        self.rules.clone()
    }

    pub fn ai(&self) -> fn(&Hand) -> Action {
        self.ai
    }

    pub fn can_hit_hand(hand: &Hand) -> bool {
        if hand.is_terminal() {
            return false;
        }
        
        true
    }

    pub fn hit(hand: Hand, added_card: Card) -> Hand {
        assert!(Self::can_hit_hand(&hand));

        let mut cards: Vec<Card> = hand.cards();
        cards.push(added_card);

        Hand::new(
            cards,
            hand.bet(),
            hand.state(),
        )
    }

    pub fn can_stand_hand(hand: &Hand) -> bool {
        if hand.is_terminal() {
            return false;
        }

        true
    }

    pub fn stand(hand: Hand) -> Hand {
        assert!(Self::can_hit_hand(&hand));

        Hand::new(
            hand.cards(),
            hand.bet(),
            HandState::Stand,
        )
    }

    pub fn can_split_hand(&self, hand: &Hand) -> bool {
        let cards: Vec<Card> = hand.cards();

        if cards.len() != 2 {
            return false;
        }

        if cards[0].rank() != cards[1].rank() {
            return false;
        }

        if self.hands.len() >= self.rules.max_hands().try_into().unwrap() {
            return false;
        }

        true
    }

    pub fn split(&self, hand: Hand, second_bet: f64, card1: Card, card2: Card) -> [Hand; 2] {
        assert!(Self::can_split_hand(&self, &hand));
        assert_eq!(hand.bet(), second_bet);

        let divided_cards: Vec<Card> = hand.cards();

        let mut state: HandState = HandState::Split;
        if divided_cards[0].rank() == Rank::Ace && !self.rules.can_play_slit_aces() {
            state = HandState::SpltA;
        }

        let mut cards1: Vec<Card> = vec![ divided_cards[0] ];
        cards1.push(card1);

        let mut cards2: Vec<Card> = vec![ divided_cards[1] ];
        cards2.push(card2);

        [
            Hand::new(cards1, hand.bet(), state),
            Hand::new(cards2, hand.bet(), state),
        ]

    }

    pub fn can_surrender_hand(&self, hand: &Hand) -> bool {
        if hand.is_terminal() {
            return false;
        }

        if !self.rules.can_surrender() {
            return false;
        }

        true
    }

    pub fn surrender(&self, hand: Hand) -> Hand {
        assert!(Self::can_surrender_hand(&self, &hand));

        Hand::new(
            hand.cards(),
            hand.bet(),
            HandState::Srndr,
        )
    }

    pub fn can_double_down_hand(&self, hand: &Hand) -> bool {
        if hand.is_terminal() {
            return false;
        }

        if hand.state() == HandState::Split && !self.rules.can_dd_after_split() {
            return false;
        }

        match hand.value() {
            HandValue::Hard(n) => {
                if self.rules.double_down_whitelist().contains(&n) {
                    return true;
                }

                false
            },
            HandValue::Soft { lower, upper } => {
                if self.rules.double_down_whitelist().contains(&lower) || self.rules.double_down_whitelist().contains(&upper) {
                    return true;
                }

                false
            },
        }
    }

    pub fn double_down_hand(&self, hand: Hand, added_bet: f64) -> Hand {
        assert!(Self::can_double_down_hand(&self, &hand));
        assert_eq!(hand.bet(), added_bet);

        Hand::new(
            hand.cards(),
            hand.bet() + added_bet,
            HandState::DDown,
        )
    }

    pub fn gen_actions_for_hands(&self) -> Vec<(Hand, Action)> {
        let mut actions_for_hands: Vec<(Hand, Action)> = Vec::new();

        for hand in &self.hands {
            actions_for_hands.push((hand.clone(), (self.ai)(hand)));
        }

        actions_for_hands
    }

}

#[cfg(test)]
mod tests {}
