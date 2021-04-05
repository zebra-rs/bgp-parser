use nom_derive::Nom;
use rusticata_macros::newtype_enum;

#[derive(Debug, Nom, PartialEq, Eq)]
pub struct BgpPacketType(pub u8);

newtype_enum! {
    impl display BgpPacketType {
        Open = 1,
        Update = 2,
        Notify = 3,
        KeepAlive = 4,
        RouteRefresh = 5,
        Capability  = 6,
        RouteRefreshOld = 128,
    }
}

#[derive(Debug, Nom)]
pub struct BgpPacketHeader {
    pub packet_length: u16,
    pub packet_type: BgpPacketType,
}

#[derive(Debug, Nom)]
pub struct BgpKeepAlivePacket {
    pub header: BgpPacketHeader,
}

#[derive(Debug)]
pub enum BgpPacket {
    KeepAlive(BgpKeepAlivePacket),
}
