extern crate electrum_client_doge;

use electrum_client_doge::{Client, ElectrumApi};

fn main() {
    let client = Client::new("ssl://electrum.blockstream.info:50002").unwrap();
    let res = client.server_features();
    println!("{:#?}", res);
}
