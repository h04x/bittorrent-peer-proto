use core::fmt;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
};

use lava_torrent::bencode::BencodeElem;
use thiserror::Error;

//use crate::{NAME, UT_PEX_EXTENDED_MSG_ID};

const UT_PEX_EXTENDED_MSG_ID: u8 = 1;

pub fn prepend(l: &[u8], r: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    ret.extend_from_slice(l);
    ret.extend_from_slice(r);
    ret
}

pub struct Bitfield {
    bitfield: Vec<u8>,
}

impl fmt::Debug for Bitfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bitfield")
            .field("bitfield(bits)", &(self.bitfield.len() * 8))
            .finish()
    }
}

impl Bitfield {
    pub fn new(bit_cnt: u32) -> Bitfield {
        Bitfield {
            bitfield: vec![0; (bit_cnt as f32 / 8.0).ceil() as usize],
        }
    }
    pub fn try_from_bytes(raw: &[u8]) -> Result<Bitfield, Error> {
        if raw.is_empty() {
            return Err(Error::InvalidMsgLen);
        }
        Ok(Bitfield {
            bitfield: raw.to_vec(),
        })
    }
    pub fn bytes(&self) -> Vec<u8> {
        unimplemented!()
    }
    pub fn get(&self, n: usize) -> Option<bool> {
        let byte = n / 8;
        let bit = n % 8;
        Some((self.bitfield.get(byte)? >> (7 - bit)) & 1 == 1)
    }
    pub fn set(&mut self, index: usize, flag: bool) {
        let byte = index / 8;
        let bit = index % 8;
        if let Some(b) = self.bitfield.get_mut(byte) {
            if flag {
                *b |= 0b10000000u8 >> bit;
            } else {
                *b &= !(0b10000000u8 >> bit);
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Inmvalid message length")]
    InvalidMsgLen,
    #[error("Lava torrent error")]
    LavaTorreent(#[from] lava_torrent::LavaTorrentError),
    #[error("Empty extended payload")]
    EmptyExtended,
}

#[derive(Debug)]
pub struct Have {
    pub piece_index: u32,
}

impl Have {
    pub fn try_from_bytes(raw: &[u8]) -> Result<Have, Error> {
        if raw.len() != 4 {
            return Err(Error::InvalidMsgLen);
        }
        Ok(Have {
            piece_index: u32::from_be_bytes(raw.try_into().unwrap()),
        })
    }
    pub fn bytes(&self) -> [u8; 4] {
        self.piece_index.to_be_bytes()
    }
}

#[derive(Debug)]
pub struct Request {
    index: u32,
    begin: u32,
    len: u32,
}

impl Request {
    pub fn new(index: u32, begin: u32, len: u32) -> Request {
        Request { index, begin, len }
    }
    pub fn try_from_bytes(raw: &[u8]) -> Result<Request, Error> {
        if raw.len() != 12 {
            return Err(Error::InvalidMsgLen);
        }
        Ok(Request {
            index: u32::from_be_bytes(raw[0..4].try_into().unwrap()),
            begin: u32::from_be_bytes(raw[4..8].try_into().unwrap()),
            len: u32::from_be_bytes(raw[8..].try_into().unwrap()),
        })
    }
    pub fn bytes(&self) -> [u8; 12] {
        let mut raw = Vec::new();
        raw.extend_from_slice(&self.index.to_be_bytes());
        raw.extend_from_slice(&self.begin.to_be_bytes());
        raw.extend_from_slice(&self.len.to_be_bytes());
        raw.try_into().unwrap()
    }
}

pub struct Piece {
    pub index: u32,
    pub begin: u32,
    pub block: Vec<u8>,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Piece")
            .field("index", &self.index)
            .field("begin", &self.begin)
            .field("block(bytes)", &self.block.len())
            .finish()
    }
}

impl Piece {
    pub fn try_from_bytes(raw: &[u8]) -> Result<Piece, Error> {
        if raw.len() < 8 {
            return Err(Error::InvalidMsgLen);
        }
        Ok(Piece {
            index: u32::from_be_bytes(raw[0..4].try_into().unwrap()),
            begin: u32::from_be_bytes(raw[4..8].try_into().unwrap()),
            block: raw[8..].to_vec(),
        })
    }
    pub fn bytes(&self) -> Vec<u8> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Cancel {
    index: u32,
    begin: u32,
    len: u32,
}

impl Cancel {
    pub fn try_from_bytes(raw: &[u8]) -> Result<Cancel, Error> {
        if raw.len() != 12 {
            return Err(Error::InvalidMsgLen);
        }
        Ok(Cancel {
            index: u32::from_be_bytes(raw[0..4].try_into().unwrap()),
            begin: u32::from_be_bytes(raw[4..8].try_into().unwrap()),
            len: u32::from_be_bytes(raw[8..].try_into().unwrap()),
        })
    }
    pub fn bytes(&self) -> [u8; 12] {
        let mut raw = Vec::new();
        raw.extend_from_slice(&self.index.to_be_bytes());
        raw.extend_from_slice(&self.begin.to_be_bytes());
        raw.extend_from_slice(&self.len.to_be_bytes());
        raw.try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Port {
    pub listen_port: u16,
}

impl Port {
    pub fn try_from_bytes(raw: &[u8]) -> Result<Port, Error> {
        if raw.len() != 2 {
            return Err(Error::InvalidMsgLen);
        }
        Ok(Port {
            listen_port: u16::from_be_bytes(raw.try_into().unwrap()),
        })
    }
    pub fn bytes(&self) -> [u8; 2] {
        let mut raw = Vec::new();
        raw.extend_from_slice(&self.listen_port.to_be_bytes());
        raw.try_into().unwrap()
    }
}

#[derive(Debug)]
// TODO: parse other PEB 11 fields
pub struct UtPex {
    pub added: Vec<SocketAddr>,
}

impl UtPex {
    pub fn try_from_bytes(raw: &[u8]) -> Result<UtPex, Error> {
        let msg = BencodeElem::from_bytes(raw)?
            .first()
            .cloned()
            .ok_or(Error::EmptyExtended)?;

        let mut added = Vec::new();
        if let BencodeElem::Dictionary(d) = msg {
            if let Some(BencodeElem::Bytes(bytes)) = d.get("added") {
                for chunk in bytes.chunks_exact(6) {
                    let (ip, port) = chunk.split_at(4);
                    let ip = IpAddr::from(<[u8; 4]>::try_from(ip).unwrap());
                    let port = u16::from_be_bytes(<[u8; 2]>::try_from(port).unwrap());
                    added.push(SocketAddr::new(ip, port));
                }
            }
        }
        Ok(UtPex { added })
    }
    pub fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Extended {
    Handshake(BencodeElem),
    UtPex(UtPex),
    Unknown(Vec<u8>),
}

impl Extended {
    pub fn try_from_bytes(raw: &[u8]) -> Result<Extended, Error> {
        match raw.first() {
            Some(0) => Ok(Extended::Handshake(
                BencodeElem::from_bytes(raw)?
                    .first()
                    .cloned()
                    .ok_or(Error::EmptyExtended)?,
            )),
            Some(&UT_PEX_EXTENDED_MSG_ID) => Ok(Self::UtPex(UtPex::try_from_bytes(&raw[1..])?)),
            Some(_) => Ok(Self::Unknown(raw.to_vec())),
            None => Err(Error::InvalidMsgLen),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Self::Handshake(hs) => prepend(&[0], &hs.encode()),
            Self::UtPex(msg) => prepend(&[UT_PEX_EXTENDED_MSG_ID], &msg.bytes()),
            Self::Unknown(_) => todo!(),
        }
    }

    pub fn handshake() -> Extended {
        let p = HashMap::from([(
            "ut_pex".to_string(),
            BencodeElem::Integer(UT_PEX_EXTENDED_MSG_ID as i64),
        )]);
        let pd = BencodeElem::Dictionary(p);
        let m = HashMap::from([("m".to_string(), pd)]);
        let md = BencodeElem::Dictionary(m);
        Extended::Handshake(md)
        //Extended::Handshake()
    }
}
