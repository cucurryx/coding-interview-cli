#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate serde_json;

mod cinterview;

use crate::cinterview::app::*;
use crate::cinterview::problem::*;
use crate::cinterview::submit::*;

fn main() {
    let options = parse_args();

    if options.cmd_clean {
        clean_problems();
    }

    // init cinterview, download problems
    if options.cmd_init {
        init_problems();
    }

    // list problems status
    if options.cmd_list {
        list_problems_unlogin();
    }

    // submit a solution
    if options.cmd_submit {
        submit();
    }
}
