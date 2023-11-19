use std::{path::{PathBuf, Path}, collections::BTreeMap, fs};

use directories::BaseDirs;
use log::{info, warn, debug};
use walkdir::WalkDir;

mod fof9_leaguedata;

pub use fof9_leaguedata::LeagueData;

pub const LEAGUES_PATH: &str = "Solecismic Software\\Front Office Football Nine\\saved_games";
pub const LEAGUEINFO_FILENAME: &str = "league.dat";

#[allow(dead_code)]
#[derive(Debug)]
pub struct LeagueFileInfo {
    pub datapath: PathBuf,
    pub gamepath: PathBuf,
}

pub fn find_leagues () -> BTreeMap<String, LeagueFileInfo> {
    info!("finding fof9 leagues");
    let base_dirs = BaseDirs::new().unwrap();
    let localdata_path = base_dirs.data_local_dir();
    let leagues_pathbuf = localdata_path.join(LEAGUES_PATH);

    let leagues_path = leagues_pathbuf.as_path();
    let league_hash:BTreeMap<String, LeagueFileInfo> = WalkDir::new(leagues_path).min_depth(1).max_depth(1).into_iter()
        .filter_map(|entry| {
        let e = entry.unwrap();
        let p = e.path();
        if let Ok(m) = fs::metadata(p.join(LEAGUEINFO_FILENAME).as_path()) {
            if m.is_file() {
                // we know this is an ok directory
                let l = e.path().file_stem().unwrap().to_string_lossy().to_string().to_owned();
                let l_p = Path::new(&l);
                let lg_pathbuf = leagues_pathbuf.join(l_p);
                Some((l, LeagueFileInfo{datapath: p.to_path_buf().to_owned(), gamepath: lg_pathbuf.to_owned()}))
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

#[cfg(test)]
mod tests {
}
