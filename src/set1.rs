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

/// Produces XOR product of equal-length `a` and `b` buffers.
///
/// This function is designed to help resolve the [Challenge 2].
///
/// [Challenge 2]: http://cryptopals.com/sets/1/challenges/2
pub fn fixed_xor<T: AsRef<[u8]>>(a: T, b: T) -> Result<Vec<u8>, hex::FromHexError> {
    let a = hex::decode(a)?;
    let b = hex::decode(b)?;

    // TODO: check if the buffers are equal-length and throw error if not.

    Ok(a.iter()
        .zip(b)
        .map(|(x, y)| x ^ y)
        .collect())
}

#[cfg(test)]
mod test {
    use super::{fixed_xor, hex::FromHex, hex_to_base64};

    #[test]
    fn test_challenge1_solution() {
        assert_eq!(
            hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            Ok("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()),
        );
    }

    #[test]
    fn test_challenge2_solution() {
        assert_eq!(
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            Vec::from_hex("746865206b696420646f6e277420706c6179"),
        )
    }
}
