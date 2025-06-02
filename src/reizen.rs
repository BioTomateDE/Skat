use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::cards::{Card, CardColor, CardNumber};
use crate::game::{GameMode, Trumpf};

#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Ansage {
    Keine = 0,         // player must get at least 60 points (normal game)
    Schneider = 1,      // player must get at least 90 points
    Schwarz = 2,        // player must get all stiche
}


/// Returns a boolean slice of 4; each bool saying whether that Bube exists or not.
/// Sorted in descending order (Kreuz, Pik, Herz, Karo)
fn get_buben(cards: &[Card]) -> [bool; 4] {
    let mut buben: [bool; 4] = [false, false, false, false];

    for card in cards {
        if card.number != CardNumber::Bube {
            continue
        }
        match card.color {
            CardColor::Kreuz => buben[0] = true,
            CardColor::Pik   => buben[1] = true,
            CardColor::Herz  => buben[2] = true,
            CardColor::Karo  => buben[3] = true,
        }
    }

    buben
}


/// Get the "base factor" for Reizen (mit x / ohne x).
fn get_base_factor(buben: &[bool; 4]) -> u32 {
    for i in 0..4 {
        if buben[i] != buben[0] {
            return i as u32
        }
    }
    4
}


pub fn get_reizen_points(cards: &[Card], gamemode: &GameMode) -> Result<u32, String> {
    let buben: [bool; 4] = get_buben(cards);
    let mut factor: u32 = get_base_factor(&buben);

    let ansage: Ansage;
    let grundwert: u32 = match gamemode {
        GameMode::Normal(game) => {
            ansage = game.ansage;
            if game.hand {
                factor += 1;
            }
            if game.ouvert {
                factor += 1;
            }
            match game.trumpf {
                Trumpf::Karo => 9,
                Trumpf::Herz => 10,
                Trumpf::Pik => 11,
                Trumpf::Kreuz => 12,
                Trumpf::Bube => 24,
            }
        }
        GameMode::Null(game) => {
            ansage = Ansage::Keine;
            match (game.hand, game.ouvert) {
                (false, false) => 23,
                (true, false) => 35,
                (false, true) => 46,
                (true, true) => 59,
            }
        }
        GameMode::Ramsch => return Err("Gamemode cannot be Ramsch if Reizen is relevant".to_string())
    };

    factor += 1;    // mit x -> spiel x+1
    factor += match ansage {
        Ansage::Keine => 0,
        Ansage::Schneider => 1,
        Ansage::Schwarz => 2,
    };

    Ok(factor * grundwert)
}


