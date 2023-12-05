use log::{info, error, debug};

mod common;

#[test]
fn load_players ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    // const LEAGUE_NAME: &str = "New_Trial";
    const LEAGUE_NAME: &str = "Try_2";

    let mut done = true;

    let mut league_info = fofdata::find_leagues_9();

    if let Some(league) = league_info.get_mut(LEAGUE_NAME) {
        info!("processing league: {}", LEAGUE_NAME);
        if let Some(players) = league.get_players() {
            for player in players.players() {
                debug!("{}", player);
            }
        } else {
            error!("unable to read players for league {}", LEAGUE_NAME);
            done = false;
        }
    } else {
        error!("unable to find league {}", LEAGUE_NAME);
        done = false;
    }

    assert!(done);
}

#[test]
fn load_all_players ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    // const LEAGUE_NAME: &str = "New_Trial";

    let mut done = true;

    let mut league_info = fofdata::find_leagues_9();

    for (league_name, league) in league_info.iter_mut() {
        info!("processing league: {}", league_name);
        if let Some(players) = league.get_players() {
            for player in players.players() {
                debug!("{}", player);
            }
        } else {
            error!("unable to read players for league {}", league_name);
            done = false;
        }
    }

    assert!(done);
}

