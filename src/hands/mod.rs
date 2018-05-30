pub mod ali;
pub mod flor;
pub mod secansa;

use deck::Card;

pub trait Hand<'a>: Ord + Sized {
    fn from_cards(cards: &'a [Card], marker: Card) -> Option<Self>;
}
