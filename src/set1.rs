// Copyright (c) 2018, Peter Mrekaj. All rights reserved.
// Use of this source code is governed by a ISC-style
// license that can be found in the LICENSE.txt file.

//! Solutions for the [Set 1].
//!
//! [Set 1]: http://cryptopals.com/sets/1

extern crate base64;
extern crate hex;

use std::collections::HashMap;
use Result;

lazy_static! {
    // The English ASCII character frequencies derived from: http://www.gutenberg.org/ebooks/2701.
    static ref CHARACTER_FREQUENCIES: HashMap<u8, f64> = [
        (b'\n', 0.01771649014697238),
        (b'\r', 0.01771649014697238),
        (b' ', 0.15680041758654964),
        (b'!', 0.0014025323324160285),
        (b'#', 0.0000007932875183348578),
        (b'$', 0.000003173150073339431),
        (b'%', 0.0000007932875183348578),
        (b'&', 0.0000015865750366697155),
        (b'(', 0.00018721585432702643),
        (b')', 0.00018721585432702643),
        (b'*', 0.00008170861438849034),
        (b',', 0.015379465117957888),
        (b'-', 0.0021045917861423776),
        (b'.', 0.006495438200125815),
        (b'/', 0.000020625475476706303),
        (b'0', 0.00014358504081860926),
        (b'1', 0.00021498091746874646),
        (b'2', 0.00008726162701683436),
        (b'3', 0.0000713958766501372),
        (b'4', 0.00005711670132010976),
        (b'5', 0.00007218916416847205),
        (b'6', 0.00005315026372843547),
        (b'7', 0.00006980930161346749),
        (b'8', 0.00007298245168680691),
        (b'9', 0.00005315026372843547),
        (b':', 0.00017690311658867327),
        (b';', 0.00331832168919471),
        (b'?', 0.0007988405309632018),
        (b'@', 0.0000015865750366697155),
        (b'A', 0.0019030967564853238),
        (b'B', 0.001099496500412113),
        (b'C', 0.0009400457092268064),
        (b'D', 0.0005124637368443181),
        (b'E', 0.0006576353526995971),
        (b'F', 0.0006116246766361753),
        (b'G', 0.0004886651112942724),
        (b'H', 0.0010169945985052877),
        (b'I', 0.002566285121813265),
        (b'J', 0.00020387489221205846),
        (b'K', 0.00009836765227352236),
        (b'L', 0.0005481616751693867),
        (b'M', 0.000524363049619341),
        (b'N', 0.0006655682278829456),
        (b'O', 0.0004823188111475935),
        (b'P', 0.0009662241973318568),
        (b'Q', 0.00025702515594049394),
        (b'R', 0.0005545079753160656),
        (b'S', 0.0015310449103862755),
        (b'T', 0.0018840578560452871),
        (b'U', 0.00012137299030523323),
        (b'V', 0.00010550723993853609),
        (b'W', 0.0010019221356569253),
        (b'X', 0.000012692600293357724),
        (b'Y', 0.00020704804228539788),
        (b'Z', 0.000015072462848362297),
        (b'[', 0.0000023798625550045732),
        (b']', 0.0000023798625550045732),
        (b'_', 0.0006155911142278496),
        (b'a', 0.0609522464712588),
        (b'b', 0.012553774977649125),
        (b'c', 0.017557832643305408),
        (b'd', 0.03030913621301991),
        (b'e', 0.09400536421019898),
        (b'f', 0.0162536679631629),
        (b'g', 0.016396459716463176),
        (b'h', 0.049566190720598584),
        (b'i', 0.05034678563864008),
        (b'j', 0.0007290312293497343),
        (b'k', 0.0064248356109940135),
        (b'l', 0.033855131419976724),
        (b'm', 0.01827337798484345),
        (b'n', 0.05230937895900052),
        (b'o', 0.05567450461177699),
        (b'p', 0.01322251635560541),
        (b'q', 0.0009971624105469161),
        (b'r', 0.04195380369465729),
        (b's', 0.050147670471538036),
        (b't', 0.06942852360466675),
        (b'u', 0.021458427370957902),
        (b'v', 0.006819892795124773),
        (b'w', 0.016878778527610768),
        (b'x', 0.0008313653192149309),
        (b'y', 0.013461295898624201),
        (b'z', 0.000491044973849277),
    ].iter().cloned().collect();
}

/// Converts hexadecimal data to base64.
///
/// This function is designed to help resolve the [Challenge 1].
///
/// [Challenge 1]: http://cryptopals.com/sets/1/challenges/1
pub fn hex_to_base64<T: AsRef<[u8]>>(data: T) -> Result<String> {
    Ok(base64::encode(&hex::decode(data)?))
}

/// Produces XOR product of equal-length `a` and `b` buffers.
///
/// This function is designed to help resolve the [Challenge 2].
///
/// [Challenge 2]: http://cryptopals.com/sets/1/challenges/2
pub fn fixed_xor<T: AsRef<[u8]>>(a: T, b: T) -> Result<Vec<u8>> {
    let a = hex::decode(a)?;
    let b = hex::decode(b)?;

    // TODO: check if the buffers are equal-length and throw error if not.

    Ok(a.iter().zip(b).map(|(x, y)| x ^ y).collect())
}

/// Finds the key, a single character which XOR'd every byte of the `data`, and return decrypted `data`.
///
/// This function is designed to help resolve the [Challenge 3].
///
/// [Challenge 3]: http://cryptopals.com/sets/1/challenges/3
pub fn single_byte_xor_cipher<T: AsRef<[u8]>>(data: T) -> Result<String> { // TODO: consider passing CHARACTER_FREQUENCIES as argument and return the single character.
    let data = hex::decode(data)?;

    let mut key = 0;
    let mut max = 0.0;
    for ch in 0..127 {
        let score = data
            .iter()
            .map(|x| CHARACTER_FREQUENCIES.get(&(x ^ ch)).unwrap_or(&0.0))
            .sum();

        if max < score {
            key = ch;
            max = score;
        }
    }

    Ok(String::from_utf8(data.iter().map(|x| x ^ key).collect())?)
}

#[cfg(test)]
mod test {
    use super::{fixed_xor, hex::FromHex, hex_to_base64, single_byte_xor_cipher};

    #[test]
    fn test_challenge1_solution() {
        assert_eq!(
            hex_to_base64(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
            ).unwrap(),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string(),
        );
    }

    #[test]
    fn test_challenge2_solution() {
        assert_eq!(
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965",
            ).unwrap(),
            Vec::from_hex("746865206b696420646f6e277420706c6179").unwrap(),
        )
    }

    #[test]
    fn test_challenge3_solution() {
        assert_eq!(
            single_byte_xor_cipher(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            ).unwrap(),
            "Cooking MC's like a pound of bacon".to_string(),
        )
    }
}
