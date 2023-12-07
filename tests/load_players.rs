use fofdata::{LeagueInfo, AltPlayers9Header, AltPlayer9Data, AltPlayer9Id};
use log::{info, error, debug};
use binrw::BinReaderExt;

mod common;


#[test]
fn load_alt_players ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    // const LEAGUE_NAME: &str = "New_Trial";
    const LEAGUE_NAME: &str = "Try_2";

    let mut done = true;

    let mut league_info = fofdata::find_leagues_9();

    if let Some(league) = league_info.get_mut(LEAGUE_NAME) {
        info!("processing league: {}", LEAGUE_NAME);
        let mut file = league.get_players_file();

        match file.read_ne::<AltPlayers9Header>() {
            Ok(alt_header) => {
                const BASE_ID: u32 = 1000;
                let max_id = alt_header.max_player_id() + BASE_ID;
                let mut previous_id = 0;
                while done {
                    match file.read_ne::<AltPlayer9Id>() {
                        Ok(alt_player_id) => {
                            let player_id = alt_player_id.player_id();
                            if (player_id >= BASE_ID) && (player_id < max_id) && (player_id > previous_id)
                            {
                                previous_id = player_id;
                                match file.read_ne::<AltPlayer9Data>() {
                                    Ok(alt_player_data) => {
                                        debug!("{}, {}", alt_player_id.player_id(), alt_player_data.position_group());
                                    },

                                    Err(err) => {
                                        error!("unable to read alt player data from file: {}", err);
                                        done = false;
                                    }
                                }

                                if player_id == max_id - 1 {
                                    break
                                }
                            } else { break }
                        },

                        Err(err) => {
                            error!("unable to read alt player id from file: {}", err);
                            done = false;
                        }
                     }
                }
            },

            Err(err) => {
                error!("unable to read alt player header from file: {}", err);
                done = false;
            }
        }
    } else {
        error!("unable to find league {}", LEAGUE_NAME);
        done = false;
    }

    assert!(done);
}

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

