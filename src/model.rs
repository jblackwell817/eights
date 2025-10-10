use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The result for a particular year for either men's or womens' eights
#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    year: u16,
    standings: Vec<Crew>,
}

/// A crew is defined by their college and what boat they were in (1st, 2nd, etc)
#[derive(Serialize, Deserialize, Debug)]
pub struct Crew {
    college: College,
    boat: u8,
}

/// The colleges which have participated in eights. Some of these (e.g. Westminster) no
/// longer exist as colleges and some (e.g. BrasenoseStPeters) are combination teams.
/// This is also not an exhaustive list of all Oxford colleges, just those which have
/// participated in eights according to the data we have.
#[derive(Serialize, Deserialize, Debug, EnumString, Display)]
#[strum(serialize_all = "PascalCase")]
pub enum College {
    Balliol,
    Brasenose,
    BrasenoseStPeters,
    ChristChurch,
    CorpusChristi,
    Exeter,
    GreenTempleton,
    HarrisManchester,
    Hertford,
    Jesus,
    Keble,
    LadyMargaretHall,
    Linacre,
    Lincoln,
    Magdalen,
    Mansfield,
    Merton,
    New,
    Oriel,
    OslerGreen,
    OslerHouse,
    Pembroke,
    Queens,
    RegentsPark,
    Reuben,
    StAnnes,
    StAntonys,
    StBenetsHall,
    StCatherines,
    StEdmundHall,
    StHildas,
    StHughs,
    StJohns,
    StMaryHall,
    StPeters,
    Somerville,
    Trinity,
    University,
    Wadham,
    Westminster,
    Wolfson,
    Worcester,
}
