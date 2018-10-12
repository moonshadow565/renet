extern crate cc;

fn main() {
    cc::Build::new()
        .warnings(false)
        .include(".")
        .file("enet/callbacks.c")
        .file("enet/host.c")
        .file("enet/list.c")
        .file("enet/packet.c")
        .file("enet/peer.c")
        .file("enet/protocol.c")
        .file("enet/unix.c")
        .file("enet/win32.c")
        .compile("enet");
}
