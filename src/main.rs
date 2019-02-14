#[macro_use]
extern crate serde_derive;
extern crate serde;

mod cinterview;

use crate::cinterview::app::*;
use crate::cinterview::crawler::*;


fn main() {
    let options = parse_args();
    let problems = get_problem_details();
    println!("{:?}", problems);

    // list problems status
    if options.cmd_list {

    }
    
    // submit a solution
    if options.cmd_submit {

    }
}

