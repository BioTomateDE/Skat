use crate::reizen::Ansage;

#[derive(Debug, Clone)]
pub enum GameMode {
    Normal(NormalGame),
    Null(NullGame),
    Ramsch,
}

#[derive(Debug, Clone)]
pub struct NormalGame {
    pub trumpf: Trumpf,
    pub ansage: Ansage,
    pub hand: bool,
    pub ouvert: bool,
}

#[derive(Debug, Clone)]
pub struct NullGame {
    pub hand: bool,
    pub ouvert: bool,
}

#[derive(Debug, Clone)]
pub enum Trumpf {
    Karo,
    Herz,
    Pik,
    Kreuz,
    Bube,
}

