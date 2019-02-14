use std;
use std::io;

pub type ProblemList = Vec<Problem>;

#[derive(Debug)]
pub struct Problem {

    /// The number of problem. Start from 0 to 65 for now. 
    num: u32,

    /// The name of problem
    name: String,

    /// The content of problem
    content: String,

    /// Whether have pass this problem. If use choose login mode, it's based on the remote status.
    /// Otherwise, it's loaded from local log.
    passed: bool
}


pub fn list_problems_login() -> io::Result<Vec<Problem>> {
    return Ok(Vec::new());
}

pub fn list_problems_unlogin() -> io::Result<Vec<Problem>> {
    // TODO, read data from LOCAL_PROBLEM_PATH
    return Ok(Vec::new());
}