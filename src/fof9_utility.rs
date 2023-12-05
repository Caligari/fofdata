use std::fmt::{Display, Debug};
use binrw::{BinRead, BinWrite};
use encoding::{all::ISO_8859_1, Encoding};
use num_traits::FromPrimitive;
use log::error;


#[derive(BinRead, BinWrite, PartialEq)]
pub struct FixedString {
    #[bw(map = |_| u32::from_usize(string.len()).unwrap())]
    len: u32,
    #[bw(map = |s| s.as_bytes().to_vec())]
    #[br(count = len, map = |s: Vec<u8>| match ISO_8859_1.decode(&s, encoding::DecoderTrap::Strict) {
        Ok(out) => out,
        Err(_)  => { error!("unable to convert text {:?}", s); "<bad conversion>".to_string() }
    })]
    pub string: String,
}

impl Display for FixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl Debug for FixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
