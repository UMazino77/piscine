use rand::random;

#[derive(Debug,PartialEq)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

#[derive(Debug,PartialEq)]

pub enum Rank {
    Ace, 
    King,
    Queen, 
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let a = (random::<u8>() % 4) + 1;
        Suit::translate(a)
    }

    pub fn translate(value: u8) -> Suit {
        return match value {
            1=> Suit::Heart,
            2=> Suit::Diamond,
            3=> Suit::Spade,
            4=> Suit::Club,
            _=> todo!()
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let a = (random::<u8>() % 13) + 1;
        Rank::translate(a)
    }

    pub fn translate(value: u8) -> Rank {
        return match value {
            1=> Rank::Ace,
            11=> Rank::Jack,
            12=> Rank::Queen,
            13=> Rank::King,
            2..=10 => Rank::Number(value),
            _=> todo!()          
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card == &Card{suit : Suit::Spade, rank : Rank::Ace}
}