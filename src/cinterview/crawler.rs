use std;
use std::io;

use crate::cinterview::problem::{ProblemList};

pub fn get_problem_details() -> io::Result<ProblemList> {
    Ok(ProblemList::new())
}