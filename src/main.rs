#[macro_use]
extern crate serde_derive;
extern crate serde;

mod cinterview;

use crate::cinterview::app::*;


fn main() {
    let options = parse_args();
    println!("{:?}", options);
}