use crate::card::Card;

pub struct Hand<S: HandState> {
    stack: Vec<Card>,
    marker: std::marker::PhantomData<S>,
}

// possible hand states
pub enum Fresh {}
pub enum FreshSplit {}
pub enum SplitAcesOpen {}
pub enum SplitAcesLocked {}
pub enum DoubledDown {}
pub enum Stood {}
pub enum Surrendered {}

pub trait HandState {}
impl HandState for Fresh {}
impl HandState for FreshSplit {}
impl HandState for SplitAcesOpen {}
impl HandState for SplitAcesLocked {}
impl HandState for DoubledDown {}
impl HandState for Stood {}
impl HandState for Surrendered {}
