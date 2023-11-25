use std::fmt::Display;
use num_traits::FromPrimitive;
use binrw::{BinRead, helpers::{until_eof, until}, binread};

use crate::fof9_utility::FixedString;


#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct Week9Data {
    // some number of games
    #[br(parse_with = until_eof)]
    pub games: Vec<Game9Data>,
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
#[br(assert(matches!(sections.first().unwrap(), Game9Section::Start{..})), assert(matches!(sections.last().unwrap(), Game9Section::End{..})))]
pub struct Game9Data {
    // begin, plays, end
    #[br(parse_with = until(|section| matches!(section, Game9Section::End{..})))]
    pub sections: Vec<Game9Section>,
}

#[allow(dead_code)]
impl Game9Data {
    pub fn home_team ( &self ) -> &WeekTeamInfo9 {
        if let Game9Section::Start{home_team, ..} = self.sections.first().unwrap() {
            home_team
        } else {
            panic!("first section of game is not start");
        }
    }

    pub fn away_team ( &self ) -> &WeekTeamInfo9 {
        if let Game9Section::Start{away_team, ..} = self.sections.first().unwrap() {
            away_team
        } else {
            panic!("first section of game is not start");
        }
    }

    pub fn team ( &self, game_team: u32 ) -> &WeekTeamInfo9 {
        assert!(game_team < 2);
        if let Game9Section::Start{home_team, away_team, ..} = self.sections.first().unwrap() {
            [home_team, away_team][usize::from_u32(game_team).unwrap()]
        } else {
            panic!("first section of game is not start");
        }
    }

    pub fn field_yardline ( &self, yards: u32 ) -> String {
        let (showteam, showyards) = if yards > 50 { (0, 50-(yards-50)) } else { (1, yards) };
        format!("{}{:02}", self.team(showteam).short(), showyards)
    }
}

#[allow(dead_code)]
#[binread]
#[derive(Debug)]
pub enum Game9Section {
    #[br(magic = b"\x0a\0\0\0BEGIN_GAME")] Start {
        version: u32,   // is this the version of the data?
        year: u32,
        current_week: u32,
        exhibition_weeks: u32,
        regular_and_ex_weeks: u32,
        full_season_weeks: u32,
        location: FixedString,
        when: FixedString,

        total_attendance: u32,
        no_shows: u32,
        upperdeck: Attendance9,
        endzone: Attendance9,
        mezzanine: Attendance9,
        sidelines: Attendance9,
        club: Attendance9,
        boxes: Attendance9,
        something1: u32,
        something2: u32,
        starting_temperature: u32,
        starting_weather: u32,  // (0=sunny, 1=partly cloudy)
        total_capacity: u32,
        windspeed_mph: u32,

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

        #[br(temp)]
        home_drive_len: u32,
        #[br(temp)]
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

            Game9Section::Play { quarter, minutes_remaining, seconds_remaining, off_team, down, yards_to_go, yardline, play, ..} => {
                write!(f, "{}-{}-{}_{} ({}Q: {}:{:02}), {}", down, yards_to_go, off_team, yardline, quarter, minutes_remaining, seconds_remaining, play)
            },

            Game9Section::End { .. } => {
                write!(f, "")
            },
        }
    }
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub enum GamePlay9 {
    #[br(magic = 1u32)] FieldGoal {
        #[br(count = 421)]
        data: Vec<u32>
    },

    #[br(magic = 2u32)] Kickoff {
        #[br(count = 421)]
        data: Vec<u32>
    },

    #[br(magic = 3u32)] OnsideKick {
        #[br(count = 421)]
        data: Vec<u32>
    },

    #[br(magic = 4u32)] Punt {
        #[br(count = 421)]
        data: Vec<u32>
    },

    #[br(magic = 5u32)] Run {
        offensive_formation: OffensiveFormation9,
        offensive_personnel: OffensivePersonnel9,
        #[br(count = 419)]
        data: Vec<u32>
    },

    #[br(magic = 6u32)] Pass {
        #[br(count = 421)]
        data: Vec<u32>
    },

    #[br(magic = 7u32)] Special {
        #[br(count = 294)]
        data1: Vec<u32>,

        extra_point: ExtraPointResult9,

        something8: u32,  // ?
        something9: u32,  // ?
        something10: u32,  // ?

        specialplay: SpecialPlay9,

        #[br(count = 116)]
        data2: Vec<u32>,
    },
}

impl Display for GamePlay9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GamePlay9::FieldGoal { .. } => {
                write!(f, "Field Goal")
            },

            GamePlay9::Kickoff { .. } => {
                write!(f, "Kickoff")
            },

            GamePlay9::OnsideKick { .. } => {
                write!(f, "Onside Kick")
            },

            GamePlay9::Punt { .. } => {
                write!(f, "Punt")
            },

            GamePlay9::Run { .. } => {
                write!(f, "Run")
            },

            GamePlay9::Pass { .. } => {
                write!(f, "Pass")
            },

            GamePlay9::Special { specialplay, extra_point, .. } => {
                // write!(f, "{}, {}  {}", specialplay, data1.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", "), data2.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", "))
                match specialplay {
                    SpecialPlay9::ExtraPoint => { write!(f, "{}: {}", specialplay, extra_point) },
                    _ => { write!(f, "{}", specialplay) }
                }
            },
        }
    }
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub enum SpecialPlay9 {
    #[br(magic = 0u32)] ExtraPoint,
    #[br(magic = 1u32)] HomeTimeout,
    #[br(magic = 2u32)] AwayTimeout,
    #[br(magic = 3u32)] TwoMinute,

    #[br(magic = 4u32)] HomeCoin,
    #[br(magic = 5u32)] AwayCoin,
    #[br(magic = 6u32)] UnknownSix,
    #[br(magic = 7u32)] UnknownSeven,

    #[br(magic = 8u32)] StartQ1,
    #[br(magic = 9u32)] StartQ2,
    #[br(magic = 10u32)] StartQ3,
    #[br(magic = 11u32)] StartQ4,

    #[br(magic = 12u32)] StartOT1,
    #[br(magic = 13u32)] StartOT2,
    #[br(magic = 14u32)] StartOT3,
    #[br(magic = 15u32)] StartOT4,
}

impl Display for SpecialPlay9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialPlay9::ExtraPoint => {
                write!(f, "Extra Point")
            },

            SpecialPlay9::HomeTimeout => {
                write!(f, "Home Timeout")
            },

            SpecialPlay9::AwayTimeout => {
                write!(f, "Away Timeout")
            },

            SpecialPlay9::TwoMinute => {
                write!(f, "Two Minute Warning")
            },

            SpecialPlay9::HomeCoin => {
                write!(f, "Coin Toss: Home")
            },

            SpecialPlay9::AwayCoin => {
                write!(f, "Coin Toss: Away")
            },

            SpecialPlay9::UnknownSix => {
                write!(f, "?Six")
            },

            SpecialPlay9::UnknownSeven => {
                write!(f, "?Seven")
            },

            SpecialPlay9::StartQ1 => {
                write!(f, "Start Q1")
            },

            SpecialPlay9::StartQ2 => {
                write!(f, "Start Q2")
            },

            SpecialPlay9::StartQ3 => {
                write!(f, "Start Q3")
            },

            SpecialPlay9::StartQ4 => {
                write!(f, "Start Q4")
            },

            SpecialPlay9::StartOT1 => {
                write!(f, "Start OT1")
            },

            SpecialPlay9::StartOT2 => {
                write!(f, "Start OT2")
            },

            SpecialPlay9::StartOT3 => {
                write!(f, "Start OT3")
            },

            SpecialPlay9::StartOT4 => {
                write!(f, "Start OT4")
            },

        }
    }
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub enum ExtraPointResult9 {
    // #[br(magic = b"\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] MissedKick,
    #[br(magic = b"\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] Kick,
    #[br(magic = b"\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] WideLeftKick,
    #[br(magic = b"\x01\0\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] LeftUprightKick,
    #[br(magic = b"\x01\0\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] WideRightKick,
    #[br(magic = b"\x01\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] RightUprightKick,
    #[br(magic = b"\x01\0\0\0\0\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] BlockedKick,
    #[br(magic = b"\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] FailedTwoPointRun,
    #[br(magic = b"\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0")] TwoPointRun,
    #[br(magic = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0")] FailedTwoPointPass,
    #[br(magic = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0")] TwoPointPass,
    #[br(magic = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")] Nothing,
}

impl Display for ExtraPointResult9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ExtraPointResult9::MissedKick => { write!(f, "Missed Kick") },
            ExtraPointResult9::Kick => { write!(f, "Good") },
            ExtraPointResult9::WideLeftKick => { write!(f, "Wide Left") },
            ExtraPointResult9::WideRightKick => { write!(f, "Wide Right") },
            ExtraPointResult9::RightUprightKick => { write!(f, "Hit Right Upright") },
            ExtraPointResult9::LeftUprightKick => { write!(f, "Hit Left Upright") },
            ExtraPointResult9::BlockedKick => { write!(f, "Blocked") },
            ExtraPointResult9::FailedTwoPointRun => { write!(f, "Failed Two Point Run") },
            ExtraPointResult9::TwoPointRun => { write!(f, "Two Point Run") },
            ExtraPointResult9::FailedTwoPointPass => { write!(f, "Failed Two Point Pass") },
            ExtraPointResult9::TwoPointPass => { write!(f, "Two Point Pass") },
            ExtraPointResult9::Nothing => { write!(f, "<Not Extra Point>") },
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

impl WeekTeamInfo9 {
    pub fn city ( &self ) -> String {
        self.city.to_string()
    }

    pub fn name ( &self ) -> String {
        self.name.to_string()
    }

    pub fn short ( &self ) -> String {
        self.short.to_string()
    }
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

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub struct Attendance9 {
    attendance: u32,
    capacity: u32,
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub enum OffensivePersonnel9 {
	#[br(magic = 0u32)] Op005,
	#[br(magic = 1u32)] Op014,
	#[br(magic = 2u32)] Op014t,
	#[br(magic = 3u32)] Op023,
	#[br(magic = 4u32)] Op104,
	#[br(magic = 5u32)] Op113,
	#[br(magic = 6u32)] Op113t,
	#[br(magic = 7u32)] Op122,
	#[br(magic = 8u32)] Op131,
	#[br(magic = 9u32)] Op203,
	#[br(magic = 10u32)] Op212,
	#[br(magic = 11u32)] Op221,
	#[br(magic = 12u32)] Op230,
}

impl Display for OffensivePersonnel9 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			OffensivePersonnel9::Op005 => "005",
			OffensivePersonnel9::Op014 => "014",
			OffensivePersonnel9::Op014t => "014t",
			OffensivePersonnel9::Op023 => "023",
			OffensivePersonnel9::Op104 => "104",
			OffensivePersonnel9::Op113 => "113",
			OffensivePersonnel9::Op113t => "113t",
			OffensivePersonnel9::Op122 => "122",
			OffensivePersonnel9::Op131 => "131",
			OffensivePersonnel9::Op203 => "203",
			OffensivePersonnel9::Op212 => "212",
			OffensivePersonnel9::Op221 => "221",
			OffensivePersonnel9::Op230 => "230",
		})
	}
}

#[allow(dead_code)]
#[derive(BinRead, Debug)]
pub enum OffensiveFormation9 {
	#[br(magic = 0u32)] Pro,
	#[br(magic = 1u32)] Weak,
	#[br(magic = 2u32)] Strong,
	#[br(magic = 3u32)] IForm,
	#[br(magic = 4u32)] Empty,
}

impl Display for OffensiveFormation9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OffensiveFormation9::Pro => "Pro",
            OffensiveFormation9::Weak => "Weak",
            OffensiveFormation9::Strong => "Strong",
            OffensiveFormation9::IForm => "I",
            OffensiveFormation9::Empty => "Empty Backfield",
        })
    }
}
