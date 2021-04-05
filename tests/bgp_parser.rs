use bgp_parser::*;
use hex_literal::hex;

#[test]
pub fn test_keepalive_packet() {
    const BGP_KEEPALIVE: &[u8] = &hex!(
        "
ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff
00 13 04
        "
    );
    let (rem, res) = parse_bgp_packet(BGP_KEEPALIVE).expect("parse is failed");
    assert!(rem.is_empty());
    if let BgpPacket::KeepAlive(pkt) = res {
        assert_eq!(pkt.header.packet_length, 19);
        assert_eq!(pkt.header.packet_type, BgpPacketType::KeepAlive);
    } else {
        panic!("wrong packet type");
    }
}

// #[test]
// pub fn test_wrong_marker() {
//     const BGP_KEEPALIVE: &[u8] = &hex!(
//         "
// ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff fe
// 00 13 04
//         "
//     );
//     let (rem, res) = parse_bgp_packet(BGP_KEEPALIVE).expect("parse is failed");
//     println!("rem {:?}", rem);
//     println!("res {:?}", res);
// }
