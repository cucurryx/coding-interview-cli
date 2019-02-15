use std;
use std::io;
use std::io::Write;

use std::fs;
use std::path::PathBuf;

extern crate dirs;
use dirs::home_dir;

extern crate termion;
use termion::{color};

use crate::cinterview::crawler::*;
use crate::cinterview::error::*;

pub type ProblemList = Vec<Problem>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    /// The number of problem. Start from 0 to 65 for now.
    pub num: u32,

    /// The name of problem
    pub name: String,

    /// The content of problem
    pub content: String,

    /// Whether have pass this problem. If use choose login mode, it's based on the remote status.
    /// Otherwise, it's loaded from local log.
    pub passed: bool,
}

/// TODO, support login
pub fn list_problems_login() {}

pub fn list_problems_unlogin() {
    let local_root = home_dir().unwrap().join(".coding-interview");
    let problem_path = local_root.join("problem.json");
    ensure_local_data(&local_root, &problem_path);
    match fs::read_to_string(problem_path) {
        Ok(data) => {
            let problems: ProblemList =
                serde_json::from_str(data.as_str()).expect("parse problem list fail! ❌");
            print_problem_infos(problems);
        }
        Err(_e) => {}
    }
}

pub fn init_problems() {
    let local_root = home_dir().unwrap().join(".coding-interview");
    ensure_local_data(&local_root, &local_root.join("problem.json"));
    println!("\n 😘😘😘\tinit ok...");
}

fn ensure_local_data(root: &PathBuf, problem_path: &PathBuf) {
    ensure_dir(root).expect("ensure dir fail!");
    if !problem_path.exists() {
        update_problems(get_problems()).expect("update problems ! ❌");
    }
}

fn print_problem_infos(problems: ProblemList) {
    for x in problems {
        let emoji = if x.passed {
            print!("{}", color::Fg(color::Green));
            "👍🏻"
        } else {
            print!("{}", color::Fg(color::Red));
            "😡"
        };
        println!("{}\t {}[{}] \t{}", emoji, color::Fg(color::Red), x.num, x.name);
    }
    println!("{}[passed] [num] [problem-title]", color::Fg(color::Reset));
}

fn ensure_dir(path: &PathBuf) -> GenResult<()> {
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

fn update_problems(problems: ProblemList) -> io::Result<()> {
    let json_str = serde_json::to_string(&problems)?;
    let problem_path = home_dir().unwrap().join(".coding-interview/problem.json");
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(problem_path)?
        .write_all(json_str.as_bytes())
}
