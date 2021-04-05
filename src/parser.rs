use crate::*;
use nom::bytes::complete::take;
use nom::combinator::{map, peek};
use nom::error::{make_error, ErrorKind};
use nom::IResult;

fn parse_marker(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rem, res) = take(16usize)(input)?;
    for i in res.into_iter() {
        if *i != 0xffu8 {
            return Err(nom::Err::Error(make_error(input, ErrorKind::Tag)));
        }
    }
    Ok((rem, res))
}

pub fn parse_bgp_packet(input: &[u8]) -> IResult<&[u8], BgpPacket> {
    let (rem, _) = parse_marker(input)?;
    let (_, ltype) = peek(take(3u8))(rem)?;

    match BgpPacketType(ltype[2] as u8) {
        BgpPacketType::Open => map(BgpOpenPacket::parse, BgpPacket::Open)(rem),
        BgpPacketType::Update => map(BgpUpdatePacket::parse, BgpPacket::Update)(rem),
        BgpPacketType::Notification => {
            map(BgpNotificationPacket::parse, BgpPacket::Notification)(rem)
        }
        BgpPacketType::KeepAlive => map(BgpKeepAlivePacket::parse, BgpPacket::KeepAlive)(rem),
        _ => Err(nom::Err::Error(make_error(rem, ErrorKind::Tag))),
    }
}
