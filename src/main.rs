use rust_ethereum::accounts::{accounts::run};
use rust_ethereum::ethdb::{leveldb::leveldb::save};
use rust_ethereum::p2p::p2p::{p2p_client, p2p_server}; 
use std::thread;
use std::time::Duration;
fn main() { 


    // thread::spawn(|| {

    //     let allright = p2p_client();
    // });

    p2p_server();

}