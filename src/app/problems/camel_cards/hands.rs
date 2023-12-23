use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CamelCard {
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
    Ace
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CamelJokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace
}

impl From<char> for CamelCard {
    fn from(value: char) -> Self {
        match value {
            '2' => CamelCard::Two,
            '3' => CamelCard::Three,
            '4' => CamelCard::Four,
            '5' => CamelCard::Five,
            '6' => CamelCard::Six,
            '7' => CamelCard::Seven,
            '8' => CamelCard::Eight,
            '9' => CamelCard::Nine,
            'T' => CamelCard::Ten,
            'J' => CamelCard::Jack,
            'Q' => CamelCard::Queen,
            'K' => CamelCard::King,
            'A' => CamelCard::Ace,
            _ => panic!("Invalid char for a card")
        }
    }
}

impl From<char> for CamelJokerCard {
    fn from(value: char) -> Self {
        match value {
            'J' => CamelJokerCard::Joker,
            '2' => CamelJokerCard::Two,
            '3' => CamelJokerCard::Three,
            '4' => CamelJokerCard::Four,
            '5' => CamelJokerCard::Five,
            '6' => CamelJokerCard::Six,
            '7' => CamelJokerCard::Seven,
            '8' => CamelJokerCard::Eight,
            '9' => CamelJokerCard::Nine,
            'T' => CamelJokerCard::Ten,
            'Q' => CamelJokerCard::Queen,
            'K' => CamelJokerCard::King,
            'A' => CamelJokerCard::Ace,
            _ => panic!("Invalid char for a card")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CamelHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CamelHand<T> {
    hand_type: CamelHandType,
    hand_cards: [T; 5]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CamelBid<T> {
    pub hand: CamelHand<T>,
    pub bid_size: usize
}

impl CamelBid<CamelCard> {
    fn without_jokers(hand_cards: [CamelCard; 5], bid_size: usize) -> Self {
        let mut cards_map: HashMap<CamelCard, usize> = HashMap::new();

        for hand_card in hand_cards {
            if let Some(value) = cards_map.get_mut(&hand_card) {
                *value += 1;
            } else {
                cards_map.insert(hand_card, 1);
            }
        }

        let mut partition = cards_map.into_values().collect::<Vec<usize>>();
        partition.sort();

        let hand_type = get_hand_type_by_partition(partition);

        CamelBid {
            hand: CamelHand {
                hand_type,
                hand_cards
            },
            bid_size
        }
    }
}

impl From<&str> for CamelBid<CamelCard> {
    fn from(value: &str) -> Self {
        let (hand_string, bid_string) = value.split_once(' ').unwrap();
        let hand = hand_string.chars().map(|x| -> CamelCard { x.into() }).collect::<Vec<CamelCard>>();
        let hand = [hand[0], hand[1], hand[2], hand[3], hand[4]];
        let bid_size = bid_string.parse::<usize>().unwrap();

        CamelBid::without_jokers(hand, bid_size)
    }
}

impl CamelBid<CamelJokerCard> {
    fn with_jokers(hand_cards: [CamelJokerCard; 5], bid_size: usize) -> CamelBid<CamelJokerCard> {
        let mut cards_map: HashMap<CamelJokerCard, usize> = HashMap::new();
        let mut jokers: usize = 0;

        for hand_card in hand_cards {
            if hand_card == CamelJokerCard::Joker {
                jokers += 1;
            } else if let Some(value) = cards_map.get_mut(&hand_card) {
                *value += 1;
            } else {
                cards_map.insert(hand_card, 1);
            }
        }

        let mut partition = cards_map.into_values().collect::<Vec<usize>>();
        partition.sort();
        if let Some(last) = partition.last_mut() {
            *last += jokers;
        } else {
            partition.push(jokers);
        }

        let hand_type = get_hand_type_by_partition(partition);

        CamelBid {
            hand: CamelHand {
                hand_type,
                hand_cards
            },
            bid_size
        }
    }
}

impl From<&str> for CamelBid<CamelJokerCard> {
    fn from(value: &str) -> Self {
        let (hand_string, bid_string) = value.split_once(' ').unwrap();
        let hand = hand_string.chars().map(|x| -> CamelJokerCard { x.into() }).collect::<Vec<CamelJokerCard>>();
        let hand = [hand[0], hand[1], hand[2], hand[3], hand[4]];
        let bid_size = bid_string.parse::<usize>().unwrap();

        CamelBid::with_jokers(hand, bid_size)
    }
}

fn get_hand_type_by_partition(partition: Vec<usize>) -> CamelHandType {
    match partition.len() {
        5 => CamelHandType::HighCard, // [1, 1, 1, 1, 1]
        4 => CamelHandType::OnePair, // [1, 1, 1, 2]
        3 if partition[1] == 2 => CamelHandType::TwoPair, // [1, 2, 2]
        3 => CamelHandType::ThreeOfKind, // [1, 1, 3]
        2 if partition[0] == 2 => CamelHandType::FullHouse, // [2, 3]
        2 => CamelHandType::FourOfKind, // [1, 4]
        1 => CamelHandType::FiveOfKind, // [5]
        _ => panic!("Unexpected card distribution")
    }
}
