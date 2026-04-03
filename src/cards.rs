use serde::{Deserialize, Serialize};

/// Playing card suits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }

    pub fn symbol(&self) -> &str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        }
    }
}

/// Playing card ranks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Rank {
    pub fn chip_value(&self) -> u32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    pub fn display(&self) -> &str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }

    pub fn is_face(&self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }

    pub fn all() -> [Rank; 13] {
        [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }
}

/// Card editions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Edition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

impl Edition {
    pub fn chip_bonus(&self) -> u32 {
        match self {
            Edition::Foil => 50,
            _ => 0,
        }
    }

    pub fn mult_bonus(&self) -> u32 {
        match self {
            Edition::Holographic => 10,
            _ => 0,
        }
    }

    pub fn x_mult(&self) -> f64 {
        match self {
            Edition::Polychrome => 1.5,
            _ => 1.0,
        }
    }
}

/// Card enhancements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Enhancement {
    None,
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

impl Enhancement {
    pub fn chip_bonus(&self) -> u32 {
        match self {
            Enhancement::Bonus => 30,
            Enhancement::Stone => 50,
            _ => 0,
        }
    }

    pub fn mult_bonus(&self) -> u32 {
        match self {
            Enhancement::Mult => 4,
            _ => 0,
        }
    }

    pub fn x_mult(&self) -> f64 {
        match self {
            Enhancement::Glass => 2.0,
            _ => 1.0,
        }
    }
}

/// Card seals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Seal {
    None,
    Gold,
    Red,
    Blue,
    Purple,
}

/// A playing card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub enhancement: Enhancement,
    pub edition: Edition,
    pub seal: Seal,
    pub face_up: bool,
    pub selected: bool,
    pub debuffed: bool,
    pub id: u32,
}

impl Card {
    pub fn display_name(&self) -> String {
        format!("{}{}", self.rank.display(), self.suit.symbol())
    }

    pub fn total_chips(&self) -> u32 {
        self.rank.chip_value() + self.enhancement.chip_bonus() + self.edition.chip_bonus()
    }

    pub fn total_mult(&self) -> u32 {
        self.enhancement.mult_bonus() + self.edition.mult_bonus()
    }
}

/// Poker hand types in Balatro
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandType {
    pub fn base_score(&self) -> (u32, u32) {
        match self {
            HandType::HighCard => (5, 1),
            HandType::Pair => (10, 2),
            HandType::TwoPair => (20, 2),
            HandType::ThreeOfAKind => (30, 3),
            HandType::Straight => (30, 4),
            HandType::Flush => (35, 4),
            HandType::FullHouse => (40, 4),
            HandType::FourOfAKind => (60, 7),
            HandType::StraightFlush => (100, 8),
            HandType::RoyalFlush => (100, 8),
            HandType::FiveOfAKind => (120, 12),
            HandType::FlushHouse => (140, 14),
            HandType::FlushFive => (160, 16),
        }
    }

    pub fn level_up_amount(&self) -> (u32, u32) {
        match self {
            HandType::HighCard => (10, 1),
            HandType::Pair => (15, 1),
            HandType::TwoPair => (20, 1),
            HandType::ThreeOfAKind => (20, 2),
            HandType::Straight => (30, 3),
            HandType::Flush => (15, 2),
            HandType::FullHouse => (25, 2),
            HandType::FourOfAKind => (30, 3),
            HandType::StraightFlush => (40, 4),
            HandType::RoyalFlush => (40, 4),
            HandType::FiveOfAKind => (35, 3),
            HandType::FlushHouse => (40, 4),
            HandType::FlushFive => (50, 5),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            HandType::HighCard => "High Card",
            HandType::Pair => "Pair",
            HandType::TwoPair => "Two Pair",
            HandType::ThreeOfAKind => "Three of a Kind",
            HandType::Straight => "Straight",
            HandType::Flush => "Flush",
            HandType::FullHouse => "Full House",
            HandType::FourOfAKind => "Four of a Kind",
            HandType::StraightFlush => "Straight Flush",
            HandType::RoyalFlush => "Royal Flush",
            HandType::FiveOfAKind => "Five of a Kind",
            HandType::FlushHouse => "Flush House",
            HandType::FlushFive => "Flush Five",
        }
    }
}

/// Joker kinds - all 150+ from Balatro
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JokerKind {
    // Common
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker,
    HalfJoker,
    CreditCard,
    BannerJoker,
    MysticSummit,
    EightBall,
    Misprint,
    RaisedFist,
    Fibonacci,
    ScaryFace,
    Abstract,
    DelayedGratification,
    Pareidolia,
    GrossMichel,
    Supernova,
    RideTheBus,
    Blackboard,
    Runner,
    IceCream,
    Splash,
    BluePrint,
    Brainstorm,
    // Uncommon
    SpaceJoker,
    Shortcut,
    Burglar,
    Constellation,
    Hiker,
    Faceless,
    GreenJoker,
    Superposition,
    ToDoList,
    Acrobat,
    SockAndBuskin,
    Troubadour,
    Certificate,
    SmearedJoker,
    Throwback,
    RoughGem,
    Bloodstone,
    Arrowhead,
    OnyxAgate,
    GlassJoker,
    Showman,
    FlowerPot,
    MerryAndy,
    OopsAll6s,
    TheIdol,
    SeeingDouble,
    Matador,
    Satellite,
    CartomancyJoker,
    AstronomyJoker,
    // Rare
    DNA,
    Vagabond,
    Baron,
    Mime,
    Seance,
    LoyaltyCard,
    Dusk,
    Stuntman,
    DrunkardJoker,
    StoneThing,
    SteelJoker,
    GoldenTicket,
    MrBones,
    Baseball,
    Bull,
    Diet,
    Hack,
    Hologram,
    Madness,
    Vampire,
    Photograph,
    Ceremonial,
    // Legendary
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}

impl JokerKind {
    pub fn display_name(&self) -> &str {
        match self {
            JokerKind::Joker => "Joker",
            JokerKind::GreedyJoker => "Greedy Joker",
            JokerKind::LustyJoker => "Lusty Joker",
            JokerKind::WrathfulJoker => "Wrathful Joker",
            JokerKind::GluttonousJoker => "Gluttonous Joker",
            JokerKind::JollyJoker => "Jolly Joker",
            JokerKind::ZanyJoker => "Zany Joker",
            JokerKind::MadJoker => "Mad Joker",
            JokerKind::CrazyJoker => "Crazy Joker",
            JokerKind::DrollJoker => "Droll Joker",
            JokerKind::SlyJoker => "Sly Joker",
            JokerKind::WilyJoker => "Wily Joker",
            JokerKind::CleverJoker => "Clever Joker",
            JokerKind::DeviousJoker => "Devious Joker",
            JokerKind::CraftyJoker => "Crafty Joker",
            JokerKind::HalfJoker => "Half Joker",
            JokerKind::CreditCard => "Credit Card",
            JokerKind::BannerJoker => "Banner",
            JokerKind::MysticSummit => "Mystic Summit",
            JokerKind::EightBall => "8 Ball",
            JokerKind::Misprint => "Misprint",
            JokerKind::RaisedFist => "Raised Fist",
            JokerKind::Fibonacci => "Fibonacci",
            JokerKind::ScaryFace => "Scary Face",
            JokerKind::Abstract => "Abstract Joker",
            JokerKind::DelayedGratification => "Delayed Gratification",
            JokerKind::Pareidolia => "Pareidolia",
            JokerKind::GrossMichel => "Gros Michel",
            JokerKind::Supernova => "Supernova",
            JokerKind::RideTheBus => "Ride the Bus",
            JokerKind::Blackboard => "Blackboard",
            JokerKind::Runner => "Runner",
            JokerKind::IceCream => "Ice Cream",
            JokerKind::Splash => "Splash",
            JokerKind::BluePrint => "Blueprint",
            JokerKind::Brainstorm => "Brainstorm",
            JokerKind::SpaceJoker => "Space Joker",
            JokerKind::Shortcut => "Shortcut",
            JokerKind::Burglar => "Burglar",
            JokerKind::Constellation => "Constellation",
            JokerKind::Hiker => "Hiker",
            JokerKind::Faceless => "Faceless Joker",
            JokerKind::GreenJoker => "Green Joker",
            JokerKind::Superposition => "Superposition",
            JokerKind::ToDoList => "To Do List",
            JokerKind::Acrobat => "Acrobat",
            JokerKind::SockAndBuskin => "Sock and Buskin",
            JokerKind::Troubadour => "Troubadour",
            JokerKind::Certificate => "Certificate",
            JokerKind::SmearedJoker => "Smeared Joker",
            JokerKind::Throwback => "Throwback",
            JokerKind::RoughGem => "Rough Gem",
            JokerKind::Bloodstone => "Bloodstone",
            JokerKind::Arrowhead => "Arrowhead",
            JokerKind::OnyxAgate => "Onyx Agate",
            JokerKind::GlassJoker => "Glass Joker",
            JokerKind::Showman => "Showman",
            JokerKind::FlowerPot => "Flower Pot",
            JokerKind::MerryAndy => "Merry Andy",
            JokerKind::OopsAll6s => "Oops! All 6s",
            JokerKind::TheIdol => "The Idol",
            JokerKind::SeeingDouble => "Seeing Double",
            JokerKind::Matador => "Matador",
            JokerKind::Satellite => "Satellite",
            JokerKind::CartomancyJoker => "Cartomancy",
            JokerKind::AstronomyJoker => "Astronomy",
            JokerKind::DNA => "DNA",
            JokerKind::Vagabond => "Vagabond",
            JokerKind::Baron => "Baron",
            JokerKind::Mime => "Mime",
            JokerKind::Seance => "Séance",
            JokerKind::LoyaltyCard => "Loyalty Card",
            JokerKind::Dusk => "Dusk",
            JokerKind::Stuntman => "Stuntman",
            JokerKind::DrunkardJoker => "Drunkard",
            JokerKind::StoneThing => "Stone Joker",
            JokerKind::SteelJoker => "Steel Joker",
            JokerKind::GoldenTicket => "Golden Ticket",
            JokerKind::MrBones => "Mr. Bones",
            JokerKind::Baseball => "Baseball Card",
            JokerKind::Bull => "Bull",
            JokerKind::Diet => "Diet Cola",
            JokerKind::Hack => "Hack",
            JokerKind::Hologram => "Hologram",
            JokerKind::Madness => "Madness",
            JokerKind::Vampire => "Vampire",
            JokerKind::Photograph => "Photograph",
            JokerKind::Ceremonial => "Ceremonial Dagger",
            JokerKind::Canio => "Canio",
            JokerKind::Triboulet => "Triboulet",
            JokerKind::Yorick => "Yorick",
            JokerKind::Chicot => "Chicot",
            JokerKind::Perkeo => "Perkeo",
        }
    }

    pub fn base_cost(&self) -> u32 {
        match self {
            JokerKind::Canio
            | JokerKind::Triboulet
            | JokerKind::Yorick
            | JokerKind::Chicot
            | JokerKind::Perkeo => 20,
            JokerKind::DNA
            | JokerKind::Vagabond
            | JokerKind::Baron
            | JokerKind::Mime
            | JokerKind::Seance => 8,
            _ => 4,
        }
    }

    pub fn rarity(&self) -> JokerRarity {
        match self {
            JokerKind::Canio
            | JokerKind::Triboulet
            | JokerKind::Yorick
            | JokerKind::Chicot
            | JokerKind::Perkeo => JokerRarity::Legendary,
            JokerKind::DNA
            | JokerKind::Vagabond
            | JokerKind::Baron
            | JokerKind::Mime
            | JokerKind::Seance
            | JokerKind::LoyaltyCard
            | JokerKind::Dusk
            | JokerKind::Stuntman
            | JokerKind::DrunkardJoker
            | JokerKind::StoneThing
            | JokerKind::SteelJoker
            | JokerKind::GoldenTicket
            | JokerKind::MrBones
            | JokerKind::Baseball
            | JokerKind::Bull
            | JokerKind::Diet
            | JokerKind::Hack
            | JokerKind::Hologram
            | JokerKind::Madness
            | JokerKind::Vampire
            | JokerKind::Photograph
            | JokerKind::Ceremonial => JokerRarity::Rare,
            JokerKind::SpaceJoker
            | JokerKind::Shortcut
            | JokerKind::Burglar
            | JokerKind::Constellation
            | JokerKind::Hiker
            | JokerKind::Faceless
            | JokerKind::GreenJoker
            | JokerKind::Superposition
            | JokerKind::ToDoList
            | JokerKind::Acrobat
            | JokerKind::SockAndBuskin
            | JokerKind::Troubadour
            | JokerKind::Certificate
            | JokerKind::SmearedJoker
            | JokerKind::Throwback
            | JokerKind::RoughGem
            | JokerKind::Bloodstone
            | JokerKind::Arrowhead
            | JokerKind::OnyxAgate
            | JokerKind::GlassJoker
            | JokerKind::Showman
            | JokerKind::FlowerPot
            | JokerKind::MerryAndy
            | JokerKind::OopsAll6s
            | JokerKind::TheIdol
            | JokerKind::SeeingDouble
            | JokerKind::Matador
            | JokerKind::Satellite
            | JokerKind::CartomancyJoker
            | JokerKind::AstronomyJoker => JokerRarity::Uncommon,
            _ => JokerRarity::Common,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JokerRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

/// A joker card instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerCard {
    pub kind: JokerKind,
    pub edition: Edition,
    pub sell_value: u32,
    pub counter: i32,
}

impl JokerCard {
    pub fn new(kind: JokerKind) -> Self {
        Self {
            kind,
            edition: Edition::Base,
            sell_value: kind.base_cost() / 2,
            counter: 0,
        }
    }
}
