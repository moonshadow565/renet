use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::*;
use std::slice::from_raw_parts;
use std::ptr;

pub type size_t = *const c_void;

pub const HOST_ANY: u32 = 0;
pub const HOST_BROADCAST: u32 = 0xFFFFFFFF;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ENetAddress {
    pub host: u32,
    pub port: u16,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ENetPacketFlag {
    RELIABLE = (1 << 0),
    UNSEQUENCED = (1 << 1),
    NO_ALLOCATE = (1 << 2),
}

#[repr(C)]
pub struct ENetPacket {
    refrenceCount: size_t,
    flags: ENetPacketFlag,
    data: *mut u8,
    dataLength: size_t,
    freeCallback: extern "C" fn(packet: *mut ENetPacket),
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum ENetPeerState {
    DISCONNECTED = 0,
    CONNECTING = 1,
    ACKNOWLEDGING_CONNECT = 2,
    CONNECTION_PENDING = 3,
    CONNECTION_SUCCEEDED = 4,
    CONNECTED = 5,
    DISCONNECT_LATER = 6,
    DISCONNECTING = 7,
    ACKNOWLEDGING_DISCONNECT = 8,
    ZOMBIE = 9,
}

#[repr(C)]
pub struct ENetPeer {
    pub host: *mut ENetHost,
    outgoingPeerID: u16,
    incomingPeerID: u16,
    pub sessionID: u32,
    pub address: ENetAddress,
    pub data: *mut c_void,
    pub state: ENetPeerState,
    // incomplete type
}

#[repr(C)]
pub struct ENetHost {
    // incomplete type
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum ENetEventType {
    NONE = 0,
    CONNECT = 1,
    DISCONNECT = 2,
    RECEIVE = 3,
}

#[repr(C)]
pub struct ENetEvent {
    etype: ENetEventType,
    peer: *mut ENetPeer,
    channelID: u8,
    data: u32,
    packet: *mut ENetPacket,
}

#[link(name = "enet")]
extern "C" {
    fn enet_initialize() -> c_int;
    fn enet_deinitialize();

    fn enet_address_set_host(address: *mut ENetAddress, hostName: *const c_char) -> c_int;
    fn enet_address_get_host_ip(
        address: *const ENetAddress,
        hostName: *mut c_char,
        nameLength: size_t,
    ) -> c_int;
    fn enet_address_get_host(
        address: *const ENetAddress,
        hostName: *mut c_char,
        nameLength: size_t,
    ) -> c_int;

    fn enet_packet_create(data: *const u8, size: size_t, flags: ENetPacketFlag) -> *mut ENetPacket;
    fn enet_packet_destroy(packet: *mut ENetPacket);
    // fn enet_packet_resize(packet: *mut ENetPacket, size: size_t) -> c_int;

    fn enet_host_create(
        address: *const ENetAddress,
        peerCount: size_t,
        incomingBandwith: u32,
        outgoingBandwith: u32,
    ) -> *mut ENetHost;
    fn enet_host_destroy(host: *mut ENetHost);
    fn enet_host_connect(
        host: *mut ENetHost,
        address: *const ENetAddress,
        channelCount: size_t,
    ) -> *mut ENetPeer;
    fn enet_host_service(host: *mut ENetHost, event: *mut ENetEvent, timeout: u32) -> c_int;
    fn enet_host_flush(host: *mut ENetHost);
    fn enet_host_broadcast(host: *mut ENetHost, channelID: u8, packet: *mut ENetPacket);
    fn enet_host_bandwidth_limit(
        host: *mut ENetHost,
        incomingBandwidth: u32,
        outgoingBandwith: u32,
    );

    fn enet_peer_send(peer: *mut ENetPeer, channelID: u8, packet: *mut ENetPacket) -> c_int;
    fn enet_peer_receive(peer: *mut ENetPeer, channelID: u8) -> *mut ENetPacket;
    fn enet_peer_ping(peer: *mut ENetPeer);
    fn enet_peer_reset(peer: *mut ENetPeer);
    fn enet_peer_disconnect(peer: *mut ENetPeer, data: u32);
    fn enet_peer_disconnect_now(peer: *mut ENetPeer, data: u32);
    fn enet_peer_disconnect_later(peer: *mut ENetPeer, data: u32);
    fn enet_peer_throttle_configure(
        peer: *mut ENetPeer,
        interval: u32,
        acceleration: u32,
        deceleration: u32,
    );
}

pub fn renet_initialize() -> bool {
    unsafe { enet_initialize() == 0 }
}

pub fn renet_deinitialize() {
    unsafe {
        enet_deinitialize();
    }
}



/**
* ADDRESS
**/
pub type ReNetAddress = ENetAddress;
impl ReNetAddress {
    pub fn new(host: u32, port: u16) -> ENetAddress {
        ENetAddress { host, port }
    }

    pub fn set_host(&mut self, host_name: &CStr) -> Result<(), &'static str> {
        unsafe {
            if enet_address_set_host(self, host_name.as_ptr()) == 0 {
                Ok(())
            } else {
                Err("Bad address!")
            }
        }
    }

    pub fn get_host_ip(&self) -> Option<CString> {
        let mut buffer: [c_char; 256] = [0; 256];
        unsafe {
            if enet_address_get_host_ip(self, &mut buffer[0], 255 as size_t) == 0 {
                Some(CString::from_raw(&mut buffer[0]))
            } else {
                None
            }
        }
    }

    pub fn get_host(&self) -> Option<CString> {
        let mut buffer: [c_char; 256] = [0; 256];
        unsafe {
            if enet_address_get_host(self, &mut buffer[0], 255 as size_t) == 0 {
                Some(CString::from_raw(&mut buffer[0]))
            } else {
                None
            }
        }
    }
}

/**
* PACKET
**/
#[derive(Clone)]
pub struct ReNetPacket<'a> {
    pub data: &'a [u8],
    pub flags: ENetPacketFlag,
}

/**
* PEER
**/
type ReNetPeer = ENetPeer;
impl ReNetPeer {
    fn send(&mut self, channelID: u8, packet: &ReNetPacket) -> Result<(), &'static str> {
        unsafe {
            let rawPacket = enet_packet_create(
                &packet.data[0],
                packet.data.len() as size_t,
                packet.flags,
            );
            if enet_peer_send(self,channelID, rawPacket) == 0 {
                Ok(())
            } else {
                Err("Failed to send to peer")
            }
        }
    }

    fn receive(&mut self, channelID: u8) -> Option<ReNetPacket> {
        unsafe {
            let rawPacket = enet_peer_receive(self, channelID);
            if rawPacket == 0 as *mut ENetPacket {
                None
            } else {
                Some(ReNetPacket{
                    data: from_raw_parts((*rawPacket).data, (*rawPacket).dataLength as usize),
                    flags: (*rawPacket).flags
                })
            }
        }
    }

    fn ping(&mut self) {
        unsafe {
            enet_peer_ping(self);
        }
    }

    fn reset(&mut self) {
        unsafe {
            enet_peer_reset(self);
        }
    }

    fn disconnect(&mut self, data: u32) {
        unsafe {
            enet_peer_disconnect(self, data);
        }
    }

    fn disconnect_now(&mut self, data: u32) {
        unsafe {
            enet_peer_disconnect_now(self, data);
        }
    }

    fn disconnect_later(&mut self, data: u32) {
        unsafe {
            enet_peer_disconnect_later(self, data);
        }
    }
    fn throttle_configure(&mut self, interval: u32, acceleration: u32, deceleration: u32) {
        unsafe {
            enet_peer_throttle_configure(self, interval, acceleration, deceleration);
        }
    }
}

/**
* EVENT
**/
pub enum ReNetEvent<'a, 'b> {
    Connect {
        peer: &'a mut ReNetPeer
    },
    Disconnect {
        peer: &'a mut ReNetPeer,
        data: u32,
    },
    Receive {
        peer: &'a mut ReNetPeer,
        packet: ReNetPacket<'b>,
        channelID: u8
    }
}

/**
* HOST
**/
pub trait ReNetHost: Drop {
    fn host_ptr(&mut self) -> *mut ENetHost;
    fn flush(&mut self) {
        unsafe {
            enet_host_flush(self.host_ptr());
        }
    }
    fn bandwidth_limit(&mut self, incomingBandwidth: u32, outgoingBandwith: u32) {
        unsafe {
            enet_host_bandwidth_limit(self.host_ptr(), incomingBandwidth, outgoingBandwith);
        }
    }
    fn broadcast(&mut self, channelID: u8, packet: &ReNetPacket) {
        unsafe {
            let rawPacket = enet_packet_create(
                &packet.data[0],
                packet.data.len() as size_t,
                packet.flags,
            );
            enet_host_broadcast(self.host_ptr(), channelID, rawPacket);
        }
    }

    fn service<'a, 'b>(&'a mut self, timeout: u32) -> Result<Option<ReNetEvent<'a, 'b>>, &'static str> {
        unsafe {
            let mut event = ENetEvent {
                etype: ENetEventType::NONE,
                peer: 0 as *mut ENetPeer,
                channelID: 0u8,
                data: 0u32,
                packet: 0 as *mut ENetPacket,
            };
            let result = enet_host_service(self.host_ptr(), &mut event, timeout);
            if result < 0 {
                Err("Failed to host service!")
            } else if result > 0 {
                match event.etype {
                    ENetEventType::NONE => Ok(None),
                    ENetEventType::CONNECT => Ok(Some(ReNetEvent::Connect{
                        peer: &mut *(event.peer),
                    })),
                    ENetEventType::DISCONNECT => Ok(Some(ReNetEvent::Disconnect{
                        peer: &mut *(event.peer),
                        data: event.data,
                    })),
                    ENetEventType::RECEIVE =>  {
                        let rawPacket = &*(event.packet);
                        Ok(Some(ReNetEvent::Receive{
                            peer: &mut *(event.peer),
                            channelID: event.channelID,
                            packet: ReNetPacket {
                                data: from_raw_parts(rawPacket.data, rawPacket.dataLength as usize),
                                flags: rawPacket.flags,
                            }
                        }))
                    },
                    _ => Err("Unknown event!"),
                }
            } else {
                Ok(None)
            }
        }
    }
}

/**
* SERVER HOST
**/
pub struct ReNetServerHost {
    host: *mut ENetHost,
}

impl ReNetServerHost {
    pub fn new(
        address: ReNetAddress,
        peerCount: u32,
        incommingBandwith: u32,
        outgouingBandwith: u32,
    ) -> Option<ReNetServerHost> {
        unsafe {
            let r = enet_host_create(
                &address,
                peerCount as size_t,
                incommingBandwith,
                outgouingBandwith,
            );
            if r != ptr::null_mut() {
                Some(ReNetServerHost {
                    host: r
                })
            } else {
                None
            }
        }
    }
}

impl ReNetHost for ReNetServerHost {
    fn host_ptr(&mut self) -> *mut ENetHost {
        self.host
    }
}

impl Drop for ReNetServerHost {
    fn drop(&mut self) {
        unsafe {
            enet_host_destroy(self.host);
        }
    }
}

/**
* CLIENT HOST
**/
pub struct ReNetClientHost {
    host: *mut ENetHost,
}

impl ReNetClientHost {
    pub fn new(incommingBandwith: u32, outgouingBandwith: u32) -> Option<ReNetClientHost> {
        unsafe {
            let r = enet_host_create(
                0 as *const ENetAddress,
                1 as size_t,
                incommingBandwith,
                outgouingBandwith,
            );
            if r != ptr::null_mut() {
                Some(ReNetClientHost {
                    host: r
                })
            } else {
                None
            }
        }
    }
}

impl ReNetHost for ReNetClientHost {
    fn host_ptr(&mut self) -> *mut ENetHost {
        self.host
    }
}

impl Drop for ReNetClientHost {
    fn drop(&mut self) {
        unsafe {
            enet_host_destroy(self.host);
        }
    }
}
