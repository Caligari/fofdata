use std::fmt::Display;
use binrw::{binread, BinRead};

use crate::fof9_utility::FixedString;

#[binread]
#[derive(Debug)]
#[br(magic = b"\x0c\0\0\0STRUCTPLAYER")]
pub struct AltPlayers9Header {
    data_version: u32,
    max_player_id: u32,
}

impl AltPlayers9Header {
    pub fn max_player_id ( &self ) -> u32 {
        self.max_player_id
    }
}

#[derive(BinRead, Debug)]
pub struct AltPlayer9Id {
    player_id: u32,
}

impl AltPlayer9Id {
    pub fn player_id ( &self ) -> u32 {
        self.player_id
    }
}

#[binread]
#[derive(Debug)]
pub struct AltPlayer9Data {
    firstname: FixedString,
    nickname: FixedString,
    lastname: FixedString,
    position: PlayerPosition9,
    position_group: PlayerPositionGroup9,

    #[br(temp)]
    _some_1: u32,

    years_experience: u32,

    #[br(temp, count = 152)]
    _data_1: Vec<u32>,

    // #[br(temp)]
    some1_count: u32,
    #[br(temp, count = some1_count)]
    _some1: Vec<SomeData2>,

    // #[br(temp)]
    some2_count: u32,
    #[br(temp, count = some2_count)]
    _some2: Vec<SomeData3>,

    // #[br(temp)]
    some3_count: u32,
    #[br(temp, count = some3_count)]
    _some3: Vec<SomeData3>,

    // #[br(temp)]
    some4_count: u32,
    #[br(temp, count = some4_count)]
    _some4: Vec<SomeData3>,

    // #[br(temp)]
    some5_count: u32,
    #[br(temp, count = some5_count)]
    _some5: Vec<SomeData3>,

    // #[br(temp)]
    past_count: u32,
    #[br(temp, count = past_count)]
    _something_1: Vec<SomeData>,

    // #[br(temp)]
    current_count: u32,
    #[br(temp, count = current_count)]
    _something_2: Vec<SomeData>,

    #[br(count = 3)]
    _what: Vec<u32>,

    #[br(count = 3)]
    _overall: Vec<RelativeStats9>,

    #[br(count = 48)]
    _data_3: Vec<u32>,

    another_count: u32,
    #[br(count = another_count)]
    _data_4: Vec<SomeData4>,
}

impl AltPlayer9Data {
    pub fn position_group ( &self ) -> PlayerPositionGroup9 {
        self.position_group
    }

    pub fn position ( &self ) -> PlayerPosition9 {
        self.position
    }

    pub fn name ( &self ) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

#[binread]
#[derive(Debug)]
#[br(magic = b"\x0c\0\0\0STRUCTPLAYER")]
pub struct Players9Data {
    data_version: u32,

    #[br(temp)]
    player_count: u32,
    // #[br(parse_with = until_exclusive(|player: &Player9Data| matches!(player.data, PresentPlayer9::Gone)))]
    #[br(count = player_count)]
    players: Vec<Player9Data>,

    #[br(temp)]
    next_1_count: u32,
    #[br(count = next_1_count)]
    next_1: Vec<NextData9>,

    #[br(temp)]
    next_2_count: u32,
    #[br(count = next_2_count)]
    next_2: Vec<NextData9>,

    #[br(temp)]
    next_3_count: u32,
    #[br(count = next_3_count)]
    next_3: Vec<NextData9>,

    #[br(temp)]
    next_4_count: u32,
    #[br(count = next_4_count)]
    next_4: Vec<NextData9>,

    #[br(temp)]
    more_1_count: u32,
    #[br(count = more_1_count)]
    more_1: Vec<MoreData9>,

    #[br(temp)]
    staff_count: u32,
    #[br(count = staff_count)]
    staff: Vec<StaffData9>,
}

impl Players9Data {
    pub fn players ( &self ) -> &Vec<Player9Data> {
        &self.players
    }

    pub fn max_player_id ( &self ) -> u32 {
        self.players.len() as u32
    }

    pub fn staff ( &self ) -> &Vec<StaffData9> {
        &self.staff
    }
}

#[binread]
#[derive(Debug)]
pub struct NextData9 {
    #[br(count = 147)]
    stuff: Vec<u32>
}

#[binread]
#[derive(Debug)]
pub struct MoreData9 {
    #[br(count = 53)]
    stuff: Vec<u32>
}

#[binread]
#[derive(Debug)]
pub struct StaffData9 {
    // #[br(dbg)]
    staff_id: u32,
    firstname: FixedString,
    lastname: FixedString,

    #[br(count = 41)]
    stuff_1: Vec<u32>,

    #[br(temp)]
    list_count: u32,
    #[br(count = list_count)]
    list: Vec<StaffListItem9>,

    #[br(temp)]
    last_count: u32,
    #[br(count = last_count)]
    last_thing: Vec<StaffSmall1>,

    #[br(count = 9)]
    stuff_2: Vec<u32>,

}

impl StaffData9 {
    pub fn staff_id ( &self ) -> u32 {
        self.staff_id
    }

    pub fn position ( &self ) -> &str {
        "Staff"
    }

    pub fn name ( &self ) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

impl Display for StaffData9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {} {}", self.staff_id,
        self.position(),
        self.firstname, self.lastname)
    }
}

#[binread]
#[derive(Debug)]
pub struct StaffSmall1 {
    #[br(count = 2)]
    stuff: Vec<u32>,
}

#[binread]
#[derive(Debug)]
pub struct StaffListItem9 {
    #[br(count = 7)]
    stuff: Vec<u32>,
}

#[binread]
#[derive(Debug)]
pub struct Player9Data {
    // #[br(dbg)]
    player_id: u32,
    firstname: FixedString,
    middlename: FixedString,
    lastname: FixedString,

    position: PlayerPosition9,
    position_group: PlayerPositionGroup9,
    some_1: u32,
    years_experience: u32,

    #[br(count = 150)]
    data_1: Vec<u32>,

    year_1: u32,
    year_2: u32,

    #[br(temp)]
    some1_count: u32,
    #[br(count = some1_count)]
    some1: Vec<SomeData2>,

    #[br(temp)]
    some2_count: u32,
    #[br(count = some2_count)]
    some2: Vec<SomeData3>,

    #[br(temp)]
    some3_count: u32,
    #[br(count = some3_count)]
    some3: Vec<SomeData3>,

    #[br(temp)]
    some4_count: u32,
    #[br(count = some4_count)]
    some4: Vec<SomeData3>,

    #[br(temp)]
    some5_count: u32,
    #[br(count = some5_count)]
    some5: Vec<SomeData3>,

    #[br(temp)]
    past_count: u32,
    #[br(count = past_count)]
    something_1: Vec<SomeData>,

    #[br(temp)]
    current_count: u32,
    #[br(count = current_count)]
    something_2: Vec<SomeData>,

    what_1: u32,
    what_2: u32,
    what_3: u32,

    overall_1: RelativeStats9,
    overall_2: RelativeStats9,
    overall_3: RelativeStats9,

    #[br(count = 48)]
    data_3: Vec<u32>,

    #[br(temp)]
    another_count: u32,
    #[br(count = another_count)]
    data_4: Vec<SomeData4>,
}

impl Player9Data {
    pub fn player_id ( &self ) -> u32 {
        self.player_id
    }

    pub fn position_group ( &self ) -> PlayerPositionGroup9 {
        self.position_group
    }

    pub fn position ( &self ) -> PlayerPosition9 {
        self.position
    }

    pub fn name ( &self ) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

impl Display for Player9Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}|{}, {}, {} {}", self.player_id,
        self.position, self.position_group,
        self.years_experience,
        self.firstname, self.lastname)
    }
}

// #[binread]
// #[derive(Debug)]
// pub struct GonePlayerData9 {
//     some_year: u32,
//     #[br(count = 7)]
//     some_data: Vec<u32>,
// }


// Might be Staff??

// #[binread]
// #[derive(Debug)]
// pub struct InactivePlayerData9 {
//     #[br(count = 41)]
//     stuff: Vec<u32>,

//     some_count: u32,
//     #[br(count = some_count)]
//     history: Vec<PastDat9>,

//     #[br(count = 10)]
//     stuff_2: Vec<u32>,
// }

// #[derive(BinRead, Debug)]
// pub struct PastDat9 {
//     #[br(count = 7)]
//     stuff: Vec<u32>,
// }

#[derive(BinRead, Debug)]
pub struct SomeData {
    year: u32,
    something: u32,
}

#[derive(BinRead, Debug)]
pub struct SomeData2 {
    year: u32,
    some_1: u32,
    some_2: u32,
    some_3: u32,
}

#[derive(BinRead, Debug)]
pub struct SomeData3 {
    some_1: u32,
    some_2: u32,
    some_3: u32,
}

#[derive(BinRead, Debug)]
pub struct SomeData4 {
    #[br(count = 52)]
    data: Vec<u32>,
}

#[derive(BinRead, Debug)]
pub struct RelativeStats9 {
    #[br(count = 64)]
    stats: Vec<u32>,
}

#[derive(BinRead, Clone, Copy, Debug)]
pub enum PlayerPosition9 {
    #[br(magic = 1u32)] QB,
    #[br(magic = 2u32)] RB,
    #[br(magic = 3u32)] FB,
    #[br(magic = 4u32)] TE,
    #[br(magic = 5u32)] FL,
    #[br(magic = 6u32)] SE,
    #[br(magic = 7u32)] LT,
    #[br(magic = 8u32)] LG,
    #[br(magic = 9u32)] C,
    #[br(magic = 10u32)] RG,
    #[br(magic = 11u32)] RT,
    #[br(magic = 12u32)] P,
    #[br(magic = 13u32)] K,
    #[br(magic = 14u32)] LDE,
    #[br(magic = 15u32)] LDT,
    #[br(magic = 16u32)] NT,
    #[br(magic = 17u32)] RDT,
    #[br(magic = 18u32)] RDE,
    #[br(magic = 19u32)] SLB,
    #[br(magic = 20u32)] SILB,
    #[br(magic = 21u32)] MLB,
    #[br(magic = 22u32)] WILB,
    #[br(magic = 23u32)] WLB,
    #[br(magic = 24u32)] LCB,
    #[br(magic = 25u32)] RCB,
    #[br(magic = 26u32)] SS,
    #[br(magic = 27u32)] FS,
    #[br(magic = 28u32)] LS,
}

impl Display for PlayerPosition9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PlayerPosition9::*;
        write!(f, "{}", match self {
            QB => "QB",
            RB => "RB",
            FB => "FB",
            TE => "TE",
            FL => "FL",
            SE => "SE",
            LT => "LT",
            LG => "LG",
            C => "C",
            RG => "RG",
            RT => "RT",
            P => "P",
            K => "K",
            RDE => "RDE",
            RDT => "RDT",
            NT => "NT",
            LDT => "LDT",
            LDE => "LDE",
            SLB => "SLB",
            SILB => "SILB",
            MLB => "MLB",
            WILB => "WILB",
            WLB => "WLB",
            LCB => "LCB",
            RCB => "RCB",
            SS => "SS",
            FS => "FS",
            LS => "LS",
        })
    }
}

#[derive(BinRead, Clone, Copy, Debug)]
pub enum PlayerPositionGroup9 {
    #[br(magic = 1u32)] QB,
    #[br(magic = 2u32)] RB,
    #[br(magic = 3u32)] FB,
    #[br(magic = 4u32)] TE,
    #[br(magic = 5u32)] WR,
    #[br(magic = 6u32)] C,
    #[br(magic = 7u32)] OG,
    #[br(magic = 8u32)] OT,
    #[br(magic = 9u32)] P,
    #[br(magic = 10u32)] K,
    #[br(magic = 11u32)] DE,
    #[br(magic = 12u32)] DT,
    #[br(magic = 13u32)] ILB,
    #[br(magic = 14u32)] OLB,
    #[br(magic = 15u32)] CB,
    #[br(magic = 16u32)] S,
    #[br(magic = 17u32)] LS,
}

impl Display for PlayerPositionGroup9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PlayerPositionGroup9::*;
        write!(f, "{}", match self {
            QB => "QB",
            RB => "RB",
            FB => "FB",
            TE => "TE",
            WR => "WR",
            OT => "OT",
            OG => "OG",
            C => "C",
            P => "P",
            K => "K",
            DE => "DE",
            DT => "DT",
            OLB => "OLB",
            ILB => "ILB",
            CB => "CB",
            S => "S",
            LS => "LS",
        })
    }
}

