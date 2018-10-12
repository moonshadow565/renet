#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub unsafe fn to_u32<T: Copy>(theEnum: T) -> u32 {
    *(&theEnum as *const T as *const u32)
}

pub unsafe fn from_u32<T: Copy>(value: u32) -> T {
    *(&value as *const u32 as *const T)
}

pub use std::option::Option;
pub use std::os::raw::*;

pub const ENET_HOST_ANY: u32 = 0;
pub const ENET_HOST_BROADCAST: u32 = 4294967295;
pub const ENET_PORT_ANY: u32 = 0;

pub const ENET_PROTOCOL_MINIMUM_MTU: u32 = 576;
pub const ENET_PROTOCOL_MAXIMUM_MTU: u32 = 4096;
pub const ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS: u32 = 32;
pub const ENET_PROTOCOL_MINIMUM_WINDOW_SIZE: u32 = 4096;
pub const ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE: u32 = 32768;
pub const ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT: u32 = 1;
pub const ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT: u32 = 255;
pub const ENET_PROTOCOL_MAXIMUM_PEER_ID: u32 = 32767;
pub const ENET_BUFFER_MAXIMUM: u32 = 65;
pub const ENET_HOST_RECEIVE_BUFFER_SIZE: u32 = 262144;
pub const ENET_HOST_SEND_BUFFER_SIZE: u32 = 262144;
pub const ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL: u32 = 1000;
pub const ENET_HOST_DEFAULT_MTU: u32 = 1400;
pub const ENET_PEER_DEFAULT_ROUND_TRIP_TIME: u32 = 500;
pub const ENET_PEER_DEFAULT_PACKET_THROTTLE: u32 = 32;
pub const ENET_PEER_PACKET_THROTTLE_SCALE: u32 = 32;
pub const ENET_PEER_PACKET_THROTTLE_COUNTER: u32 = 7;
pub const ENET_PEER_PACKET_THROTTLE_ACCELERATION: u32 = 2;
pub const ENET_PEER_PACKET_THROTTLE_DECELERATION: u32 = 2;
pub const ENET_PEER_PACKET_THROTTLE_INTERVAL: u32 = 5000;
pub const ENET_PEER_PACKET_LOSS_SCALE: u32 = 65536;
pub const ENET_PEER_PACKET_LOSS_INTERVAL: u32 = 10000;
pub const ENET_PEER_WINDOW_SIZE_SCALE: u32 = 65536;
pub const ENET_PEER_TIMEOUT_LIMIT: u32 = 32;
pub const ENET_PEER_TIMEOUT_MINIMUM: u32 = 5000;
pub const ENET_PEER_TIMEOUT_MAXIMUM: u32 = 30000;
pub const ENET_PEER_PING_INTERVAL: u32 = 500;
pub const ENET_PEER_UNSEQUENCED_WINDOWS: u32 = 64;
pub const ENET_PEER_UNSEQUENCED_WINDOW_SIZE: u32 = 1024;
pub const ENET_PEER_FREE_UNSEQUENCED_WINDOWS: u32 = 32;
pub const ENET_PEER_RELIABLE_WINDOWS: u32 = 16;
pub const ENET_PEER_RELIABLE_WINDOW_SIZE: u32 = 4096;
pub const ENET_PEER_FREE_RELIABLE_WINDOWS: u32 = 8;
pub type enet_uint8 = c_uchar;
pub type enet_uint16 = c_ushort;
pub type enet_uint32 = c_uint;
pub type ENetSocket = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetBuffer {
    pub data: *mut c_void,
    pub dataLength: usize,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ENetProtocolCommand {
    None = 0,
    Acknowledge = 1,
    Connect = 2,
    VerifyConnect = 3,
    Disconnect = 4,
    Ping = 5,
    SendReliable = 6,
    EndReliable = 7,
    SendFragment = 8,
    SendUnsequenced = 9,
    BandwidthLimit = 10,
    ThrottleConfigure = 11,
}

bitflags! {
    #[derive(Default)]
    #[repr(C)]
    pub struct ENetProtocolFlag : u32 {
        const Acknowledge = 128;
        const Unsequenced = 64;
        const SentTime = 32768;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolHeader {
    pub checksum: enet_uint32,
    pub peerID: enet_uint16,
    pub sentTime: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolCommandHeader {
    pub command: enet_uint8,
    pub channelID: enet_uint8,
    pub reliableSequenceNumber: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolAcknowledge {
    pub header: ENetProtocolCommandHeader,
    pub receivedReliableSequenceNumber: enet_uint16,
    pub receivedSentTime: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: enet_uint16,
    pub mtu: enet_uint16,
    pub windowSize: enet_uint32,
    pub channelCount: enet_uint32,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
    pub sessionID: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolVerifyConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: enet_uint16,
    pub mtu: enet_uint16,
    pub windowSize: enet_uint32,
    pub channelCount: enet_uint32,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolBandwidthLimit {
    pub header: ENetProtocolCommandHeader,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolThrottleConfigure {
    pub header: ENetProtocolCommandHeader,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolDisconnect {
    pub header: ENetProtocolCommandHeader,
    pub data: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolPing {
    pub header: ENetProtocolCommandHeader,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolSendReliable {
    pub header: ENetProtocolCommandHeader,
    pub dataLength: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolSendUnreliable {
    pub header: ENetProtocolCommandHeader,
    pub unreliableSequenceNumber: enet_uint16,
    pub dataLength: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolSendUnsequenced {
    pub header: ENetProtocolCommandHeader,
    pub unsequencedGroup: enet_uint16,
    pub dataLength: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolSendFragment {
    pub header: ENetProtocolCommandHeader,
    pub startSequenceNumber: enet_uint16,
    pub dataLength: enet_uint16,
    pub fragmentCount: enet_uint32,
    pub fragmentNumber: enet_uint32,
    pub totalLength: enet_uint32,
    pub fragmentOffset: enet_uint32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ENetProtocol {
    pub header: ENetProtocolCommandHeader,
    pub acknowledge: ENetProtocolAcknowledge,
    pub connect: ENetProtocolConnect,
    pub verifyConnect: ENetProtocolVerifyConnect,
    pub disconnect: ENetProtocolDisconnect,
    pub ping: ENetProtocolPing,
    pub sendReliable: ENetProtocolSendReliable,
    pub sendUnreliable: ENetProtocolSendUnreliable,
    pub sendUnsequenced: ENetProtocolSendUnsequenced,
    pub sendFragment: ENetProtocolSendFragment,
    pub bandwidthLimit: ENetProtocolBandwidthLimit,
    pub throttleConfigure: ENetProtocolThrottleConfigure,
    _union_align: [u32; 10usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetListNode {
    pub next: *mut ENetListNode,
    pub previous: *mut ENetListNode,
}

pub type ENetListIterator = *mut ENetListNode;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetList {
    pub sentinel: ENetListNode,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ENetSocketType {
    Stream = 1,
    Datagram = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ENetSocketWait {
    None = 0,
    Send = 1,
    Receive = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ENetSocketOption {
    NonBlock = 1,
    Broadcast = 2,
    RcvBuff = 3,
    SndBuff = 4,
    ReuseAddress = 5,
}

/// Portable internet address structure.
///
/// The host must be specified in network byte-order, and the port must be in host
/// byte-order. The constant `ENET_HOST_ANY` may be used to specify the default
/// server host. The constant `ENET_HOST_BROADCAST` may be used to specify the
/// broadcast address (255.255.255.255).  This makes sense for `enet_host_connect`,
/// but not for `enet_host_create`.  Once a server responds to a broadcast, the
/// address is updated from `ENET_HOST_BROADCAST` to the server's actual IP address.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetAddress {
    pub host: enet_uint32,
    pub port: enet_uint16,
}

bitflags! {
    /// Packet flag bit constants.
    ///
    /// The host must be specified in network byte-order, and the port must be in
    /// host byte-order. The constant `ENET_HOST_ANY` may be used to specify the
    /// default server host.
    #[repr(C)]
    #[derive(Default)]
    pub struct ENetPacketFlag : u32 {
        /// Packet must be received by the target peer and resend attempts should be
        /// made until the packet is delivered
        const Reliable = 1;
        /// Packet will not be sequenced with other packets
        /// not supported for reliable packets
        const Unsequenced = 2;
        /// Packet will not allocate data, and user must supply it instead
        const NoAllocate = 4;
    }
}

pub type ENetPacketFreeCallback = Option<unsafe extern "C" fn(arg1: *mut ENetPacket)>;

/// ENet packet structure.
///
/// An ENet data packet that may be sent to or received from a peer. The shown
/// fields should only be read and never modified. The data field contains the
/// allocated data for the packet. The `dataLength` fields specifies the length
/// of the allocated data.  The flags field is either 0 (specifying no flags),
/// or a bitwise-or of any combination of the following flags:
///
///    `ENET_PACKET_FLAG_RELIABLE` - packet must be received by the target peer
///    and resend attempts should be made until the packet is delivered
///
///    `ENET_PACKET_FLAG_UNSEQUENCED` - packet will not be sequenced with other packets
///    (not supported for reliable packets)
///
///    `ENET_PACKET_FLAG_NO_ALLOCATE` - packet will not allocate data, user must supply it instead
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetPacket {
    ///< internal use only
    pub referenceCount: usize,
    ///< bitwise-or of ENetPacketFlag constants
    pub flags: enet_uint32,
    ///< allocated data for packet
    pub data: *mut enet_uint8,
    ///< length of data
    pub dataLength: usize,
    ///< function to be called when the packet is no longer in use
    pub freeCallback: ENetPacketFreeCallback,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ENetAcknowledgement {
    pub acknowledgementList: ENetListNode,
    pub sentTime: enet_uint32,
    pub command: ENetProtocol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ENetOutgoingCommand {
    pub outgoingCommandList: ENetListNode,
    pub reliableSequenceNumber: enet_uint16,
    pub unreliableSequenceNumber: enet_uint16,
    pub sentTime: enet_uint32,
    pub roundTripTimeout: enet_uint32,
    pub roundTripTimeoutLimit: enet_uint32,
    pub fragmentOffset: enet_uint32,
    pub fragmentLength: enet_uint16,
    pub sendAttempts: enet_uint16,
    pub command: ENetProtocol,
    pub packet: *mut ENetPacket,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ENetIncomingCommand {
    pub incomingCommandList: ENetListNode,
    pub reliableSequenceNumber: enet_uint16,
    pub unreliableSequenceNumber: enet_uint16,
    pub command: ENetProtocol,
    pub fragmentCount: enet_uint32,
    pub fragmentsRemaining: enet_uint32,
    pub fragments: *mut enet_uint32,
    pub packet: *mut ENetPacket,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ENetPeerState {
    Disconnected = 0,
    Connecting = 1,
    AcknowledgingConnect = 2,
    ConnectionPending = 3,
    ConnectionSucceeded = 4,
    Connected = 5,
    DisconnectLatter = 6,
    Disconnecting = 7,
    AcknowledgingDisconnect = 8,
    Zombie = 9,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetChannel {
    pub outgoingReliableSequenceNumber: enet_uint16,
    pub outgoingUnreliableSequenceNumber: enet_uint16,
    pub usedReliableWindows: enet_uint16,
    pub reliableWindows: [enet_uint16; 16usize],
    pub incomingReliableSequenceNumber: enet_uint16,
    pub incomingUnreliableSequenceNumber: enet_uint16,
    pub incomingReliableCommands: ENetList,
    pub incomingUnreliableCommands: ENetList,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetPeer {
    pub dispatchList: ENetListNode,
    pub host: *mut ENetHost,
    pub outgoingPeerID: enet_uint16,
    pub incomingPeerID: enet_uint16,
    pub sessionID: enet_uint32,
    ///< Internet address of the peer
    pub address: ENetAddress,
    ///< Application private data, may be freely modified
    pub data: *mut c_void,
    pub state: ENetPeerState,
    pub channels: *mut ENetChannel,
    ///< Number of channels allocated for communication with peer
    pub channelCount: usize,
    ///< Downstream bandwidth of the client in bytes/second
    pub incomingBandwidth: enet_uint32,
    ///< Upstream bandwidth of the client in bytes/second
    pub outgoingBandwidth: enet_uint32,
    pub incomingBandwidthThrottleEpoch: enet_uint32,
    pub outgoingBandwidthThrottleEpoch: enet_uint32,
    pub incomingDataTotal: enet_uint32,
    pub outgoingDataTotal: enet_uint32,
    pub lastSendTime: enet_uint32,
    pub lastReceiveTime: enet_uint32,
    pub nextTimeout: enet_uint32,
    pub earliestTimeout: enet_uint32,
    pub packetLossEpoch: enet_uint32,
    pub packetsSent: enet_uint32,
    pub packetsLost: enet_uint32,
    ///< mean packet loss of reliable packets as a ratio with
    /// respect to the constant `ENET_PEER_PACKET_LOSS_SCALE`
    pub packetLoss: enet_uint32,
    pub packetLossVariance: enet_uint32,
    pub packetThrottle: enet_uint32,
    pub packetThrottleLimit: enet_uint32,
    pub packetThrottleCounter: enet_uint32,
    pub packetThrottleEpoch: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub lastRoundTripTime: enet_uint32,
    pub lowestRoundTripTime: enet_uint32,
    pub lastRoundTripTimeVariance: enet_uint32,
    pub highestRoundTripTimeVariance: enet_uint32,
    ///< mean round trip time (RTT), in milliseconds,
    /// between sending a reliable packet and receiving its acknowledgement
    pub roundTripTime: enet_uint32,
    pub roundTripTimeVariance: enet_uint32,
    pub mtu: enet_uint16,
    pub windowSize: enet_uint32,
    pub reliableDataInTransit: enet_uint32,
    pub outgoingReliableSequenceNumber: enet_uint16,
    pub acknowledgements: ENetList,
    pub sentReliableCommands: ENetList,
    pub sentUnreliableCommands: ENetList,
    pub outgoingReliableCommands: ENetList,
    pub outgoingUnreliableCommands: ENetList,
    pub dispatchedCommands: ENetList,
    pub needsDispatch: c_int,
    pub incomingUnsequencedGroup: enet_uint16,
    pub outgoingUnsequencedGroup: enet_uint16,
    pub unsequencedWindow: [enet_uint32; 32usize],
    pub disconnectData: enet_uint32,
}

/// Callback that computes the checksum of the data held in buffers `[0..bufferCount-1]`
pub type ENetChecksumCallback =
    Option<unsafe extern "C" fn(buffers: *const ENetBuffer, bufferCount: usize) -> enet_uint32>;

/// An ENet peer which data packets may be sent or received from.
///
/// No fields should be modified unless otherwise specified.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ENetHost {
    pub socket: ENetSocket,
    ///< Internet address of the host
    pub address: ENetAddress,
    ///< downstream bandwidth of the host
    pub incomingBandwidth: enet_uint32,
    ///< upstream bandwidth of the host
    pub outgoingBandwidth: enet_uint32,
    pub bandwidthThrottleEpoch: enet_uint32,
    pub mtu: enet_uint32,
    pub recalculateBandwidthLimits: c_int,
    ///< array of peers allocated for this host
    pub peers: *mut ENetPeer,
    ///< number of peers allocated for this host
    pub peerCount: usize,
    ///< maximum number of channels allowed for connected peers
    pub channelLimit: usize,
    pub serviceTime: enet_uint32,
    pub dispatchQueue: ENetList,
    pub continueSending: c_int,
    pub packetSize: usize,
    pub headerFlags: enet_uint16,
    pub commands: [ENetProtocol; 32usize],
    pub commandCount: usize,
    pub buffers: [ENetBuffer; 65usize],
    pub bufferCount: usize,
    pub checksum: ENetChecksumCallback,
    pub receivedAddress: ENetAddress,
    pub receivedData: [enet_uint8; 4096usize],
    pub receivedDataLength: usize,
    ///< total data sent, user should reset to 0 as needed to prevent overflow
    pub totalSentData: enet_uint32,
    ///< total UDP packets sent, user should reset to 0 as needed to prevent overflow
    pub totalSentPackets: enet_uint32,
    ///< total data received, user should reset to 0 as needed to prevent overflow
    pub totalReceivedData: enet_uint32,
    ///< total UDP packets received, user should reset to 0 as needed to prevent overflow
    pub totalReceivedPackets: enet_uint32,
}

#[repr(u32)]
/// An ENet event type, as specified in @ref ENetEvent.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ENetEventType {
    /// no event occurred within the specified time limit
    None = 0,
    /// a connection request initiated by `enet_host_connect` has completed.
    /// The peer field contains the peer which successfully connected.
    Connect = 1,
    /// a peer has disconnected.  This event is generated on a successful
    /// completion of a disconnect initiated by `enet_peer_disconnect`, if
    /// a peer has timed out, or if a connection request intialized by
    /// `enet_host_connect` has timed out.  The peer field contains the peer
    /// which disconnected. The data field contains user supplied data
    /// describing the disconnection, or 0, if none is available.
    Disconnect = 2,
    /// a packet has been received from a peer.  The peer field specifies the
    /// peer which sent the packet.  The `channelID` field specifies the channel
    /// number upon which the packet was received.  The packet field contains
    /// the packet that was received; this packet must be destroyed with
    /// enet_packet_destroy after use.
    Receive = 3,
}

/// An ENet event as returned by `enet_host_service()`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetEvent {
    ///< type of the event
    pub type_: ENetEventType,
    ///< peer that generated a connect, disconnect or receive event
    pub peer: *mut ENetPeer,
    ///< channel on the peer that generated the event, if appropriate
    pub channelID: enet_uint8,
    ///< data associated with the event, if appropriate
    pub data: enet_uint32,
    ///< packet associated with the event, if appropriate
    pub packet: *mut ENetPacket,
}

#[link(name = "enet")]
extern "C" {
    /// Initializes ENet globally.  Must be called prior to using any functions in
    /// ENet.
    ///
    /// # Returns
    ///
    /// `== 0` on success
    ///
    /// `< 0` on failure
    pub fn enet_initialize() -> c_int;

    /// Shuts down ENet globally.  Should be called when a program that has
    /// initialized ENet exits.
    pub fn enet_deinitialize();

    pub fn enet_socket_create(type_: ENetSocketType) -> ENetSocket;

    pub fn enet_socket_bind(socket: ENetSocket, address: *const ENetAddress) -> c_int;

    pub fn enet_socket_listen(socket: ENetSocket, backlog: c_int) -> c_int;

    pub fn enet_socket_accept(socket: ENetSocket, address: *mut ENetAddress) -> ENetSocket;

    pub fn enet_socket_connect(socket: ENetSocket, address: *const ENetAddress) -> c_int;

    pub fn enet_socket_send(
        socket: ENetSocket,
        address: *const ENetAddress,
        buffers: *const ENetBuffer,
        bufferCount: usize,
    ) -> c_int;

    pub fn enet_socket_receive(
        socket: ENetSocket,
        address: *mut ENetAddress,
        buffers: *mut ENetBuffer,
        bufferCount: usize,
    ) -> c_int;

    pub fn enet_socket_wait(
        socket: ENetSocket,
        condition: *mut enet_uint32,
        timeout: enet_uint32,
    ) -> c_int;

    pub fn enet_socket_set_option(
        socket: ENetSocket,
        option: ENetSocketOption,
        value: c_int,
    ) -> c_int;

    pub fn enet_socket_destroy(socket: ENetSocket);

    /// Attempts to resolve the host named by the parameter hostName and sets
    /// the host field in the address parameter if successful.
    ///
    /// `address`: destination to store resolved address
    ///
    /// `hostName`: host name to lookup
    ///
    /// # Returns
    ///
    /// the address of the given hostName in address on success
    ///
    /// `retval` 0 on success
    ///
    /// `retval` < 0 on failure
    pub fn enet_address_set_host(address: *mut ENetAddress, hostName: *const c_char) -> c_int;

    /// Gives the printable form of the ip address specified in the address parameter.
    ///
    /// `address`: address printed
    ///
    /// `hostName`: destination for name, must not be `NULL`
    ///
    /// `nameLength`: maximum length of `hostName`.
    ///
    /// # Returns
    ///
    /// the null-terminated name of the host in `hostName` on success
    ///
    /// `retval 0` on success
    ///
    /// `retval < 0` on failure
    pub fn enet_address_get_host_ip(
        address: *const ENetAddress,
        hostName: *mut c_char,
        nameLength: usize,
    ) -> c_int;

    /// Attempts to do a reverse lookup of the host field in the address parameter.
    ///
    /// `address`: used for reverse lookup
    ///
    /// `hostName`: destination for name, must not be `NULL`
    ///
    /// `nameLength`: is maximum length of `hostName`.
    ///
    /// # Returns
    ///
    /// the null-terminated name of the host in `hostName` on success
    ///
    /// `retval 0` on success
    ///
    /// `retval < 0` on failure
    pub fn enet_address_get_host(
        address: *const ENetAddress,
        hostName: *mut c_char,
        nameLength: usize,
    ) -> c_int;

    /// Creates a packet that may be sent to a peer.
    ///
    /// `dataContents`: initial contents of the packet's data;
    /// the packet's data will remain uninitialized if dataContents is `NULL`.
    ///
    /// `dataLength`: size of the data allocated for this packet
    ///
    /// `flags`: for this packet as described for the `ENetPacket` structure.
    ///
    /// # Returns
    ///
    /// the `packet` on success, `NULL` on failure
    pub fn enet_packet_create(
        data: *const c_void,
        dataLength: usize,
        flags: enet_uint32,
    ) -> *mut ENetPacket;

    /// Destroys the packet and deallocates its data.
    ///
    /// `packet`: to be destroyed
    pub fn enet_packet_destroy(packet: *mut ENetPacket);

    /// Attempts to resize the data in the packet to length specified in the
    /// dataLength parameter
    ///
    /// `packet`: to resize
    ///
    /// `dataLength`: new size for the packet data
    ///
    /// # Returns
    ///
    /// `= 0` on success
    ///
    /// `< 0` on failure
    pub fn enet_packet_resize(packet: *mut ENetPacket, dataLength: usize) -> c_int;

    /// Creates a host for communicating to peers.
    ///
    /// `address`: at which other peers may connect to this host.
    /// If NULL, then no peers may connect to the host.
    ///
    /// `peerCount`: the maximum number of peers that should be allocated for the host.
    ///
    /// `incomingBandwidth`: downstream bandwidth of the host in bytes/second;
    /// if 0, ENet will assume unlimited bandwidth.
    ///
    /// `outgoingBandwidth`: upstream bandwidth of the host in bytes/second;
    /// if 0, ENet will assume unlimited bandwidth.
    ///
    /// # Returns
    ///
    /// the `host` on success and NULL on failure
    ///
    /// # Remarks
    ///
    /// ENet will strategically drop packets on specific sides of a connection between hosts
    /// to ensure the host's bandwidth is not overwhelmed.  The bandwidth parameters also determine
    /// the window size of a connection which limits the amount of reliable packets
    /// that may be in transit at any given time.
    pub fn enet_host_create(
        address: *const ENetAddress,
        peerCount: usize,
        incomingBandwidth: enet_uint32,
        outgoingBandwidth: enet_uint32,
    ) -> *mut ENetHost;

    /// Destroys the host and all resources associated with it.
    ///@param host pointer to the host to destroy
    pub fn enet_host_destroy(host: *mut ENetHost);

    /// Initiates a connection to a foreign host.
    ///
    /// `host`: seeking the connection
    ///
    /// `address`: destination for the connection
    ///
    /// `channelCount`: number of channels to allocate
    ///
    /// # Returns
    ///
    /// a peer representing the foreign host on success, NULL on failure
    ///
    /// # Remarks
    ///
    /// The peer returned will have not completed the connection until `enet_host_service()`
    /// notifies of an `ENET_EVENT_TYPE_CONNECT` event for the peer.
    pub fn enet_host_connect(
        host: *mut ENetHost,
        address: *const ENetAddress,
        channelCount: usize,
    ) -> *mut ENetPeer;

    /// Checks for any queued events on the host and dispatches one if available.
    ///
    /// `host`: to check for events
    ///
    /// `event`: structure where event details will be placed if available
    ///
    ///  # Returns
    ///
    /// `retval > 0` if an event was dispatched
    ///
    /// `retval 0` if no events are available
    ///
    /// `retval < 0` on failure
    pub fn enet_host_check_events(host: *mut ENetHost, event: *mut ENetEvent) -> c_int;

    /// Waits for events on the host specified and shuttles packets between
    /// the host and its peers.
    ///
    /// `host`: to service
    ///
    /// `event`: structure where event details will be placed if one occurs
    /// if `event == NULL` then no events will be delivered
    ///
    /// `timeout` number of milliseconds that ENet should wait for events
    ///
    /// # Returns
    ///
    /// `retval > 0` if an event occurred within the specified time limit
    ///
    /// `retval 0` if no event occurred
    ///
    /// `retval < 0` on failure
    ///
    /// # Remarks
    ///
    /// `enet_host_service` should be called fairly regularly for adequate performance
    pub fn enet_host_service(
        host: *mut ENetHost,
        event: *mut ENetEvent,
        timeout: enet_uint32,
    ) -> c_int;

    /// Sends any queued packets on the host specified to its designated peers.
    ///
    /// `host`: to flush
    ///
    /// # Remarks
    ///
    /// this function need only be used in circumstances where one wishes to send queued
    /// packets earlier than in a call to `enet_host_service()`.
    pub fn enet_host_flush(host: *mut ENetHost);

    /// Queues a packet to be sent to all peers associated with the host.
    ///
    /// `host`: on which to broadcast the packet
    ///
    /// `channelID`: channel on which to broadcast
    ///
    /// `packet`: to broadcast
    pub fn enet_host_broadcast(host: *mut ENetHost, channelID: enet_uint8, packet: *mut ENetPacket);

    /// Limits the maximum allowed channels of future incoming connections.
    ///
    /// `host`: to limit
    ///
    /// `channelLimit`: the maximum number of channels allowed
    /// if 0, then this is equivalent to `ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT`
    pub fn enet_host_channel_limit(host: *mut ENetHost, channelLimit: usize);

    /// Adjusts the bandwidth limits of a host.
    ///
    /// `host`: to adjust
    ///
    /// `incomingBandwidth`: new incoming bandwidth
    ///
    /// `outgoingBandwidth`: new outgoing bandwidth
    ///
    /// # Remarks
    /// The incoming and outgoing bandwidth parameters are identical in function to those
    /// specified in `enet_host_create()`.
    pub fn enet_host_bandwidth_limit(
        host: *mut ENetHost,
        incomingBandwidth: enet_uint32,
        outgoingBandwidth: enet_uint32,
    );

    /// Queues a packet to be sent.
    ///
    /// `peer`: destination for the packet
    ///
    /// `channelID`: channel on which to send
    ///
    ///  # Returns
    ///
    /// `packet` to send
    ///
    /// `retval == 0` on success
    ///
    /// `retval < 0` on failure
    pub fn enet_peer_send(
        peer: *mut ENetPeer,
        channelID: enet_uint8,
        packet: *mut ENetPacket,
    ) -> c_int;

    /// Attempts to dequeue any incoming queued packet.
    ///
    /// `peer`: to dequeue packets from
    ///
    /// `channelID`: holds the channel ID of the channel the packet was received on success
    ///
    /// # Returns
    ///
    ///  a pointer to the packet, or NULL if there are no available incoming queued packets
    ///
    pub fn enet_peer_receive(peer: *mut ENetPeer, channelID: *mut enet_uint8) -> *mut ENetPacket;

    /// Sends a ping request to a peer.
    ///
    /// `peer`: destination for the ping request
    ///
    /// # Remarks
    ///
    /// Ping requests factor into the mean round trip time as designated by the
    /// roundTripTime field in the `ENetPeer` structure.  Enet automatically pings all connected
    /// peers at regular intervals, however, this function may be called to ensure more
    /// frequent ping requests.
    pub fn enet_peer_ping(peer: *mut ENetPeer);

    /// Forcefully disconnects a peer.
    ///
    /// `peer`: to forcefully disconnect
    ///
    /// # Remarks
    ///
    /// The foreign host represented by the peer is not notified of the disconnection
    /// and will timeout
    /// on its connection to the local host.
    pub fn enet_peer_reset(peer: *mut ENetPeer);

    /// Request a disconnection from a peer.
    ///
    /// `peer`: to request a disconnection
    ///
    /// `data`: describing the disconnection
    ///
    /// # Remarks
    ///
    /// An `ENET_EVENT_DISCONNECT` event will be generated by `enet_host_service()`
    /// once the disconnection is complete.
    pub fn enet_peer_disconnect(peer: *mut ENetPeer, data: enet_uint32);

    /// Force an immediate disconnection from a peer.
    ///
    /// `peer`: to disconnect
    ///
    /// `data`: describing the disconnection
    ///
    /// # Remarks
    ///
    /// No `ENET_EVENT_DISCONNECT` event will be generated. The foreign peer is not
    /// guaranteed to receive the disconnect notification, and is reset immediately upon
    /// return from this function.
    pub fn enet_peer_disconnect_now(peer: *mut ENetPeer, data: enet_uint32);

    /// Request a disconnection from a peer, but only after all queued outgoing packets are sent.
    ///
    /// `peer`: to request a disconnection
    ///
    /// `data`: data describing the disconnection
    ///
    /// # Remarks
    ///
    /// An `ENET_EVENT_DISCONNECT` event will be generated by `enet_host_service()`
    /// once the disconnection is complete.
    pub fn enet_peer_disconnect_later(peer: *mut ENetPeer, data: enet_uint32);

    /// Configures throttle parameter for a peer.
    ///
    /// `peer`: to configure
    ///
    /// `interval`: interval, in milliseconds, over which to measure lowest mean RTT;
    ///  the default value is `ENET_PEER_PACKET_THROTTLE_INTERVAL`.
    ///
    /// `acceleration`: rate at which to increase the throttle probability as mean RTT declines
    ///
    /// `deceleration`: rate at which to decrease the throttle probability as mean RTT increases
    ///
    /// # Remarks
    ///
    /// Unreliable packets are dropped by ENet in response to the varying conditions
    /// of the Internet connection to the peer.  The throttle represents a probability
    /// that an unreliable packet should not be dropped and thus sent by ENet to the peer.
    /// The lowest mean round trip time from the sending of a reliable packet to the
    /// receipt of its acknowledgement is measured over an amount of time specified by
    /// the interval parameter in milliseconds.  If a measured round trip time happens to
    /// be significantly less than the mean round trip time measured over the interval,
    /// then the throttle probability is increased to allow more traffic by an amount
    /// specified in the acceleration parameter, which is a ratio to the
    /// `ENET_PEER_PACKET_THROTTLE_SCALE` constant.  If a measured round trip time
    /// happens to be significantly greater than
    /// the mean round trip time measured over the interval, then the throttle probability
    /// is decreased to limit traffic by an amount specified in the deceleration parameter, which
    /// is a ratio to the `ENET_PEER_PACKET_THROTTLE_SCALE` constant.  When the throttle has
    /// a value of `ENET_PEER_PACKET_THROTTLE_SCALE`, on unreliable packets are dropped by
    /// ENet, and so 100% of all unreliable packets will be sent.  When the throttle has a
    /// value of 0, all unreliable packets are dropped by ENet, and so 0% of all unreliable
    /// packets will be sent.  Intermediate values for the throttle represent intermediate
    /// probabilities between 0% and 100% of unreliable packets being sent.  The bandwidth
    /// limits of the local and foreign hosts are taken into account to determine a
    /// sensible limit for the throttle probability above which it should not raise even in
    /// the best of conditions.
    pub fn enet_peer_throttle_configure(
        peer: *mut ENetPeer,
        interval: enet_uint32,
        acceleration: enet_uint32,
        deceleration: enet_uint32,
    );
}
