use nom_derive::Nom;
use rusticata_macros::newtype_enum;

#[derive(Debug, Nom, PartialEq, Eq)]
pub struct BgpPacketType(pub u8);

newtype_enum! {
    impl display BgpPacketType {
        Open = 1,
        Update = 2,
        Notification = 3,
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
pub struct BgpOpenPacket {
    pub header: BgpPacketHeader,
    pub version: u8,
    pub as_number: u16,
    pub hold_time: u16,
    pub bgp_id: u32,
}

#[derive(Debug, Nom)]
pub struct BgpUpdatePacket {
    pub header: BgpPacketHeader,
}

#[derive(Debug, Nom)]
pub struct BgpNotificationPacket {
    pub header: BgpPacketHeader,
    pub error_code: u8,
    pub error_subcode: u8,
}

#[derive(Debug, Nom)]
pub struct BgpKeepAlivePacket {
    pub header: BgpPacketHeader,
}

#[derive(Debug)]
pub enum BgpPacket {
    Open(BgpOpenPacket),
    Update(BgpUpdatePacket),
    Notification(BgpNotificationPacket),
    KeepAlive(BgpKeepAlivePacket),
}
