#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

#[derive(Debug)]
pub enum Rank {
    Ace,
    Jack,
    Number(u8),
    Queen,
    King
}

use rand::Rng;

impl Suit {
    pub fn random() -> Suit {
        let random_number = rand::thread_rng().gen_range(1..=4);
        Self::translate(random_number)
    }

    pub fn translate(number: u8) -> Suit {
        match number {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random_number = rand::thread_rng().gen_range(1..=13);
        Self::translate(random_number)
    }

    pub fn translate(number: u8) -> Rank {
        match number {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
        }
    }
}


#[derive(Debug)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    matches!(&card.rank, Rank::Ace) && matches!(&card.suit, Suit::Spade)
}

// fn main() {
// 	let your_card = Card {
// 		rank: Rank::random(),
// 		suit: Suit::random(),
// 	};

// 	println!("Your card is {:?}", your_card);

// 	// Now if the card is an Ace of Spades print "You are the winner"
// 	if winner_card(&your_card) {
// 		println!("You are the winner!");
// 	}
// }
