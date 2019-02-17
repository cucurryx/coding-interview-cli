extern crate dirs;
use dirs::home_dir;

use std::path::PathBuf;

lazy_static! {
    /// Static local data root pathbuf, equals to "$HOME/.coding-interview/"
    pub static ref LOCAL_ROOT: PathBuf = home_dir().unwrap().join(".coding-interview");

    /// Static local problem data path, equals to "$HOME/.coding-interview/problem.json"
    pub static ref PROBLEM_PATH: PathBuf = LOCAL_ROOT.join("problem.json");
}
