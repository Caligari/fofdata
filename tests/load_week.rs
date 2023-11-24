use fofdata::{LeagueInfo, Game9Section, GamePlay9};
use log::{info, debug, warn, error};

mod common;

#[test]
fn load_week ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    const LEAGUE_NAME: &str = "New_Trial";
    let mut done = false;

    let mut league_info = fofdata::find_leagues_9();

    if let Some(league) = league_info.get_mut(LEAGUE_NAME) {
        info!("processing league: {}", LEAGUE_NAME);
        league.load_league_data();

        if let Some(year) = league.get_year(0) {
            if let Some(weeks_list) = league.get_weeks_list_for_year(year) {
                if !weeks_list.is_empty() {
                    if let Some(week) = league.get_week(year, 1) {
                        info!("loaded week 1 for year 0 ({}) in league {}", year, LEAGUE_NAME);
                        debug!("there are {} sections in the week", week.sections.len());
                        for section in week.sections.iter() {
                            match section {
                                Game9Section::Start {..} => {
                                    debug!("starting game: {}", section);
                                },
                                Game9Section::Play {play, ..} => {
                                    debug!("{}", section)
                                },
                                Game9Section::End {..} => {
                                    debug!("ending game");
                                    break
                                },
                            }
                        }
                        done = true;
                    } else {
                        error!("unable to load week 1 for year 0 ({}) in league {}", year, LEAGUE_NAME);
                    }
                } else {
                    error!("no weeks found for year 0 ({}) in league {}", year, LEAGUE_NAME);
                }
            } else {
                error!("unable to find weeks in year 0 ({}) in league {}", year, LEAGUE_NAME);
            }
        } else {
            error!("unable to find year 0 in league {}", LEAGUE_NAME);
        }
    } else {
        error!("unable to find league {}", LEAGUE_NAME);
    }

    assert!(done);
}

#[test]
fn load_all_weeks ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    const LEAGUE_NAME: &str = "Try_2";
    // const LEAGUE_NAME: &str = "New_Trial";
    let mut done = false;

    let mut league_info = fofdata::find_leagues_9();

    if let Some(league) = league_info.get_mut(LEAGUE_NAME) {
        info!("processing league: {}", LEAGUE_NAME);
        league.load_league_data();

        if let Some(year) = league.get_year(0) {
            if let Some(weeks_list) = league.get_weeks_list_for_year(year) {
                for week_num in weeks_list {
                    if let Some(week) = league.get_week(year, week_num) {
                        info!("loaded week {} for year 0 ({}) in league {}", week_num, year, LEAGUE_NAME);
                        debug!("there are {} sections in the week", week.sections.len());
                        for section in week.sections.iter() {
                            match section {
                                Game9Section::Start {..} => {
                                    debug!("starting game: {}", section);
                                },
                                Game9Section::Play {play, ..} => {
                                    debug!("{}", section)
                                    // match play {
                                    //     GamePlay9::Special {..} => {
                                    //         debug!("{}", section)
                                    //     }
                                    //     _ => {}
                                    // }
                                },
                                Game9Section::End {..} => {
                                    debug!("ending game");
                                    // break
                                },
                            }
                        }
                        done = true;
                    } else {
                        error!("unable to load week {} for year 0 ({}) in league {}", week_num, year, LEAGUE_NAME);
                    }
                }
            } else {
                error!("unable to find weeks in year 0 ({}) in league {}", year, LEAGUE_NAME);
            }
        } else {
            error!("unable to find year 0 in league {}", LEAGUE_NAME);
        }
    } else {
        error!("unable to find league {}", LEAGUE_NAME);
    }

    assert!(done);
}
