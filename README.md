# bittorrent-peer-proto
Partial implementations  
 **peer protocol** section from [BEP03](https://www.bittorrent.org/beps/bep_0003.html)  
 **BitTorrent Protocol Extension** section from [BEP05](https://www.bittorrent.org/beps/bep_0005.html)  
 [BEP10](https://www.bittorrent.org/beps/bep_0010.html)  
 [BEP11](https://www.bittorrent.org/beps/bep_0011.html)  
 

## Example
```Rust
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
        _ => panic!(),
    };
}
```