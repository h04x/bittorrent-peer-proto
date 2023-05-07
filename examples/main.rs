use std::{net::TcpStream, sync::Arc};

use bittorrent_peer_proto::peer_proto::{PeerProto, Message};

//use bittorrent_peer_proto::{Message, PeerProto};

fn main() {
let addr = "1.1.1.1:1111";
let my_peer_id = [0; 20];
let info_hash = [0; 20];

let s = TcpStream::connect(addr).unwrap();
let t = PeerProto::handshake(s, info_hash, my_peer_id);
if t.is_err() {
    println!(
        "peer {} connected but handshake failed due to {:?}",
        addr, t
    );
}
let p = Arc::new(t.unwrap());

let msg = p.recv().unwrap();
let bitfield = match msg {
    Message::Bitfield(bf) => bf,
    _ => panic!()
};
}