use std::fmt::Display;
use num_traits::FromPrimitive;
use binrw::{BinRead, helpers::{until_eof, until}, binread};

use crate::{fof9_utility::FixedString, Position};

const NUM_BLITZERS: usize = 10;


#[derive(BinRead, Debug)]
pub struct Week9Data {
    // some number of games
    #[br(parse_with = until_eof)]
    pub games: Vec<Game9Data>,
}

#[derive(BinRead, Debug)]
#[br(assert(matches!(sections.first().unwrap(), Game9Section::Start{..})), assert(matches!(sections.last().unwrap(), Game9Section::End{..})))]
pub struct Game9Data {
    // begin, plays, end
    #[br(parse_with = until(|section| matches!(section, Game9Section::End{..})))]
    pub sections: Vec<Game9Section>,
}

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

#[binread]
#[derive(Debug)]
pub enum Game9Section {
    #[br(magic = b"\x0a\0\0\0BEGIN_GAME")] Start {
        data_version: u32,   // is this the version of the data?
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

#[derive(Debug)]
#[binread]
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
        formation: FormationData9,

        #[br(map = |val: u32| { assert!(val < 2); val == 1})]
        start_drive: bool,

        // #[br(temp)]
        num_blitz: u32,
        #[br(count = NUM_BLITZERS)]  // temp
        def_assign: Vec<DefensiveAssignment9>,

        #[br(calc = def_assign.iter().enumerate().filter_map(|(pos, assign)| if matches!(assign, DefensiveAssignment9::Blitz) { Some(pos) } else { None }).collect())]
        defensive_blitzers: Vec<usize>,
        #[br(calc = def_assign.iter().enumerate().filter_map(|(pos, assign)| if matches!(assign, DefensiveAssignment9::Spy) { Some(pos) } else { None }).collect())]
        defensive_spies: Vec<usize>,

        #[br(assert(num_blitz == u32::from_usize(defensive_blitzers.len()).unwrap()))]
        #[br(assert(formation.defensive_special.has_spy() == (defensive_spies.len() == 1)))]

        penalty: PenaltyInfo9,

        #[br(count = 7)]
        unknown: Vec<u32>,

        injury: InjuryInfo9,

        #[br(count = 373)]
        data: Vec<u32>,  // minutes left, seconds left, penalty player (on field?)
    },

    #[br(magic = 6u32)] Pass {
        formation: FormationData9,

        #[br(map = |val: u32| { assert!(val < 2); val == 1})]
        start_drive: bool,

        // #[br(temp)]
        num_blitz: u32,
        #[br(count = NUM_BLITZERS)]  // temp
        def_assign: Vec<DefensiveAssignment9>,

        #[br(calc = def_assign.iter().enumerate().filter_map(|(pos, assign)| if matches!(assign, DefensiveAssignment9::Blitz) { Some(pos) } else { None }).collect())]
        defensive_blitzers: Vec<usize>,
        #[br(calc = def_assign.iter().enumerate().filter_map(|(pos, assign)| if matches!(assign, DefensiveAssignment9::Spy) { Some(pos) } else { None }).collect())]
        defensive_spies: Vec<usize>,

        #[br(assert(num_blitz == u32::from_usize(defensive_blitzers.len()).unwrap()))]
        #[br(assert(formation.defensive_special.has_spy() == (defensive_spies.len() == 1)))]

        penalty: PenaltyInfo9,

        #[br(count = 7)]
        unknown: Vec<u32>,

        injury: InjuryInfo9,

        #[br(count = 373)]
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

            GamePlay9::Run { formation, defensive_blitzers, defensive_spies, penalty, injury, data, .. } => {
                write!(f, "{}", display_play_common("Run", formation, defensive_blitzers, defensive_spies, penalty, injury, data))
            },

            GamePlay9::Pass { formation, defensive_blitzers, defensive_spies, penalty, injury, data, .. } => {
                write!(f, "{}", display_play_common("Pass", formation, defensive_blitzers, defensive_spies, penalty, injury, data))
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

fn display_play_common ( play_type: &str, formation: &FormationData9, defensive_blitzers: &Vec<usize>, defensive_spies: &Vec<usize>, penalty: &PenaltyInfo9, injury: &InjuryInfo9, data: &[u32] ) -> String {
    format!("{} ({}{}{}{}{}){}", play_type,
        formation,
        if defensive_spies.is_empty() { "".to_string() } else {
            format_args!(", {} Spy", formation.blitz_position(*defensive_spies.first().unwrap())).to_string()
        },
        if defensive_blitzers.is_empty() {
            "".to_string()
        } else {
            let first = ", Blitz:".to_string() + &formation.blitz_position(*defensive_blitzers.first().unwrap());
            let ret = if defensive_blitzers.len() > 1 {
                defensive_blitzers[1..].iter().fold(first, |acc, next| acc + "," + &formation.blitz_position(*next))
            } else { first };
            ret
        },
        penalty,
        injury,
        format_args!(" {:?}", &data[0..7]),
    )
}

#[derive(BinRead, Debug)]
pub struct FormationData9 {
    offensive_formation: OffensiveFormation9,
    offensive_personnel: OffensivePersonnel9,
    def_team: u32,  // 0 or 1
    defensive_personnel: DefensivePersonnel9,
    defensive_coverage: DefensiveCoverage9,
    defensive_front: DefensiveFront9,
    defensive_special: SpecialCoverage9,
}

impl FormationData9 {
    fn blitz_position ( &self, blitzer_number: usize ) -> String {
        use DefensiveFront9::*;
        use DefensivePersonnel9::*;
        use Position::*;

        let blitzer_number = blitzer_number.min(NUM_BLITZERS-1) - 1;

        if let Some(list) = match (self.defensive_front, self.defensive_personnel) {
            (True34 | Eagle34, Man) => Some([DRE, SLB, SILB, WILB, WLB, LCB, RCB, SS, FS]),
            (True34 | Eagle34, Nickel) => Some([DRE, SLB, SILB, WLB, LCB, RCB, NB, SS, FS]),
            (True34 | Eagle34, Dime) => Some([DRE, SLB, WLB, LCB, RCB, NB, DB, SS, FS]),

            (Over43 | Under43, Man) => Some([DLE, DRE, SLB, MLB, WLB, LCB, RCB, SS, FS]),
            (Over43 | Under43, Nickel) => Some([DLE, DRE, MLB, WLB, LCB, RCB, NB, SS, FS]),
            (Over43 | Under43, Dime) => Some([DLE, DRE, MLB, LCB, RCB, NB, DB, SS, FS]),

            _ => None,
        } {
            format!("{:?}", list[blitzer_number])
        } else { blitzer_number.to_string() }
    }
}

impl Display for FormationData9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} | {} {} {}", // {}",
            self.offensive_formation, self.offensive_personnel,
            self.defensive_front, self.defensive_personnel, self.defensive_coverage, // self.defensive_special,
        )
    }
}

#[derive(BinRead, Debug)]
pub struct PenaltyInfo9 {
    #[br(map = |val: u32| { assert!(val < 2); val == 1})]
    defensive_penalty: bool,
    #[br(map = |val: u32| { assert!(val < 2); val == 1})]
    offensive_penalty: bool,

    penalty_yards: u32,

    #[br(map = |val: u32| { assert!(val < 2); val == 1})]
    kicking_play: bool,  // ??

    #[br(map = |val: u32| { assert!(val < 2); val == 1})]
    loss_of_down: bool,  // verify

    #[br(count = 4)]
    something: Vec<u32>,

    accept_yardline: u32,
    accept_down: u32,
    accept_yards_to_go: u32,

    decline_yardline: u32,
    decline_down: u32,
    decline_yards_to_go: u32,

    #[br(count = 4)]
    something2: Vec<u32>,

    penalty_type: u32,  // (1 = false start, 7 = off pass interference?)
}

impl PenaltyInfo9 {
    pub fn is_penalty ( &self ) -> bool {
        self.defensive_penalty || self.offensive_penalty
    }
}

impl Display for PenaltyInfo9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_penalty() {
            write!(f, ", {}[{}] => {}-{}/{}",
                match (self.defensive_penalty, self.offensive_penalty) {
                    (true, false) => "def pen",
                    (false, true) => "off pen",
                    (true, true) => "off/def pen",
                    _ => unreachable!(),
                },
                self.penalty_type,
                self.accept_down,
                self.accept_yards_to_go,
                self.accept_yardline,
            )
        } else {
            write!(f, "")
        }
    }
}

#[derive(BinRead, Debug)]
pub struct InjuryInfo9 {
    injury: u32,
    player: u32,
}

impl Display for InjuryInfo9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.injury == 0 && self.player == 0 {
            write!(f, "")
        } else {
            write!(f, " injury {}/{}", self.player, self.injury)
        }
    }
}

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

#[derive(BinRead, Debug)]
pub struct PassStats9 {
    screen: PassPlayStats9,
    short: PassPlayStats9,
    medium: PassPlayStats9,
    long: PassPlayStats9,
    other: PassPlayStats9,
}

#[derive(BinRead, Debug)]
pub struct PassPlayStats9 {
    attempts: u32,
    completions: u32,
    yards: i32,
}

#[derive(BinRead, Debug)]
pub struct RunStats9 {
    left: RunPlayStats9,
    middle: RunPlayStats9,
    right: RunPlayStats9,
    none: RunPlayStats9,
}

#[derive(BinRead, Debug)]
pub struct RunPlayStats9 {
    attempts: u32,
    yards: i32
}

#[derive(BinRead, Debug)]
pub struct PossessionStats9 {
    seconds: u32,
    red_zone_attempts: u32,
    red_zone_td: u32,
    red_zone_fg: u32,
}

#[derive(BinRead, Debug)]
pub struct Attendance9 {
    attendance: u32,
    capacity: u32,
}

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

#[derive(BinRead, Debug, Clone, Copy)]
pub enum DefensivePersonnel9 {
	#[br(magic = 0u32)] Man,
	#[br(magic = 1u32)] Nickel,
	#[br(magic = 2u32)] Dime,
	#[br(magic = 3u32)] Prevent,
	#[br(magic = 4u32)] GoalLine,
}

impl Display for DefensivePersonnel9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DefensivePersonnel9::Man => "Man",  // Man to Man
            DefensivePersonnel9::Nickel => "Nickel",
            DefensivePersonnel9::Dime => "Dime",
            DefensivePersonnel9::Prevent => "Prevent",
            DefensivePersonnel9::GoalLine => "GoalLine",
        })
    }
}


#[derive(BinRead, Debug)]
pub enum DefensiveCoverage9 {
	#[br(magic = 0u32)] Zero,
	#[br(magic = 1u32)] One,
	#[br(magic = 2u32)] Two,
	#[br(magic = 3u32)] Three,
	#[br(magic = 4u32)] Four,
	#[br(magic = 5u32)] Five,
	#[br(magic = 6u32)] Six,
	#[br(magic = 7u32)] Seven,
	#[br(magic = 8u32)] Eight,
	#[br(magic = 9u32)] Nine,
	#[br(magic = 10u32)] Ten,
	#[br(magic = 11u32)] Eleven,
	#[br(magic = 12u32)] Twelve,
	#[br(magic = 13u32)] Thirteen,
	#[br(magic = 14u32)] Fourteen,
	#[br(magic = 15u32)] Fifteen,
}

impl Display for DefensiveCoverage9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DefensiveCoverage9::Zero => "Cover 0 Press",
            DefensiveCoverage9::One => "Cover 0",
            DefensiveCoverage9::Two => "Cover 1 Press",
            DefensiveCoverage9::Three => "Cover 1",
            DefensiveCoverage9::Four => "Cover 1 Hole/Press",
            DefensiveCoverage9::Five => "Cover 2 Zone",
            DefensiveCoverage9::Six => "Cover 2 Press",
            DefensiveCoverage9::Seven => "Cover 2",
            DefensiveCoverage9::Eight => "Tampa 2 Zone",
            DefensiveCoverage9::Nine => "Cover 3 Sky Zone",
            DefensiveCoverage9::Ten => "Cover 3 Cloud Zone",
            DefensiveCoverage9::Eleven => "Cover 3 Buzz Zone",
            DefensiveCoverage9::Twelve => "Cover 4 Zone",
            DefensiveCoverage9::Thirteen => "Cover 4 Match Press",
            DefensiveCoverage9::Fourteen => "Cover 4 Match",
            DefensiveCoverage9::Fifteen => "Cover 6 Zone",
        })
    }
}

#[derive(BinRead, Debug, Clone, Copy)]
pub enum DefensiveFront9 {
	#[br(magic = 0u32)] True34,
	#[br(magic = 1u32)] Eagle34,
	#[br(magic = 2u32)] Under43,
	#[br(magic = 3u32)] Over43,
}

impl Display for DefensiveFront9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DefensiveFront9::True34 => "True 34",
            DefensiveFront9::Eagle34 => "Eagle 34",
            DefensiveFront9::Under43 => "43 Under",
            DefensiveFront9::Over43 => "43 Over",
        })
    }
}

#[derive(BinRead, Debug)]
pub enum SpecialCoverage9 {
	#[br(magic = 0u32)] None,
	#[br(magic = 1u32)] Spy,
	#[br(magic = 10u32)] What10,
	#[br(magic = 11u32)] What11,
    #[br(magic = 20u32)] What20,
    #[br(magic = 21u32)] What21,
    #[br(magic = 30u32)] What30,
    #[br(magic = 31u32)] What31,
    #[br(magic = 40u32)] What40,
    #[br(magic = 41u32)] What41,
    #[br(magic = 50u32)] What50,
    #[br(magic = 51u32)] What51,
}

impl SpecialCoverage9 {
    pub fn has_spy ( &self ) -> bool {
        matches!(self, SpecialCoverage9::Spy | SpecialCoverage9::What11 | SpecialCoverage9::What21 | SpecialCoverage9::What31 |
            SpecialCoverage9::What41 | SpecialCoverage9::What51)
    }
}

impl Display for SpecialCoverage9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            SpecialCoverage9::None => "",
            SpecialCoverage9::Spy => "Spy",
            SpecialCoverage9::What10 => "DoubleTop",
            SpecialCoverage9::What11 => "DoubleTop Spy/11",
            SpecialCoverage9::What20 => "DoubleTop/20",
            SpecialCoverage9::What21 => "DoubleTop Spy/21",
            SpecialCoverage9::What30 => "DoubleTop/30",
            SpecialCoverage9::What31 => "DoubleTop Spy/31",
            SpecialCoverage9::What40 => "DoubleTop/40",
            SpecialCoverage9::What41 => "DoubleTop Spy/41",
            SpecialCoverage9::What50 => "DoubleTop/50",
            SpecialCoverage9::What51 => "DoubleTop Spy/51",
        })
    }
}

#[derive(BinRead, Debug)]
pub enum DefensiveAssignment9 {
    #[br(magic = 0u32)] Normal,
    #[br(magic = 1u32)] Blitz,
    #[br(magic = 2u32)] Spy,
}

impl Display for DefensiveAssignment9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DefensiveAssignment9::Normal => "",
            DefensiveAssignment9::Blitz => "Blitz",
            DefensiveAssignment9::Spy => "Spy",
        })
    }
}
