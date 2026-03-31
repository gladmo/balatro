use serde::{Deserialize, Serialize};

// ── Suits ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs]
    }
    pub fn name(&self) -> &'static str {
        match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
        }
    }
}

// ── Ranks ─────────────────────────────────────────────────────────────────

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
    pub fn all() -> [Rank; 13] {
        [
            Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
            Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
        ]
    }

    /// Base chip value of a card rank.
    pub fn chip_value(&self) -> u32 {
        match self {
            Rank::Two => 2, Rank::Three => 3, Rank::Four => 4,
            Rank::Five => 5, Rank::Six => 6, Rank::Seven => 7,
            Rank::Eight => 8, Rank::Nine => 9, Rank::Ten => 10,
            Rank::Jack => 10, Rank::Queen => 10, Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    pub fn is_face(&self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }

    pub fn is_even(&self) -> bool {
        (*self as u8) % 2 == 0
    }

    pub fn name(&self) -> &'static str {
        match self {
            Rank::Two => "2", Rank::Three => "3", Rank::Four => "4",
            Rank::Five => "5", Rank::Six => "6", Rank::Seven => "7",
            Rank::Eight => "8", Rank::Nine => "9", Rank::Ten => "10",
            Rank::Jack => "J", Rank::Queen => "Q", Rank::King => "K",
            Rank::Ace => "A",
        }
    }
}

// ── Enhancements ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Enhancement {
    #[default]
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

// ── Editions ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Edition {
    #[default]
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

// ── Seals ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Seal {
    #[default]
    None,
    Gold,
    Red,
    Purple,
    Blue,
}

// ── Playing Card Component ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayingCardData {
    pub suit: Suit,
    pub rank: Rank,
    pub enhancement: Enhancement,
    pub edition: Edition,
    pub seal: Seal,
    pub debuffed: bool,
}

impl PlayingCardData {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank,
            enhancement: Enhancement::None,
            edition: Edition::Base,
            seal: Seal::None,
            debuffed: false,
        }
    }

    pub fn chip_value(&self) -> u32 {
        match self.enhancement {
            Enhancement::Stone => 50,
            Enhancement::Bonus => self.rank.chip_value() + 30,
            Enhancement::Lucky => self.rank.chip_value(),
            _ => self.rank.chip_value(),
        }
    }
}

// ── Joker Types ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JokerType {
    // Common (rarity 1)
    Joker,
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
    JokerStencil,
    FourFingers,
    Mime,
    CreditCard,
    CeremonialDagger,
    Banner,
    MysticSummit,
    MarbleJoker,
    LoyaltyCard,
    EightBall,
    Misprint,
    DuskJoker,
    RaisedFist,
    Fibonacci,
    SteelJoker,
    ScaryFace,
    AbstractJoker,
    DelayedGratification,
    Hack,
    Pareidolia,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    Troubadour,
    Certificate,
    SmearedJoker,
    Throwback,
    HangingChad,
    RoughGem,
    Bloodstone,
    Arrowhead,
    OnyxAgate,
    GlassJoker,
    ShowMan,
    FlowerPot,
    Blueprint,
    WeeJoker,
    MerryAndy,
    OopsAllSixes,
    TheIdol,
    CrazyLong,
    Supernova,
    RideTheBus,
    Burglar,
    Blackboard,
    Runner,
    IceCream,
    DNA,
    Splash,
    BlueJoker,
    SixthSense,
    Constellation,
    Hiker,
    FacelessJoker,
    GreenJoker,
    SuperPosition,
    ToDo,
    // Uncommon (rarity 2)
    Caino,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
    // Rare (rarity 3)
    Photograph,
    AncientJoker,
    WalkieTalkie,
    Seltzer,
    Castle,
    SmileyFace,
    Scholar,
    BusinessCard,
    EvenSteven,
    OddTodd,
    Vampire,
    Shortcut,
    Hologram,
    Vagabond,
    Baron,
    Cloud9,
    Rocket,
    Obelisk,
    Midas,
    Luchador,
    Hitchhiker,
    Shoot,
    // Legendary (rarity 4)
    Canio,
    Gros,
    Onlooker,
    // Wild card
    Unknown,
}

impl JokerType {
    pub fn name(&self) -> &'static str {
        match self {
            JokerType::Joker => "Joker",
            JokerType::JollyJoker => "Jolly Joker",
            JokerType::ZanyJoker => "Zany Joker",
            JokerType::MadJoker => "Mad Joker",
            JokerType::CrazyJoker => "Crazy Joker",
            JokerType::DrollJoker => "Droll Joker",
            JokerType::SlyJoker => "Sly Joker",
            JokerType::WilyJoker => "Wily Joker",
            JokerType::CleverJoker => "Clever Joker",
            JokerType::DeviousJoker => "Devious Joker",
            JokerType::CraftyJoker => "Crafty Joker",
            JokerType::HalfJoker => "Half Joker",
            JokerType::AbstractJoker => "Abstract Joker",
            JokerType::ScaryFace => "Scary Face",
            JokerType::SteelJoker => "Steel Joker",
            JokerType::RaisedFist => "Raised Fist",
            JokerType::EvenSteven => "Even Steven",
            JokerType::OddTodd => "Odd Todd",
            JokerType::Scholar => "Scholar",
            JokerType::BusinessCard => "Business Card",
            JokerType::Supernova => "Supernova",
            JokerType::RideTheBus => "Ride the Bus",
            JokerType::Photograph => "Photograph",
            JokerType::AncientJoker => "Ancient Joker",
            JokerType::WalkieTalkie => "Walkie Talkie",
            JokerType::SmileyFace => "Smiley Face",
            JokerType::Banner => "Banner",
            JokerType::MysticSummit => "Mystic Summit",
            JokerType::Fibonacci => "Fibonacci",
            JokerType::Constellation => "Constellation",
            JokerType::GreenJoker => "Green Joker",
            JokerType::BlueJoker => "Blue Joker",
            JokerType::Baron => "Baron",
            JokerType::Triboulet => "Triboulet",
            JokerType::Yorick => "Yorick",
            JokerType::Canio => "Canio",
            JokerType::Chicot => "Chicot",
            JokerType::Perkeo => "Perkeo",
            _ => "Joker",
        }
    }

    pub fn base_cost(&self) -> u32 {
        match self {
            JokerType::Joker => 2,
            JokerType::JollyJoker | JokerType::ZanyJoker | JokerType::MadJoker
            | JokerType::CrazyJoker | JokerType::DrollJoker => 3,
            JokerType::SlyJoker | JokerType::WilyJoker | JokerType::CleverJoker
            | JokerType::DeviousJoker | JokerType::CraftyJoker => 4,
            JokerType::Triboulet | JokerType::Yorick | JokerType::Canio
            | JokerType::Chicot | JokerType::Perkeo => 20,
            _ => 5,
        }
    }

    pub fn rarity(&self) -> u8 {
        match self {
            JokerType::Joker | JokerType::JollyJoker | JokerType::ZanyJoker
            | JokerType::MadJoker | JokerType::CrazyJoker | JokerType::DrollJoker
            | JokerType::SlyJoker | JokerType::WilyJoker | JokerType::CleverJoker
            | JokerType::DeviousJoker | JokerType::CraftyJoker | JokerType::HalfJoker
            | JokerType::Banner | JokerType::MysticSummit | JokerType::AbstractJoker
            | JokerType::RaisedFist | JokerType::ScaryFace | JokerType::CreditCard
            | JokerType::SteelJoker | JokerType::GreenJoker | JokerType::BlueJoker
            | JokerType::OopsAllSixes | JokerType::IceCream | JokerType::Splash => 1,
            JokerType::Photograph | JokerType::AncientJoker | JokerType::WalkieTalkie
            | JokerType::Seltzer | JokerType::Castle | JokerType::SmileyFace
            | JokerType::Scholar | JokerType::BusinessCard | JokerType::EvenSteven
            | JokerType::OddTodd | JokerType::Supernova | JokerType::RideTheBus
            | JokerType::Fibonacci | JokerType::Constellation | JokerType::Baron
            | JokerType::Hologram | JokerType::Obelisk => 2,
            JokerType::Triboulet | JokerType::Yorick => 3,
            JokerType::Canio | JokerType::Chicot | JokerType::Perkeo => 4,
            _ => 1,
        }
    }
}

// ── Tarot Cards ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TarotCard {
    TheFool,
    TheMagician,
    TheHighPriestess,
    TheEmpress,
    TheEmperor,
    TheHierophant,
    TheLovers,
    TheChariot,
    Justice,
    TheHermit,
    WheelOfFortune,
    Strength,
    TheHangedMan,
    Death,
    Temperance,
    TheDevil,
    TheTower,
    TheStar,
    TheMoon,
    TheSun,
    Judgement,
    TheWorld,
}

impl TarotCard {
    pub fn name(&self) -> &'static str {
        match self {
            TarotCard::TheFool => "The Fool",
            TarotCard::TheMagician => "The Magician",
            TarotCard::TheHighPriestess => "The High Priestess",
            TarotCard::TheEmpress => "The Empress",
            TarotCard::TheEmperor => "The Emperor",
            TarotCard::TheHierophant => "The Hierophant",
            TarotCard::TheLovers => "The Lovers",
            TarotCard::TheChariot => "The Chariot",
            TarotCard::Justice => "Justice",
            TarotCard::TheHermit => "The Hermit",
            TarotCard::WheelOfFortune => "Wheel of Fortune",
            TarotCard::Strength => "Strength",
            TarotCard::TheHangedMan => "The Hanged Man",
            TarotCard::Death => "Death",
            TarotCard::Temperance => "Temperance",
            TarotCard::TheDevil => "The Devil",
            TarotCard::TheTower => "The Tower",
            TarotCard::TheStar => "The Star",
            TarotCard::TheMoon => "The Moon",
            TarotCard::TheSun => "The Sun",
            TarotCard::Judgement => "Judgement",
            TarotCard::TheWorld => "The World",
        }
    }
}

// ── Planet Cards ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetCard {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    PlanetX,
    Ceres,
    Eris,
}

impl PlanetCard {
    pub fn name(&self) -> &'static str {
        match self {
            PlanetCard::Mercury => "Mercury",
            PlanetCard::Venus => "Venus",
            PlanetCard::Earth => "Earth",
            PlanetCard::Mars => "Mars",
            PlanetCard::Jupiter => "Jupiter",
            PlanetCard::Saturn => "Saturn",
            PlanetCard::Uranus => "Uranus",
            PlanetCard::Neptune => "Neptune",
            PlanetCard::Pluto => "Pluto",
            PlanetCard::PlanetX => "Planet X",
            PlanetCard::Ceres => "Ceres",
            PlanetCard::Eris => "Eris",
        }
    }

    /// Which hand this planet levels up.
    pub fn levels_up(&self) -> &'static str {
        match self {
            PlanetCard::Mercury => "Pair",
            PlanetCard::Venus => "Three of a Kind",
            PlanetCard::Earth => "Full House",
            PlanetCard::Mars => "Four of a Kind",
            PlanetCard::Jupiter => "Flush",
            PlanetCard::Saturn => "Straight",
            PlanetCard::Uranus => "Two Pair",
            PlanetCard::Neptune => "Straight Flush",
            PlanetCard::Pluto => "High Card",
            PlanetCard::PlanetX => "Five of a Kind",
            PlanetCard::Ceres => "Flush House",
            PlanetCard::Eris => "Flush Five",
        }
    }
}

// ── Spectral Cards ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpectralCard {
    Familiar,
    Grim,
    Incantation,
    Talisman,
    Aura,
    Wraith,
    Sigil,
    Ouija,
    Ectoplasm,
    Immolate,
    Ankh,
    DejaVu,
    Hex,
    Trance,
    Medium,
    Cryptid,
    TheSoul,
    BlackHole,
}

impl SpectralCard {
    pub fn name(&self) -> &'static str {
        match self {
            SpectralCard::Familiar => "Familiar",
            SpectralCard::Grim => "Grim",
            SpectralCard::Incantation => "Incantation",
            SpectralCard::Talisman => "Talisman",
            SpectralCard::Aura => "Aura",
            SpectralCard::Wraith => "Wraith",
            SpectralCard::Sigil => "Sigil",
            SpectralCard::Ouija => "Ouija",
            SpectralCard::Ectoplasm => "Ectoplasm",
            SpectralCard::Immolate => "Immolate",
            SpectralCard::Ankh => "Ankh",
            SpectralCard::DejaVu => "Deja Vu",
            SpectralCard::Hex => "Hex",
            SpectralCard::Trance => "Trance",
            SpectralCard::Medium => "Medium",
            SpectralCard::Cryptid => "Cryptid",
            SpectralCard::TheSoul => "The Soul",
            SpectralCard::BlackHole => "Black Hole",
        }
    }
}

// ── Vouchers ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoucherType {
    Overstock,
    OverstockPlus,
    Clearance,
    ClearanceSale,
    Hone,
    GlowUp,
    Reroll,
    RerollSurplus,
    CrystalBall,
    Omen,
    Telescope,
    Observatory,
    Grabber,
    Nacho,
    PlanetMerchant,
    PlanetTycoon,
    Seed,
    BlindSpot,
    Hieroglyph,
    Petroglyph,
    DirectorsPass,
    Detox,
    Palette,
    Liquidation,
    Gimmick,
    Illusion,
    Omen2,
    TwoForOne,
    MagicTrick,
    ErraticDeck,
}

impl VoucherType {
    pub fn name(&self) -> &'static str {
        match self {
            VoucherType::Overstock => "Overstock",
            VoucherType::OverstockPlus => "Overstock Plus",
            VoucherType::Clearance => "Clearance Sale",
            VoucherType::ClearanceSale => "Liquidation",
            VoucherType::Hone => "Hone",
            VoucherType::GlowUp => "Glow Up",
            VoucherType::Reroll => "Reroll Surplus",
            VoucherType::RerollSurplus => "Reroll Glut",
            VoucherType::CrystalBall => "Crystal Ball",
            VoucherType::Omen => "Omen Globe",
            VoucherType::Telescope => "Telescope",
            VoucherType::Observatory => "Observatory",
            VoucherType::Grabber => "Grabber",
            VoucherType::Nacho => "Nacho Tray",
            VoucherType::PlanetMerchant => "Planet Merchant",
            VoucherType::PlanetTycoon => "Planet Tycoon",
            _ => "Voucher",
        }
    }
}
