use std::collections::HashMap;
use std::env::current_dir;

use std::io::Read;
use std::path::PathBuf;

use crate::cinterview::config::*;
use crate::cinterview::error::*;
use crate::cinterview::problem::*;


pub fn submit(_test: bool, exam: bool, nums: Vec<u32>) {
    let debug_info = !exam;
    read_local_code(nums, "cc".to_string()).expect("read local data fail");

}

fn read_local_code(nums: Vec<u32>, suffix: String) -> GenResult<HashMap<u32, String>> {
    let code_root = code_root_dir()?;
    let to_submit = read_local_problems(&PROBLEM_PATH)?
        .into_iter()
        .filter(|x| nums.clone().into_iter().find(|n| n == &x.num).is_some())
        .collect::<ProblemList>();

    let mut result = HashMap::new();
    for x in to_submit {
        let filename = format!("{}_{}", x.num, x.name);
        let path = code_root
            .join(&filename)
            .join(filename)
            .with_extension(&suffix);
        let mut code = String::new();
        ensure_open(&path)?.read_to_string(&mut code)?;
        result.insert(x.num, code);
    }

    Ok(result)
}

fn code_root_dir() -> GenResult<PathBuf> {
    current_dir()?
        .ancestors()
        .skip_while(|x| x.ends_with("coding-interview"))
        .next()
        .and_then(|x| Some(x.to_path_buf()))
        .ok_or(Box::new(CodeRootError {}))
}

fn send_code() {
    // current_dir()?.to_path_buf();
}
