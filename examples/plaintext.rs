extern crate electrum;

use electrum::{Client, ElectrumApi};

fn main() {
    let client = Client::new("tcp://electrum.blockstream.info:50001").unwrap();
    let res = client.server_features();
    println!("{:#?}", res);
}
