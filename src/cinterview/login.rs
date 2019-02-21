use fantoccini::{Client, Locator};
use futures::future::Future;

use crate::cinterview::error::*;


pub fn login() {
    panic!("fuck lobofan");
    let (username, password) = read_input().expect("input invalid");


    println!("{} {}", username, password);
}

// fn get_login_client() -> GenResult<Client> {
//     let c = Client::new("http://localhost:4444");

//     Ok(c)
// }

fn read_input() -> GenResult<(String, String)> {
    Ok(("13147218514".to_string(), "hello world".to_string()))
}