#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate serde_json;

extern crate rayon;

mod cinterview;

use crate::cinterview::app::*;
use crate::cinterview::problem::*;
use crate::cinterview::submit::*;

fn main() {
    let options = parse_args();

    // clean local data of problems
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
        submit(
            options.flag_test,
            options.flag_exam,
            options.arg_lang,
            options.arg_num,
        );
    }
}
