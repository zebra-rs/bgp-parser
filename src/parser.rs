use nom::bytes::complete::take;
use nom::error::{make_error, ErrorKind};

pub use nom::IResult;

fn parse_marker(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rem, res) = take(16usize)(input)?;
    for i in res.into_iter() {
        if *i != 0xffu8 {
            return Err(nom::Err::Error(make_error(input, ErrorKind::Tag)));
        }
    }
    Ok((rem, res))
}

pub fn parse_bgp_packet(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rem, res) = parse_marker(input)?;
    Ok((rem, res))
}
