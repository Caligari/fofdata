use std::fmt::Debug;
use binrw::{BinRead, BinWrite, helpers::until_eof};
use num_traits::FromPrimitive;

use crate::fof9_utility::FixedString;


#[derive(BinRead, BinWrite, Debug)]
#[brw(magic = b"\x0c\0\0\0STRUCTLEAGUE")]  // what is the Z? and three NULLs?
pub struct League9Data {
    data_version: u32,  // ?
    some2: u32,  // ?
    some3: u32,  // ? null
    some4: u32,  // next action?
    some5: u32,  // current day?

    #[bw(map = |_| u32::from_usize(calendar.len()).unwrap())]
    calendar_length: u32,
    #[br(count = calendar_length)]
    calendar: Vec<CalendarItem>,

    pre_1: u32,  // year?

    unknown1: u32,
    pub number_teams: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32, // was #[br(magic = 0x7e7u32)]
    pub number_divisions: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: u32,
    unknown10: u32,
    unknown11: u32,
    unknown12: u32,

    pub championship_name: FixedString,
    pub league_name: FixedString,
    unknown15: u32,
    pub conference1_name: FixedString,
    pub conference1_short: FixedString,
    pub conference2_name: FixedString,
    pub conference2_short: FixedString,

    #[br(count = 8)]
    pub divisions: Vec<DivisionInfo>,
    pub structure_name: FixedString,
    unknown20: u32,
    unknown21: u32,
    unknown22: u32,  // is this a count for something? or an id num? or a bool (there is a custom calendar)?
    pub calendar_path: FixedString,

    #[br(count = 10)]
    ignored1: Vec<u32>,

    // count_b: u32,  // 2248
    // #[br(count = count_b)]
    // data_b: Vec<ItemB>,  // 112,400?
    #[br(count = 112474)]  // was 112996; // was pad_before = 0x6e590
    pad2: Vec<u32>,

    #[br(count = 522)]
    pad3: Vec<u32>,

    pub teams_len: u32,
    #[br(count = teams_len)]
    pub teams: Vec<TeamInfo>,

    // #[br(count = 27216)]
    #[br(parse_with = until_eof)]  // TODO: understand this final section
    pad4: Vec<u32>,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct ItemB {  // !! needs to be 50 u32 items
    data1:u32,
    data2:u32,
    data3:u32,
    data4:u32,
    data5:u32,
    data6:u32,
    data7:u32,
    data8:u32,
    data9:u32,
    data10:u32,
    data11:u32,
    data12:u32,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct CalendarItem {
    pub number: u32,
    pub month: u32,
    pub day: u32,
    pub year: u32,
    pub some1: u32,  // counting up?
    pub some2: u32,  // sort of counting?
    pub some3: u32,
    pub some4: u32,
    pub some5: u32,
    pub some6: u32,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct TeamInfo {
    #[bw(map = |n| n+1)]
    #[br(map = |n:u32| n-1)]
    pub team_number: u32,

    pub team_city: FixedString,
    pub team_name: FixedString,
    pub team_short: FixedString,

    #[br(count = 29)]
    data1: Vec<u32>,

    #[br(count = 200)]
    pub playbook: Vec<PlaybookPlayInfo>,

    #[br(count = 113462)]  // was pad_before = 0x6ecd8
    pad1: Vec<u32>,

    #[br(count = 6)]
    data2: Vec<u32>,

    pub team_city2: FixedString,
    pub team_name2: FixedString,
    pub team_short2: FixedString,

    #[br(count = 56)]
    data3: Vec<u32>,

    #[br(count = 1047)]  // was pad_before = 0x105c
    pad2: Vec<u32>,

    #[br(count = 5)]
    data4: Vec<u32>,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct PlaybookPlayInfo {
    #[br(count = 18)]
    data: Vec<u32>,
    pub play_name: FixedString,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct DivisionInfo {
    pub division_name: FixedString,
    pub number_teams: u32,
}
