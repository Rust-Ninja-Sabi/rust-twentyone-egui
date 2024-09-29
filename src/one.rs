use std::fmt::{Display, Formatter};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::{Rng,thread_rng};
use rand::prelude::SliceRandom;

#[derive(Debug, EnumIter, Copy, Clone)]
enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades
}

impl Suit {
    fn to_string(&self)->String{
        match *self {
            Suit::Diamonds => "♦".to_string(),
            Suit::Clubs => "♣".to_string(),
            Suit::Hearts=> "♥".to_string(),
            Suit::Spades => "♠".to_string()
        }
    }
}

#[derive(Debug, EnumIter, Copy, Clone)]
enum Rank {
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Ace
}

impl Rank {
    fn to_string(&self)->String {
        match *self {
            Rank::King => "K".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Ten=> "10".to_string(),
            Rank::Nine=> "9".to_string(),
            Rank::Eight=> "8".to_string(),
            Rank::Seven=> "7".to_string(),
            Rank::Six=> "6".to_string(),
            Rank::Five=> "5".to_string(),
            Rank::Four=> "4".to_string(),
            Rank::Three=> "3".to_string(),
            Rank::Two=> "2".to_string(),
            Rank::Ace=> "A".to_string()
        }
    }
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
       write!(f, "[ {} {} ]",self.rank.to_string(),self.suit.to_string())
    }
}

impl Card {
    pub fn to_string(&self)->String {
        format!("[ {} {} ]",self.rank.to_string(),self.suit.to_string())
    }

    pub fn new(rank:Rank, suit:Suit) -> Self {
        Self {
            rank: rank,
            suit: suit
        }
    }
}

fn create_cards() -> Vec<Card>{
    let mut cards = Vec::<Card>::new();

    for r in Rank::iter() {
        for s in Suit::iter() {
            cards.push(Card::new(r,s))
        }
    };

    let mut rng = thread_rng();
    cards.shuffle(&mut rng);

    cards

}

fn main() {
    let s = Suit::Hearts;

    let mut c = Card {
        suit: Suit::Hearts,
        rank: Rank::Ace
    };

    c.suit = Suit::Clubs;

    c = Card::new(Rank::Ace, Suit::Hearts);

    //println!("card {}",c.to_string());
    //println!("{}",c.rank.to_string());

    //println!("card {}", c);
    println!("card {}", c);

    let cards = create_cards();
    println!("{:?}",cards);
}
