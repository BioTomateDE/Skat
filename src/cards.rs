

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub color: CardColor,
    pub number: CardNumber,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardColor {
    Karo,     // Diamonds
    Herz,     // Hearts
    Pik,      // Spades
    Kreuz,    // Clubs
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardNumber {
    Sieben,     // 7
    Acht,       // 8
    Neun,       // 9
    Zehn,       // 10
    Bube,       // Jack
    Dame,       // Queen
    Koenig,     // King
    Ass,        // Ace
}

