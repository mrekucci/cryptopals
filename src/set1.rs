// Copyright (c) 2018, Peter Mrekaj. All rights reserved.
// Use of this source code is governed by a ISC-style
// license that can be found in the LICENSE.txt file.

//! Solutions for the [Set 1].
//!
//! [Set 1]: http://cryptopals.com/sets/1

extern crate base64;
extern crate hex;

/// Converts hexadecimal data to base64.
///
/// This function is designed to help resolve the [Challenge 1].
///
/// [Challenge 1]: http://cryptopals.com/sets/1/challenges/1
pub fn hex_to_base64<T: AsRef<[u8]>>(data: T) -> Result<String, hex::FromHexError> {
    Ok(base64::encode(&hex::decode(data)?))
}

#[cfg(test)]
mod test {
    use super::{hex_to_base64};

    #[test]
    fn test_challenge1_solution() {
        assert_eq!(
            hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            Ok("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()),
        );
    }
}
