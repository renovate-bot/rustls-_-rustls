use core::fmt;

/// Alternative implementation of `fmt::Debug` for byte slice.
///
/// Standard `Debug` implementation for `[u8]` is comma separated
/// list of numbers. Since large amount of byte strings are in fact
/// ASCII strings or contain a lot of ASCII strings (e. g. HTTP),
/// it is convenient to print strings as ASCII when possible.
///
/// This struct wraps `&[u8]` just to override `fmt::Debug`.
///
/// `BsDebug` is not a part of public API of bytes crate.
pub(crate) struct BsDebug<'a>(pub(crate) &'a [u8]);

impl fmt::Debug for BsDebug<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(fmt, "b\"")?;
        for &c in self.0 {
            // https://doc.rust-lang.org/reference.html#byte-escapes
            if c == b'\n' {
                write!(fmt, "\\n")?;
            } else if c == b'\r' {
                write!(fmt, "\\r")?;
            } else if c == b'\t' {
                write!(fmt, "\\t")?;
            } else if c == b'\\' || c == b'"' {
                write!(fmt, "\\{}", c as char)?;
            } else if c == b'\0' {
                write!(fmt, "\\0")?;
                // ASCII printable
            } else if (0x20..0x7f).contains(&c) {
                write!(fmt, "{}", c as char)?;
            } else {
                write!(fmt, "\\x{c:02x}")?;
            }
        }
        write!(fmt, "\"")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::format;
    use std::prelude::v1::*;

    use super::BsDebug;

    #[test]
    fn debug() {
        let vec: Vec<_> = (0..0x100).map(|b| b as u8).collect();

        let expected = "b\"\
            \\0\\x01\\x02\\x03\\x04\\x05\\x06\\x07\
            \\x08\\t\\n\\x0b\\x0c\\r\\x0e\\x0f\
            \\x10\\x11\\x12\\x13\\x14\\x15\\x16\\x17\
            \\x18\\x19\\x1a\\x1b\\x1c\\x1d\\x1e\\x1f\
            \x20!\\\"#$%&'()*+,-./0123456789:;<=>?\
            @ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\\\]^_\
            `abcdefghijklmnopqrstuvwxyz{|}~\\x7f\
            \\x80\\x81\\x82\\x83\\x84\\x85\\x86\\x87\
            \\x88\\x89\\x8a\\x8b\\x8c\\x8d\\x8e\\x8f\
            \\x90\\x91\\x92\\x93\\x94\\x95\\x96\\x97\
            \\x98\\x99\\x9a\\x9b\\x9c\\x9d\\x9e\\x9f\
            \\xa0\\xa1\\xa2\\xa3\\xa4\\xa5\\xa6\\xa7\
            \\xa8\\xa9\\xaa\\xab\\xac\\xad\\xae\\xaf\
            \\xb0\\xb1\\xb2\\xb3\\xb4\\xb5\\xb6\\xb7\
            \\xb8\\xb9\\xba\\xbb\\xbc\\xbd\\xbe\\xbf\
            \\xc0\\xc1\\xc2\\xc3\\xc4\\xc5\\xc6\\xc7\
            \\xc8\\xc9\\xca\\xcb\\xcc\\xcd\\xce\\xcf\
            \\xd0\\xd1\\xd2\\xd3\\xd4\\xd5\\xd6\\xd7\
            \\xd8\\xd9\\xda\\xdb\\xdc\\xdd\\xde\\xdf\
            \\xe0\\xe1\\xe2\\xe3\\xe4\\xe5\\xe6\\xe7\
            \\xe8\\xe9\\xea\\xeb\\xec\\xed\\xee\\xef\
            \\xf0\\xf1\\xf2\\xf3\\xf4\\xf5\\xf6\\xf7\
            \\xf8\\xf9\\xfa\\xfb\\xfc\\xfd\\xfe\\xff\"";

        assert_eq!(expected, format!("{:?}", BsDebug(&vec)));
    }
}
