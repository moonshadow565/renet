extern crate std;

mod defs;
use self::defs::*;
use std::mem::size_of;
use std::os::raw::*;

fn main() {
    if !renet_initialize() {
        panic!("Failed to initialize enet!");
    } else {
        println!("Initialized enet!");
    }
    let mut server = ReNetServerHost::new(ReNetAddress{
        host: HOST_ANY,
        port: 1234,
    }, 32,0, 0).unwrap();

    while true  {
        let myb = server.service(100).unwrap();
        match myb {
            None => continue,
            Some(event) => match event {
                ReNetEvent::Connect {
                    peer
                } => {
                    println!("Peer conntected!");
                },
                ReNetEvent::Disconnect {
                    peer,
                    data
                } =>  {
                    println!("Peer disconnected!");
                },
                ReNetEvent::Receive {
                    peer,
                    packet,
                    channelID,
                } => {
                    println!("Got data on {} with message: {:x?}", channelID, packet.data);
                }
            }
        }
    }
}
