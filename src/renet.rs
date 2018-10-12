#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::native::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

pub use crate::native::ENET_HOST_ANY;
pub use crate::native::ENET_HOST_BROADCAST;
pub use crate::native::ENET_PORT_ANY;

pub fn renet_initialize() -> Result<(), &'static str> {
    unsafe {
        if enet_initialize() == 0 {
            Ok(())
        } else {
            Err("Failed to initialize enet")
        }
    }
}

pub fn renet_deinitialize() {
    unsafe {
        enet_deinitialize();
    }
}

/// Specifies ENet Address
pub type ReNetAddress = ENetAddress;

impl ReNetAddress {
    /// Creates new ENetAddress
    ///
    /// With `host` as ipv4 address and `port`
    pub fn new(host: u32, port: u16) -> ReNetAddress {
        ReNetAddress { host, port }
    }

    /// Attempts to set address to `&Cstr` representation of it
    pub fn set_host(&mut self, host_name: &CStr) -> Result<(), &'static str> {
        unsafe {
            if enet_address_set_host(self, host_name.as_ptr()) == 0 {
                Ok(())
            } else {
                Err("Bad address!")
            }
        }
    }

    /// Gets ip as string representation
    pub fn get_host_ip(&self) -> Option<CString> {
        let mut buffer: [c_char; 256] = [0; 256];
        unsafe {
            if enet_address_get_host_ip(self, buffer.as_mut_ptr(), 256) == 0 {
                Some(CString::from_raw(buffer.as_mut_ptr()))
            } else {
                None
            }
        }
    }

    /// Performs reverse lookup on ipv4 address and returns host name
    pub fn get_host(&self) -> Option<CString> {
        let mut buffer: [c_char; 256] = [0; 256];
        unsafe {
            if enet_address_get_host(self, buffer.as_mut_ptr(), 256) == 0 {
                Some(CString::from_raw(buffer.as_mut_ptr()))
            } else {
                None
            }
        }
    }
}

/// ENet packet flags
pub type ReNetPacketFlag = ENetPacketFlag;

impl From<u32> for ReNetPacketFlag {
    fn from(value: u32) -> Self {
        unsafe { *(&value as *const u32 as *const ReNetPacketFlag) }
    }
}

impl Into<u32> for ReNetPacketFlag {
    fn into(self) -> u32 {
        unsafe { *(&self as *const ReNetPacketFlag as *const enet_uint32) }
    }
}

/// Result of ENetEvent
pub enum ReNetEvent<'a> {
    None,
    Connect {
        peer: &'a mut ReNetPeer,
    },
    Disconnect {
        peer: &'a mut ReNetPeer,
        data: u32,
    },
    Receive {
        peer: &'a mut ReNetPeer,
        channelID: u8,
        data: Vec<u8>,
        flags: ReNetPacketFlag,
    },
}

/// ENetPeer
pub type ReNetPeer = ENetPeer;

impl ReNetPeer {
    pub fn send(
        &mut self,
        channelID: u8,
        data: &[u8],
        flags: ReNetPacketFlag,
    ) -> Result<(), &'static str> {
        unsafe {
            let packet =
                enet_packet_create(data.as_ptr() as *const c_void, data.len(), flags.into());
            if enet_peer_send(self, channelID, packet) == 0 {
                Ok(())
            } else {
                Err("Failed to send to peer")
            }
        }
    }

    pub fn receive(&mut self) -> Option<(u8, Vec<u8>, ReNetPacketFlag)> {
        unsafe {
            let mut channelID = 0;
            let rawPacket = enet_peer_receive(self, &mut channelID);
            if rawPacket.is_null() {
                None
            } else {
                let result = (
                    channelID,
                    Vec::from_raw_parts(
                        (*rawPacket).data,
                        (*rawPacket).dataLength,
                        (*rawPacket).dataLength,
                    ),
                    (*rawPacket).flags.into(),
                );
                enet_packet_destroy(rawPacket);
                Some(result)
            }
        }
    }

    pub fn ping(&mut self) {
        unsafe {
            enet_peer_ping(self);
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            enet_peer_reset(self);
        }
    }

    pub fn disconnect(&mut self, data: u32) {
        unsafe {
            enet_peer_disconnect(self, data);
        }
    }

    pub fn disconnect_now(&mut self, data: u32) {
        unsafe {
            enet_peer_disconnect_now(self, data);
        }
    }

    pub fn disconnect_later(&mut self, data: u32) {
        unsafe {
            enet_peer_disconnect_later(self, data);
        }
    }

    pub fn throttle_configure(&mut self, interval: u32, acceleration: u32, deceleration: u32) {
        unsafe {
            enet_peer_throttle_configure(self, interval, acceleration, deceleration);
        }
    }
}

/// Generic ENetHost
pub trait ReNetHost: Drop {
    fn host_ptr(&mut self) -> *mut ENetHost;

    fn flush(&mut self) {
        unsafe {
            enet_host_flush(self.host_ptr());
        }
    }
    fn bandwidth_limit(&mut self, incomingBandwidth: u32, outgoingBandwidth: u32) {
        unsafe {
            enet_host_bandwidth_limit(self.host_ptr(), incomingBandwidth, outgoingBandwidth);
        }
    }

    fn channel_limit(&mut self, channelLimit: u8) {
        unsafe {
            enet_host_channel_limit(self.host_ptr(), channelLimit as usize);
        }
    }

    fn broadcast(&mut self, channelID: u8, data: &[u8], flags: ReNetPacketFlag) {
        unsafe {
            let packet =
                enet_packet_create(data.as_ptr() as *const c_void, data.len(), flags.into());
            enet_host_broadcast(self.host_ptr(), channelID, packet);
        }
    }

    fn service<'a>(&'a mut self, timeout: Option<u32>) -> Result<ReNetEvent<'a>, &'static str> {
        unsafe {
            let mut event = ENetEvent {
                type_: ENetEventType::None,
                peer: ptr::null_mut(),
                channelID: 0,
                data: 0,
                packet: ptr::null_mut(),
            };

            match if let Some(time) = timeout {
                enet_host_service(self.host_ptr(), &mut event, time)
            } else {
                enet_host_service(self.host_ptr(), &mut event, 0)
            } {
                x if x > 0 => match event.type_ {
                    ENetEventType::None => Ok(ReNetEvent::None),
                    ENetEventType::Connect => Ok(ReNetEvent::Connect {
                        peer: &mut *(event.peer),
                    }),
                    ENetEventType::Disconnect => Ok(ReNetEvent::Disconnect {
                        peer: &mut *(event.peer),
                        data: event.data,
                    }),
                    ENetEventType::Receive => Ok(ReNetEvent::Receive {
                        peer: &mut *(event.peer),
                        channelID: event.channelID,
                        data: Vec::from_raw_parts(
                            (*event.packet).data,
                            (*event.packet).dataLength,
                            (*event.packet).dataLength,
                        ),
                        flags: (*event.packet).flags.into(),
                    }),
                },
                x if x < 0 => Err("Failed!"),
                _ => Ok(ReNetEvent::None),
            }
        }
    }
}

/// Client ENetHost
pub struct ReNetClientHost {
    host: *mut ENetHost,
}

impl ReNetClientHost {
    pub fn new(incomingBandwidth: u32, outgoingBandwidth: u32) -> Option<ReNetClientHost> {
        unsafe {
            let r = enet_host_create(ptr::null(), 1, incomingBandwidth, outgoingBandwidth);
            if !r.is_null() {
                Some(ReNetClientHost { host: r })
            } else {
                None
            }
        }
    }

    pub fn connect(
        &mut self,
        address: &ReNetAddress,
        channelCount: enet_uint8,
    ) -> Option<&mut ENetPeer> {
        unsafe {
            let peer = enet_host_connect(self.host_ptr(), address, channelCount as usize);
            if !peer.is_null() {
                Some(&mut (*peer))
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

/// Server ENetHost
pub struct ReNetServerHost {
    host: *mut ENetHost,
}

impl ReNetServerHost {
    pub fn new(
        address: ReNetAddress,
        peerCount: usize,
        incomingBandwidth: u32,
        outgoingBandwidth: enet_uint32,
    ) -> Option<ReNetServerHost> {
        unsafe {
            let r = enet_host_create(&address, peerCount, incomingBandwidth, outgoingBandwidth);
            if !r.is_null() {
                Some(ReNetServerHost { host: r })
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
