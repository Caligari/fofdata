use fofdata::{AltPlayer9Data, AltPlayer9Id, AltPlayers9Header, LeagueInfo};
use log::{debug, error, info};
use binrw::BinReaderExt;

mod common;


#[test]
fn load_alt_players ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    // const LEAGUE_NAME: &str = "New_Trial";
    // const LEAGUE_NAME: &str = "Try_2";
    const LEAGUE_NAME: &str = "Try_3";
    // const LEAGUE_NAME: &str = "Nawlins";

    let mut done = true;

    let league_info = fofdata::find_leagues_9();

    if let Some(league) = league_info.get_league_info(LEAGUE_NAME) {
        info!("processing league: {}", LEAGUE_NAME);
        let mut file = league.get_players_file();

        match file.read_ne::<AltPlayers9Header>() {
            Ok(alt_header) => {
                const BASE_ID: u32 = 1000;
                let max_id = alt_header.max_player_id() + BASE_ID;
                let mut previous_id = 0;
                let mut count = 0;
                while done {
                    match file.read_ne::<AltPlayer9Id>() {
                        Ok(alt_player_id) => {
                            let player_id = alt_player_id.player_id();
                            if (player_id >= BASE_ID) && (player_id < max_id) && (player_id > previous_id)
                            {
                                previous_id = player_id;
                                match file.read_ne::<AltPlayer9Data>() {
                                    Ok(alt_player_data) => {
                                        debug!("{}, {} {}", alt_player_id.player_id(), alt_player_data.position_group(), alt_player_data.name());
                                        if player_id == 1000 {
                                            debug!("{:?}", alt_player_data);
                                        }
                                        count += 1;
                                    },

                                    Err(err) => {
                                        error!("unable to read alt player data from file: {}", err);
                                        done = false;
                                    }
                                }

                                if player_id == max_id - 1 {
                                    debug!("found last id ");
                                    debug!("found {} players (predicted: {})", count, alt_header.max_player_id());

                                    match file.read_ne::<AltPlayer9Id>() {
                                        Ok(alt_player_id) => {
                                            let player_id = alt_player_id.player_id();
                                            debug!("found post player id: {}", player_id);
                                        },

                                        Err(err) => {
                                            error!("unable to read alt post-player id from file: {}", err);
                                            done = false;
                                        }
                                     }

                                    break
                                }
                            } else { debug!("player id out of sequence"); break }
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

    const LEAGUE_NAME: &str = "New_Trial";
    // const LEAGUE_NAME: &str = "Try_2";
    // const LEAGUE_NAME: &str = "Try_3";
    // const LEAGUE_NAME: &str = "Nawlins";

    let mut done = true;

    let league_info = fofdata::find_leagues_9();

    if let Some(league) = league_info.get_league_info(LEAGUE_NAME) {
        info!("processing league: {}", LEAGUE_NAME);
        if let Some(players) = league.get_players() {
            debug!("number players: {}", players.players().len());
            for player in players.players() {
                debug!("{}", player);
            }
            debug!("number staff: {}", players.staff().len());
            for _staff in players.staff() {
                // debug!("{}", staff);  // TODO
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
fn load_team_players ( ) {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    // const LEAGUE_NAME: &str = "New_Trial";
    // const LEAGUE_NAME: &str = "Try_2";
    // const LEAGUE_NAME: &str = "Try_3";
    const LEAGUE_NAME: &str = "Nawlins";

    let mut done = true;

    let league_info = fofdata::find_leagues_9();

    if let Some(league) = league_info.get_league_info(LEAGUE_NAME) {
        info!("processing league: {}", LEAGUE_NAME);
        if let Some(players) = league.get_players() {
            debug!("number players: {}", players.players().len());

            // league.load_data();  // TODO: as we can't do this, we do not know how many teams there are, or what they are called
            let team = 0;

            for player in players.players().iter().filter(|p| { if let Some(t) = p.team_id() { t == team } else { false } }) {
                debug!("{}", player);
            }

            debug!("number staff: {}", players.staff().len());
            // for _staff in players.staff() {
            //     // debug!("{}", staff);  // TODO
            // }
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

    let league_info = fofdata::find_leagues_9();

    for league_name in league_info.league_name_list() {
        info!("processing league: {}", league_name);
        if let Some(league) = league_info.get_league_info(&league_name){
            if let Some(players) = league.get_players() {
                debug!("number players: {}", players.players().len());
                for player in players.players() {
                    debug!("{}", player);
                }
                debug!("number staff: {}", players.staff().len());
                for staff in players.staff() {
                    debug!("{}", staff);
                }
            } else {
                error!("unable to read players for league {}", league_name);
                done = false;
            }
        } else { error!("unable to find league file info for {}", league_name); }
    }

    assert!(done);
}

