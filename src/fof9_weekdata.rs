use std::fmt::Display;

use binrw::{BinRead, helpers::until_eof};

use crate::fof9_utility::FixedString;


#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct Week9Data {
    // some number of games
    #[br(parse_with = until_eof)]
    pub sections: Vec<Game9Section>,
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct Game9Data {
    // begin, plays, end
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub enum Game9Section {
    #[br(magic = b"\x0a\0\0\0BEGIN_GAME")] Start {
        version: u32,   // is this the version of the data?
        year: u32,
        some3: u32,  // week?
        some4: u32,
        some5: u32,
        some6: u32,
        location: FixedString,
        when: FixedString,

        #[br(count = 20)]
        data2: Vec<u32>,

        home_team: WeekTeamInfo9,
        away_team: WeekTeamInfo9,

        end1: u32,
        end2: u32,
    },

    #[br(magic = b"\x09\0\0\0GAME_PLAY")] Play {
        quarter: u32,
        minutes_remaining: u32,
        seconds_remaining: u32,
        off_team: u32,  // 0 or 1
        down: u32,
        yards_to_go: u32,
        yardline: u32,
        home_timeouts: u32,
        away_timeouts: u32,
        play: GamePlay9,
    },

    #[br(magic = b"\x08\0\0\0END_GAME")] End {
        player_of_game: u32,
        home_drive_len: u32,
        away_drive_len: u32,

        #[br(count = home_drive_len)]
        home_drive: Vec<DriveInfo9>,

        #[br(count = away_drive_len)]
        away_drive: Vec<DriveInfo9>,

        home_pass_stats: PassStats9,
        away_pass_stats: PassStats9,

        home_run_stats: RunStats9,
        away_run_stats: RunStats9,

        home_possession: PossessionStats9,
        away_possession: PossessionStats9,
    }
}

impl Display for Game9Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game9Section::Start { location, when, home_team, away_team, .. } => {
                write!(f, "{}, {} vs {}, {}", when, home_team.city, away_team.city, location)
            },

            Game9Section::Play { quarter, minutes_remaining, seconds_remaining, off_team, down, yards_to_go, yardline, home_timeouts, away_timeouts, play } => {
                write!(f, "{}-{}-{}_{} ({}Q: {}:{}) {}", down, yards_to_go, off_team, yardline, quarter, minutes_remaining, seconds_remaining, play)
            },

            Game9Section::End { player_of_game, home_drive_len, away_drive_len, home_drive, away_drive, home_pass_stats, away_pass_stats, home_run_stats, away_run_stats, home_possession, away_possession } => {
                write!(f, "")
            },
        }
    }
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub enum GamePlay9 {
    #[br(magic = 1u32)] FieldGoal {
        #[br(count = 0x1a5)]
        data: Vec<u32>
    },

    #[br(magic = 2u32)] Kickoff {
        #[br(count = 0x1a5)]
        data: Vec<u32>
    },

    #[br(magic = 3u32)] OnsideKick {
        #[br(count = 0x1a5)]
        data: Vec<u32>
    },

    #[br(magic = 4u32)] Punt {
        #[br(count = 0x1a5)]
        data: Vec<u32>
    },

    #[br(magic = 5u32)] Run {
        #[br(count = 0x1a5)]
        data: Vec<u32>
    },

    #[br(magic = 6u32)] Pass {
        #[br(count = 0x1a5)]
        data: Vec<u32>
    },

    #[br(magic = 7u32)] Special {
        #[br(count = 0x1a5)]
        data: Vec<u32>
    },
}

impl Display for GamePlay9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GamePlay9::FieldGoal { data } => {
                write!(f, "Field Goal")
            },

            GamePlay9::Kickoff { data } => {
                write!(f, "Kickoff")
            },

            GamePlay9::OnsideKick { data } => {
                write!(f, "Onside Kick")
            },

            GamePlay9::Punt { data } => {
                write!(f, "Punt")
            },

            GamePlay9::Run { data } => {
                write!(f, "Run")
            },

            GamePlay9::Pass { data } => {
                write!(f, "Pass")
            },

            GamePlay9::Special { data } => {
                write!(f, "Special Play {} {} {} {} {} {} {} {}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7])
            },
        }
    }
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct WeekTeamInfo9 {
    number: u32,
    city: FixedString,
    name: FixedString,
    short: FixedString,

    #[br(count = 372)]
    other_data: Vec<u32>,
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct DriveInfo9 {
    start_quarter: u32,
	start_minutes: u32,
	start_seconds: u32,
	end_quarter: u32,
	end_minutes: u32,
	end_seconds: u32,
	start_yards_from_goal: u32,
	num_plays: u32,
	yards_gained: i32,
	result: u32,
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct PassStats9 {
    screen: PassPlayStats9,
    short: PassPlayStats9,
    medium: PassPlayStats9,
    long: PassPlayStats9,
    other: PassPlayStats9,
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct PassPlayStats9 {
    attempts: u32,
    completions: u32,
    yards: i32,
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct RunStats9 {
    left: RunPlayStats9,
    middle: RunPlayStats9,
    right: RunPlayStats9,
    none: RunPlayStats9,
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct RunPlayStats9 {
    attempts: u32,
    yards: i32
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct PossessionStats9 {
    seconds: u32,
    red_zone_attempts: u32,
    red_zone_td: u32,
    red_zone_fg: u32,
}
