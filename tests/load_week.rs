use fofdata::{LeagueInfo, Game9Section, Game9Data,};
use log::{info, debug, error};

mod common;

#[test]
fn load_week ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    const LEAGUE_NAME: &str = "New_Trial";
    const YEAR_SELECTION: usize = 0;
    const WEEK: u8 = 1;

    let mut done = true;

    let mut league_info = fofdata::find_leagues_9();

    if let Some(league) = league_info.get_mut(LEAGUE_NAME) {
        info!("processing league: {}", LEAGUE_NAME);
        league.load_league_data();

        if let Some(year) = league.get_year(YEAR_SELECTION) {
            if let Some(weeks_list) = league.get_weeks_list_for_year(year) {
                if !weeks_list.is_empty() {
                    if let Some(week) = league.get_week(year, WEEK) {
                        info!("loaded week 1 for year 0 ({}) in league {}", year, LEAGUE_NAME);
                        debug!("there are {} games in the week", week.games.len());
                        for game in week.games.iter() {
                            show_game(game);
                        }
                    } else {
                        error!("unable to load week 1 for year 0 ({}) in league {}", year, LEAGUE_NAME);
                        done = false;
                    }
                } else {
                    error!("no weeks found for year 0 ({}) in league {}", year, LEAGUE_NAME);
                    done = false;
                }
            } else {
                error!("unable to find weeks in year 0 ({}) in league {}", year, LEAGUE_NAME);
                done = false;
            }
        } else {
            error!("unable to find year 0 in league {}", LEAGUE_NAME);
            done = false;
        }
    } else {
        error!("unable to find league {}", LEAGUE_NAME);
        done = false;
    }

    assert!(done);
}

#[test]
fn load_all_weeks ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    const YEAR_SELECTION: usize = 0;

    let mut done = true;

    let mut league_info = fofdata::find_leagues_9();

    for (league_name, league) in league_info.iter_mut() {
        info!("processing league: {}", league_name);
        league.load_league_data();

        if let Some(year) = league.get_year(YEAR_SELECTION) {
            if let Some(weeks_list) = league.get_weeks_list_for_year(year) {
                for week_num in weeks_list {
                    if let Some(week) = league.get_week(year, week_num) {
                        info!("loaded week {} for year 0 ({}) in league {}", week_num, year, league_name);
                        debug!("there are {} games in the week", week.games.len());
                        for game in week.games.iter() {
                            show_game(game);
                        }
                    } else {
                        error!("unable to load week {} for year 0 ({}) in league {}", week_num, year, league_name);
                        done = false;
                    }
                }
            } else {
                error!("unable to find weeks in year 0 ({}) in league {}", year, league_name);
                done = false;
            }
        } else {
            error!("unable to find year 0 in league {}", league_name);
            done = false;
        }
    }

    assert!(done);
}

fn show_game ( game: &Game9Data ) {
    for section in game.sections.iter() {
        match section {
            Game9Section::Start {..} => {
                debug!("starting game: {}", section);
            },
            Game9Section::Play {quarter, minutes_remaining, seconds_remaining, down, yards_to_go, yardline, off_team, play, ..} => {
                debug!("{}-{}-{} ({}Q: {}:{:02}), {} {}", down, yards_to_go, game.field_yardline(*yardline), quarter, minutes_remaining, seconds_remaining, game.team(*off_team).short(), play);
            },
            Game9Section::End {..} => {
                debug!("ending game");
            },
        }
    }

}
