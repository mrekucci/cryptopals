// Copyright (c) 2018, Peter Mrekaj. All rights reserved.
// Use of this source code is governed by a ISC-style
// license that can be found in the LICENSE.txt file.

//! Solutions for the [Set 1].
//!
//! [Set 1]: http://cryptopals.com/sets/1

extern crate base64;
extern crate hex;

use Result;

// The English ASCII character frequencies derived from: http://www.gutenberg.org/ebooks/2701.
static ASCII_CHARACTER_FREQUENCY_MAP: [f64; 128] = [
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.01771649014697238, // b'\n'
    0.0,
    0.0,
    0.01771649014697238, // b'\r'
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.15680041758654964,   // b' '
    0.0014025323324160285, // b'!'
    0.0,
    0.0000007932875183348578, // b'#'
    0.000003173150073339431,  // b'$'
    0.0000007932875183348578, // b'%'
    0.0000015865750366697155, // b'&'
    0.0,
    0.00018721585432702643, // b'('
    0.00018721585432702643, // b')'
    0.00008170861438849034, // b'*'
    0.0,
    0.015379465117957888,    // b','
    0.0021045917861423776,   // b'-'
    0.006495438200125815,    // b'.'
    0.000020625475476706303, // b'/'
    0.00014358504081860926,  // b'0'
    0.00021498091746874646,  // b'1'
    0.00008726162701683436,  // b'2'
    0.0000713958766501372,   // b'3'
    0.00005711670132010976,  // b'4'
    0.00007218916416847205,  // b'5'
    0.00005315026372843547,  // b'6'
    0.00006980930161346749,  // b'7'
    0.00007298245168680691,  // b'8'
    0.00005315026372843547,  // b'9'
    0.00017690311658867327,  // b':'
    0.00331832168919471,     // b';'
    0.0,
    0.0,
    0.0,
    0.0007988405309632018,    // b'?'
    0.0000015865750366697155, // b'@'
    0.0019030967564853238,    // b'A'
    0.001099496500412113,     // b'B'
    0.0009400457092268064,    // b'C'
    0.0005124637368443181,    // b'D'
    0.0006576353526995971,    // b'E'
    0.0006116246766361753,    // b'F'
    0.0004886651112942724,    // b'G'
    0.0010169945985052877,    // b'H'
    0.002566285121813265,     // b'I'
    0.00020387489221205846,   // b'J'
    0.00009836765227352236,   // b'K'
    0.0005481616751693867,    // b'L'
    0.000524363049619341,     // b'M'
    0.0006655682278829456,    // b'N'
    0.0004823188111475935,    // b'O'
    0.0009662241973318568,    // b'P'
    0.00025702515594049394,   // b'Q'
    0.0005545079753160656,    // b'R'
    0.0015310449103862755,    // b'S'
    0.0018840578560452871,    // b'T'
    0.00012137299030523323,   // b'U'
    0.00010550723993853609,   // b'V'
    0.0010019221356569253,    // b'W'
    0.000012692600293357724,  // b'X'
    0.00020704804228539788,   // b'Y'
    0.000015072462848362297,  // b'Z'
    0.0000023798625550045732, // b'['
    0.0,
    0.0000023798625550045732, // b']'
    0.0,
    0.0006155911142278496, // b'_'
    0.0,
    0.0609522464712588,    // b'a'
    0.012553774977649125,  // b'b'
    0.017557832643305408,  // b'c'
    0.03030913621301991,   // b'd'
    0.09400536421019898,   // b'e'
    0.0162536679631629,    // b'f'
    0.016396459716463176,  // b'g'
    0.049566190720598584,  // b'h'
    0.05034678563864008,   // b'i'
    0.0007290312293497343, // b'j'
    0.0064248356109940135, // b'k'
    0.033855131419976724,  // b'l'
    0.01827337798484345,   // b'm'
    0.05230937895900052,   // b'n'
    0.05567450461177699,   // b'o'
    0.01322251635560541,   // b'p'
    0.0009971624105469161, // b'q'
    0.04195380369465729,   // b'r'
    0.050147670471538036,  // b's'
    0.06942852360466675,   // b't'
    0.021458427370957902,  // b'u'
    0.006819892795124773,  // b'v'
    0.016878778527610768,  // b'w'
    0.0008313653192149309, // b'x'
    0.013461295898624201,  // b'y'
    0.000491044973849277,  // b'z'
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
];

/// Returns english score for `data` base on character frequency metric.
fn score_english<T: AsRef<[u8]>>(data: T) -> f64 {
    data.as_ref()
        .iter()
        .map(|x| ASCII_CHARACTER_FREQUENCY_MAP[*x as usize])
        .sum()
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
pub fn fixed_xor<T: AsRef<[u8]>>(a: T, b: T) -> Result<String> {
    let a = hex::decode(a)?;
    let b = hex::decode(b)?;

    let xor: Vec<_> = a.iter().zip(b).map(|(x, y)| x ^ y).collect();

    Ok(hex::encode(xor))
}

/// Finds the key, a single character which XOR'd every byte of the `data`, and return decrypted
/// `data`.
///
/// This function is designed to help resolve the [Challenge 3].
///
/// [Challenge 3]: http://cryptopals.com/sets/1/challenges/3
pub fn single_byte_xor_cipher<T: AsRef<[u8]>>(data: T) -> Result<String> {
    let data = hex::decode(data)?;

    let mut max = 0.0;
    let mut msg = Vec::new();
    for key in 0..128 {
        let secret: Vec<_> = data.iter().map(|x| x ^ key).collect();
        let score = score_english(&secret);
        if max < score {
            max = score;
            msg = secret;
        }
    }

    Ok(String::from_utf8(msg)?)
}

/// Finds and returns the line inside the `data` that has been encrypted by single-character XOR.
///
/// This function is designed to help resolve the [Challenge 4].
///
/// [Challenge 4]: http://cryptopals.com/sets/1/challenges/4
pub fn detect_single_character_xor<T: AsRef<[u8]>>(data: T) -> Result<String> {
    let mut max = 0.0;
    let mut msg = String::new();

    for line in data.as_ref().split(|ch| *ch == b'\n') {
        if hex::decode(line)?.is_ascii() {
            let secret = single_byte_xor_cipher(line)?; // TODO: consider refactoring to also return the score from single_byte_xor_cipher.
            let score = score_english(&secret);
            if max < score {
                max = score;
                msg = secret;
            }
        }
    }

    Ok(msg)
}

/// Encrypts the `data` with the given `key`using XOR and returns the result.
///
/// This function is designed to help resolve the [Challenge 5].
///
/// [Challenge 5]: http://cryptopals.com/sets/1/challenges/5
pub fn encrypt_by_repeating_xor<T: AsRef<[u8]>>(key: T, data: T) -> Result<String> {
    let secret: Vec<_> = data
        .as_ref()
        .iter()
        .zip(key.as_ref().iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect();

    Ok(hex::encode(secret))
}

#[cfg(test)]
mod test {
    use super::{
        detect_single_character_xor, encrypt_by_repeating_xor, fixed_xor, hex_to_base64,
        single_byte_xor_cipher,
    };

    #[test]
    fn test_challenge1_solution() {
        assert_eq!(
            hex_to_base64(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
            ).unwrap(),
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"),
        );
    }

    #[test]
    fn test_challenge2_solution() {
        assert_eq!(
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965",
            ).unwrap(),
            String::from("746865206b696420646f6e277420706c6179"),
        )
    }

    #[test]
    fn test_challenge3_solution() {
        assert_eq!(
            single_byte_xor_cipher(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            ).unwrap(),
            String::from("Cooking MC's like a pound of bacon"),
        )
    }

    #[test]
    fn test_challenge4_solution() {
        assert_eq!(
            detect_single_character_xor(include_str!("data/set1_challenge4.txt")).unwrap(),
            String::from("Now that the party is jumping\n"),
        )
    }

    #[test]
    fn test_challenge5_solution() {
        assert_eq!(
            encrypt_by_repeating_xor(
                "ICE",
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
            ).unwrap(),
            String::from(
                "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
                 a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            ),
        )
    }
}
