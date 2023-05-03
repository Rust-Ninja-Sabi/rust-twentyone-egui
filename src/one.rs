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

struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    fn to_string(&self)->String {
        format!("[ {} {} ]",self.rank.to_string(),self.suit.to_string())
    }
}

fn main() {
    let s = Suit::Hearts;

    let mut c = Card {
        suit: Suit::Hearts,
        rank: Rank::Ace
    };

    c.suit = Suit::Clubs;

    println!("card {}",c.to_string());
}

fn random_card() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=11)
}
