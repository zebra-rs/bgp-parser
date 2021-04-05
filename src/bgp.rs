use rusticata_macros::newtype_enum;

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
