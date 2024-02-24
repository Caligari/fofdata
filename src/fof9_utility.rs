use std::fmt::{Display, Debug};
use binrw::{BinRead, BinWrite};
use encoding::{all::ISO_8859_1, Encoding};
use num_integer::div_rem;
use num_traits::FromPrimitive;
use log::error;


#[derive(BinRead, BinWrite, Clone, PartialEq)]
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


// TODO: write?
#[derive(BinRead, Clone, Copy, Debug)]
pub struct LengthInches {
    inches_eighths: u32,
}

impl Display for LengthInches {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (inches, eighths) = div_rem(self.inches_eighths, 10);
        if eighths > 7 { error!("{} eighths of an inch in length {}", eighths, self.inches_eighths); }
        let (feet, inches) = div_rem(inches, 12);

        if feet > 0 {
            write!(f, "{}'{}",
                feet,
                fmt_inches(inches, eighths),
            )
        } else {
            write!(f, "{}",
                fmt_inches(inches, eighths),
            )
        }
    }
}

fn fmt_inches ( inches: u32, eighths: u32 ) -> String {
    if inches > 0 {
        format!{" {}{}\"",
            inches,
            fmt_eighths(eighths),
        }
    } else if eighths > 0 {
        format!{"{}\"",
            fmt_eighths(eighths),
        }
    } else {
        String::new()
    }
}

fn fmt_eighths ( eighths: u32 ) -> String {
    if eighths > 0 {
        format!{" {}/8", eighths}
    } else {
        String::new()
    }
}



#[derive(BinRead, Clone, Copy, Debug)]
pub struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}",
            self.month,
            self.day,
            self.year,
        )
    }
}
