#[macro_use]
extern crate serde_derive;
extern crate serde;

mod cinterview;

use crate::cinterview::app::*;


fn main() {
    let options = parse_args();
    
    // list problems status
    if options.cmd_list {
        
    }
    
    // submit a solution
    if options.cmd_submit {

    }
}