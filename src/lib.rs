use std::{path::{PathBuf, Path}, collections::BTreeMap, fs::{self, File}, io::BufReader};
use directories::BaseDirs;
use log::{info, warn, debug, error};
use multimap::MultiMap;
use walkdir::WalkDir;
use lazy_static::lazy_static;
use regex::Regex;
use binrw::BinReaderExt;
use num_traits::FromPrimitive;

mod fof9_utility;
mod fof9_leaguedata;
mod fof9_weekdata;
mod fof9_playerdata;
pub use fof9_leaguedata::League9Data;
pub use fof9_weekdata::{Week9Data, Game9Section, GamePlay9, Game9Data};
pub use fof9_playerdata::{AltPlayers9Header, AltPlayer9Data, AltPlayer9Id, Players9Data};

pub const LEAGUES_9_PATH: &str = "Solecismic Software\\Front Office Football Nine\\saved_games";
pub const LEAGUEINFO_9_FILENAME: &str = "league.dat";


pub fn find_leagues_9 () -> BTreeMap<String, League9FileInfo> {
    info!("finding fof9 leagues");
    let base_dirs = BaseDirs::new().unwrap();
    let localdata_path = base_dirs.data_local_dir();
    let leagues_pathbuf = localdata_path.join(LEAGUES_9_PATH);

    let leagues_path = leagues_pathbuf.as_path();
    let league_hash:BTreeMap<String, League9FileInfo> = WalkDir::new(leagues_path).min_depth(1).max_depth(1).into_iter()
        .filter_map(|entry| {
        let e = entry.unwrap();
        let p = e.path();
        if let Ok(m) = fs::metadata(p.join(LEAGUEINFO_9_FILENAME).as_path()) {
            if m.is_file() {
                // we know this is an ok directory
                let l = e.path().file_stem().unwrap().to_string_lossy().to_string().to_owned();
                let l_p = Path::new(&l);
                let lg_pathbuf = leagues_pathbuf.join(l_p);
                Some((l.clone(), League9FileInfo{name: l, datapath: p.to_path_buf().to_owned(), gamepath: lg_pathbuf.to_owned(), week_index: None}))
            }
            else {
                warn!("league not a file: {}", p.to_string_lossy());
                None
            }
        }
        else {
            warn!("league has no meta data: {}", p.to_string_lossy());
            None
        }
    }).collect();
    let league_list: Vec<String> = league_hash.keys().cloned().collect();
    assert!(!league_list.is_empty());
    debug!("found leagues: {:?}", league_list);

    league_hash
}

pub trait LeagueInfo {
    fn load_league_data ( & mut self );

    fn get_week_index ( &self ) -> &Option<MultiMap<u16, u8>>;

    fn get_teams_list ( &self ) -> Vec<String>;

    // may not need path
    fn get_week_path ( &self, year: u16, week: u8 ) -> PathBuf;
    fn get_week_file ( &self, year: u16, week: u8 ) -> BufReader<File>;  // TODO: could we digest it and pass that?
    fn get_players_file ( &self ) -> BufReader<File>;  // TODO: could we digest it and pass that?

    fn get_weeks_list_for_year ( &self, year: u16 ) -> Option<Vec<u8>> {
        if let Some(week_index) = &self.get_week_index() {
            if let Some(week_list) = week_index.get_vec(&year) {
                let mut week_list = week_list.clone();
                week_list.sort();
                Some(week_list)
            } else { None }
        } else { None }
    }

    fn get_years_list_reversed ( &self ) -> Vec<u16> {
        let mut years: Vec<u16> = {
            if let Some(week_index) = &self.get_week_index() {
                week_index.keys().copied().collect()
            } else { Vec::new() }
        };
        years.sort();
        years.reverse();
        years
    }

    fn get_number_years ( &self ) -> usize {
        if let Some(week_index) = &self.get_week_index() {
            week_index.keys().len()
        } else { 0 }
    }

    fn get_year ( &self, selection_index: usize ) -> Option<u16> {
        if let Some(week_index) = &self.get_week_index() {
            if selection_index < week_index.keys().len() {
                let mut years: Vec<u16> = week_index.keys().copied().collect();
                years.sort();
                years.reverse();
                Some(years[selection_index])
            } else { None }
        } else { None }
    }
}

#[derive(Debug, Clone)]
pub struct League9FileInfo {
    pub name: String,
    pub datapath: PathBuf,
    pub gamepath: PathBuf,
    pub week_index: Option<MultiMap<u16, u8>>,  // year, week
}

impl League9FileInfo {
    pub fn get_week ( &self, year: u16, week: u8 ) -> Option<Week9Data> {
        let mut file = self.get_week_file(year, week);

        match file.read_ne() {
            Ok(week_data) => {
                Some(week_data)
            },

            Err(err) => {
                error!("unable to read week file: {}", err);
                None
            }
        }
    }

    pub fn get_players ( &self ) -> Option<Players9Data> {
        let mut file = self.get_players_file();

        match file.read_ne() {
            Ok(players) => {
                Some(players)
            },

            Err(err) => {
                error!("unable to read players file: {}", err);
                None
            }
        }
    }
}

impl LeagueInfo for League9FileInfo {
    fn load_league_data ( &mut self ) {
        if self.week_index.is_none() {
            lazy_static!{
                // year is 4 digits
                // wk is 1 or 2 digits
                // filename is year_<year>_week_<wk>.dat
                static ref FILENAME_MATCH: Regex = Regex::new(r"^year_(?P<year>\d{4})_week_(?P<week>\d{1,2})\.dat$").unwrap();
            }

            self.week_index = Some(
                WalkDir::new(&self.gamepath).min_depth(1).max_depth(1).into_iter()
                .filter_map(|e| {
                    let entry = e.unwrap();
                    let filename = entry.path().file_name().unwrap().to_string_lossy();

                    if let Some(cap) = FILENAME_MATCH.captures(&filename) {
                        if let Some(week) = cap.name("week") {
                            if let Some(year) = cap.name("year") {
                                let wk:u8 = week.as_str().parse().unwrap();
                                let yr:u16 = year.as_str().parse().unwrap();
                                Some((yr, wk))
                            } else { None }
                        } else { None }
                    } else { None }
                })
                .collect()
            );
        }  // else, week data already loaded
    }

    fn get_week_path ( &self, year: u16, week: u8 ) -> PathBuf {
        let filename = format!("year_{}_week_{}.dat", &year, &week);
        self.gamepath.join(filename)
    }

    fn get_week_file ( &self, year: u16, week: u8 ) -> BufReader<File> {
        let filename = format!("year_{}_week_{}.dat", &year, &week);
        let filepath = self.gamepath.join(&filename);
        debug!("opening game week: {}", filename);
        let file = File::open(filepath).unwrap_or_else(|_| panic!("Unable to open week file: {}", filename));
        BufReader::new(file)
    }

    fn get_players_file ( &self ) -> BufReader<File> {
        const PLAYERS_FILENAME: &str = "players.dat";
        let filepath = self.gamepath.join(PLAYERS_FILENAME);
        debug!("opening players file: {}", PLAYERS_FILENAME);
        let file = File::open(filepath).unwrap_or_else(|_| panic!("Unable to open players file: {}", PLAYERS_FILENAME));
        BufReader::new(file)
    }

    fn get_week_index ( &self ) -> &Option<MultiMap<u16, u8>> {
        &self.week_index
    }

    fn get_teams_list ( &self ) -> Vec<String> {
        let league_info_path = self.datapath.join(LEAGUEINFO_9_FILENAME);

        info!("loading league info from: {}", league_info_path.to_string_lossy());
        let lf = File::open(league_info_path).unwrap_or_else(|e| { panic!("Unable to open league file: {}", e) });  // TODO: no panic
        let mut leaguefile = BufReader::new(lf);
        debug!("opened league file");
        let league_info_result: Result<League9Data, binrw::Error> = leaguefile.read_ne();
        match league_info_result {
            Ok(league_info) => {
                debug!("league: {}", league_info.league_name.string);
                debug!("league number of teams: {}", league_info.number_teams);
                if league_info.number_teams != league_info.teams_len {
                    error!("league number of teams does not equal length of teams list ({})", league_info.teams_len);
                    Vec::new()
                } else {
                    let mut team_info = BTreeMap::<String, usize>::new();
                    for team in &league_info.teams {
                        team_info.insert(team.team_city.to_string(), usize::from_u32(team.team_number).unwrap());  // TODO: better checking
                    }
                    let teams: Vec<String> = team_info.keys().cloned().collect();
                    debug!("all teams: {:?}", teams);
                    teams
                }
            },

            Err(err) => {
                error!("unable to parse league: {:?}", err);
                Vec::new()
            }
        }
    }
}


#[derive(Debug)]
pub enum Position {
    QB,
    RB,
    FB,
    TE,
    FL,
    SE,
    WR,
    C,
    RG,
    LG,
    RT,
    LT,
    P,
    K,
    DLE,
    DRE,
    DLT,
    DRT,
    DT,
    UT,
    NT,
    SILB,
    WILB,
    MLB,
    SLB,
    WLB,
    RCB,
    LCB,
    NB,
    DB,
    SS,
    FS,
    LS,
    PR,
    KR,
}

#[cfg(test)]
mod tests {
    use crate::LeagueInfo;

    #[test]
    fn league_9_info () {
        let league_info = crate::find_leagues_9();

        for (_league_name, mut league_file_info) in league_info {
            league_file_info.load_league_data();
            league_file_info.get_number_years();
            league_file_info.get_years_list_reversed();
        }
    }
}
