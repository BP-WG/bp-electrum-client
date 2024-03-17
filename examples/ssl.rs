extern crate electrum;

use electrum::{Client, ElectrumApi};

fn main() {
    let client = Client::new("ssl://electrum.blockstream.info:50002").unwrap();
    let res = client.server_features();
    println!("{:#?}", res);
}
