#![feature(rustc_private)]

extern crate std;
#[macro_use]
extern crate bitflags;

pub mod native;
pub mod renet;


use self::renet::*;

fn main() {
    renet_initialize().unwrap();
    println!("Initialized enet!");
    let mut server = ReNetServerHost::new(
        ReNetAddress {
            host: ENET_HOST_ANY,
            port: 1234,
        },
        32,
        0,
        0,
    ).unwrap();
    println!("Created server!");

    loop {
        match server.service(Some(1000)).unwrap() {
            ReNetEvent::None => {
                server.broadcast(1, &[b'p', b'i', b'n', b'g', b'!', 0], Default::default());
                continue
            },
            ReNetEvent::Connect {
                peer
            } => {
                println!("Peer connected!");
            },
            ReNetEvent::Disconnect {
                peer,
                data,
            } => {
                println!("Peer disconnected!");
            },
            ReNetEvent::Receive {
                peer,
                channelID,
                data,
                flags
            } => {
                println!("Received data on {} with flags {:x?} : {:x?}", channelID, flags, data);
            }
        }
    }
    renet_deinitialize();
}
