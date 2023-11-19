// use fofdata;

use std::{fs::File, io::BufReader, collections::BTreeMap};
use binrw::{BinReaderExt, Error};
use log::{info, debug, warn, error};
use num_traits::FromPrimitive;

mod common;

#[test]
fn league_load() {
    common::setup_logger(module_path!()).expect("log did not start");
    info!("Starting");

    let league_info = fofdata::find_leagues();

    for (league_name, league_file_info) in league_info {
        info!("processing league: {}", league_name);
        let league_info_path = league_file_info.datapath.join(fofdata::LEAGUEINFO_FILENAME);

        info!("loading league info: {}", league_info_path.to_string_lossy());
        let lf = File::open(league_info_path).unwrap_or_else(|e| { panic!("Unable to open league file: {}", e) });
        let mut leaguefile = BufReader::new(lf);
        debug!("opened league file");
        let league_info_result: Result<fofdata::LeagueData, Error> = leaguefile.read_ne();
        match league_info_result {
            Ok(league_info) => {
                debug!("league: {}", league_info.league_name.string);
                debug!("league championship: {}", league_info.championship_name.string);
                debug!("league number of teams: {}", league_info.number_teams);
                debug!("league conference: {} ({})", league_info.conference1_name.string, league_info.conference1_short.string);
                debug!("league conference: {} ({})", league_info.conference2_name.string, league_info.conference2_short.string);
                debug!("league number of divisions: {}", league_info.number_divisions);
                debug!("league divisions: {:?}", league_info.divisions);
                debug!("league structure name: {}", league_info.structure_name.string);
                if league_info.number_teams != league_info.teams_len {
                    error!("league number of teams does not equal length of teams list ({})", league_info.teams_len);
                } else {
                    let mut team_info = BTreeMap::<String, usize>::new();
                    for team in &league_info.teams {
                        debug!("team {}: {} {} ({})", team.team_number, team.team_city, team.team_name, team.team_short);
                        if team.team_city.ne(&team.team_city2) { warn!("city name 2 does not agree ({})", team.team_city2)}
                        if team.team_name.ne(&team.team_name2) { warn!("team name 2 does not agree ({})", team.team_name2)}
                        if team.team_short.ne(&team.team_short2) { warn!("short name 2 does not agree ({})", team.team_short2)}
                        team_info.insert(team.team_city.to_string(), usize::from_u32(team.team_number).unwrap());  // TODO: better checking
                    }
                    let teams: Vec<String> = team_info.keys().cloned().collect();
                    debug!("all teams: {:?}", teams);
                }
            },

            Err(err) => {
                error!("unable to parse league: {:?}", err);
            }
        }
    }

    info!("Done");
}
