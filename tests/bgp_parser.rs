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
    println!("rem {:?}", rem);
    println!("res {:?}", res);
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
