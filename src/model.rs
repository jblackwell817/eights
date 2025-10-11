use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub type Position = u8; // Ranking on the river
pub type Year = u16;

/// The result for a particular year for either men's or womens' eights
#[derive(Serialize, Deserialize, Debug)]
pub struct AnnualResult {
    year: Year,
    standings: Vec<Crew>,
}

impl AnnualResult {
    pub fn new(year: Year, standings: Vec<Crew>) -> Self {
        Self { year, standings }
    }

    pub fn year(&self) -> Year {
        self.year
    }

    pub fn standings(&self) -> &Vec<Crew> {
        &self.standings
    }
}

/// The result for a particular crew over time
#[derive(Serialize, Deserialize, Debug)]
pub struct CrewResult {
    crew: Crew,
    results: Vec<(Year, Position)>,
}

impl CrewResult {
    pub fn new(crew: Crew, results: Vec<(Year, Position)>) -> Self {
        Self { crew, results }
    }

    pub fn crew(&self) -> Crew {
        self.crew
    }

    pub fn results(&self) -> Vec<(Year, Position)> {
        self.results.clone()
    }
}

/// A crew is defined by their college and what boat they were in (1st, 2nd, etc)
#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Crew {
    college: College,
    boat: u8,
}

impl Crew {
    pub fn new(college: College, boat: u8) -> Self {
        Self { college, boat }
    }

    pub fn college(&self) -> College {
        self.college
    }

    pub fn boat(&self) -> u8 {
        self.boat
    }
}

/// The colleges which have participated in eights. Some of these (e.g. Westminster) no
/// longer exist as colleges and some (e.g. BrasenoseStPeters) are combination teams.
/// This is also not an exhaustive list of all Oxford colleges, just those which have
/// participated in eights according to the data we have.
#[derive(Serialize, Deserialize, Debug, EnumString, Display, PartialEq, Copy, Clone)]
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
