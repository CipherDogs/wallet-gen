/*
 * cryptonote.rs
 *
 * Copyright 2018 Standard Mining
 *
 * Available to be used and modified under the terms of the MIT License.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
 * WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! Various functions related to Cryptonote (e.g. Monero) wallet generation and validation.

use super::prelude::*;
use arrayvec::ArrayVec;
use base58::ToBase58;
use ed25519::{keypair_from_bytes, PublicKey};
use hex_slice::HexSlice;
use openssl::bn::BigNumContext;
use openssl::rand::rand_bytes;
use std::collections::HashMap;
use std::io::Write;
use tiny_keccak::keccak256;

fn get_prefix(coin: Coin) -> Option<&'static [u8]> {
    match coin {
        Coin::Testnet => Some(&[0x35]),
        Coin::Monero => Some(&[0x12]),
        Coin::Aeon => Some(&[0xb2, 0x01]),
        _ => None,
    }
}

/// Generate the Cryptonote wallet address from the two public keys
/// and the type of coin.
pub fn generate_address(coin: Coin, spend_key: &PublicKey, view_key: &PublicKey) -> Result<String> {
    let mut bytes = ArrayVec::<[u8; 72]>::new();

    // Add coin prefix
    match get_prefix(coin) {
        Some(prefix) => bytes.write_all(prefix).unwrap(),
        None => return Err(Error::CoinNotSupported(coin)),
    };

    // Add public keys
    bytes.write_all(spend_key.as_ref()).unwrap();
    bytes.write_all(view_key.as_ref()).unwrap();

    // Add checksum
    let hash = keccak256(bytes.as_slice());
    bytes.write_all(&hash[..4]).unwrap();

    // Convert to base58 in 8 byte chunks
    let mut base58 = String::new();
    for chunk in bytes.as_slice().chunks(8) {
        let mut part = chunk.to_base58();
        let exp_len = match chunk.len() {
            8 => 11,
            6 => 9,
            5 => 7,
            _ => panic!("Invalid chunk length: {}", chunk.len()),
        };
        let missing = exp_len - part.len();
        if missing > 0 {
            part.insert_str(0, &"11111111111"[..missing]);
        }
        base58.push_str(&part);
    }

    Ok(base58)
}

/// Generates a new random Cryptonote wallet for coins like Monero or Aeon.
/// Uses [`from_seed`] to create the wallet.
///
/// The returned wallet will store the view keys in the `other` map.
/// The fields are `view_public_key` and `view_private_key`, and have
/// the same format as the normal (spend) public and private key fields.
pub fn new_wallet(coin: Coin) -> Result<Wallet> {
    let mut seed = [0; 32];
    rand_bytes(&mut seed[..])?;
    from_seed(coin, seed)
}

/// Creates a new Cryptonote wallet using the Electrum/Deterministic style,
/// from the given random seed.
///
/// The "public" and "private" keys are only that of the spend keypair. To
/// determine the view keys, you must perform the [`keccak256`] hash on the
/// private key and use the `sc_reduce32` transformation to make the result
/// usable as an ed25519 private key.
///
/// [`keccak256`]: https://docs.rs/tiny-keccak/1.4.0/tiny_keccak/fn.keccak256.html
pub fn from_seed(coin: Coin, seed: [u8; 32]) -> Result<Wallet> {
    let mut ctx = BigNumContext::new()?;

    let spend_keypair = keypair_from_bytes(seed, &mut ctx)?;
    let view_keypair = {
        let mut buffer = keccak256(spend_keypair.private.as_ref());
        keypair_from_bytes(buffer, &mut ctx)?
    };

    let addr = generate_address(coin, &spend_keypair.public, &view_keypair.public)?;

    Ok(Wallet {
        coin:        coin,
        address:     addr,
        public_key:  HexSlice::new(spend_keypair.public.as_ref()).format(),
        private_key: HexSlice::new(spend_keypair.private.as_ref()).format(),
        other:       {
            let mut map = HashMap::new();
            map.insert(
                "view_public_key".into(),
                HexSlice::new(view_keypair.public.as_ref()).format(),
            );
            map.insert(
                "view_private_key".into(),
                HexSlice::new(view_keypair.private.as_ref()).format(),
            );
            Some(map)
        },
    })
}

#[test]
fn gen_xmr_wallet() {
    println!("XMR {:?}", &new_wallet(Coin::Monero).unwrap());
}

#[test]
fn gen_aeon_wallet() {
    println!("AEON {:?}", &new_wallet(Coin::Aeon).unwrap());
}

#[test]
fn test_xmr_wallet() {
    let seed = [
        0xbd, 0xb2, 0x5d, 0x9d, 0x7b, 0xdb, 0xda, 0x38, 0x97, 0xf6, 0xc9, 0x42, 0x7a, 0xd6, 0x57,
        0xd1, 0x56, 0x75, 0xa9, 0x4a, 0x06, 0xf0, 0xdb, 0x66, 0xb9, 0xb0, 0x53, 0xb0, 0xb2, 0x78,
        0xa8, 0x00,
    ];

    let wallet = from_seed(Coin::Monero, seed).unwrap();
    assert_eq!(wallet.coin, Coin::Monero);
    assert_eq!(
        &wallet.address,
        "4B5hMDhQyxb3aCpo7aQjrN8WCVfW3fRZwYGwDiMYBuHdPSfxSxxk5PG7arpdLpLi91N8ozt129c4w2vxhfQURRP8JQHmbvi",
    );
    assert_eq!(
        &wallet.public_key,
        "f99782b370c9100f613e341bf2577f2cdc28d6a3795e86bafabcc0cd3fb05486",
    );
    assert_eq!(
        &wallet.private_key,
        "bdb25d9d7bdbda3897f6c9427ad657d15675a94a06f0db66b9b053b0b278a800",
    );
    let ref other = wallet.other.unwrap();
    assert_eq!(
        &other["view_public_key"],
        "2cf5e093e437e3275ca066fdf27fdc7e5b1ae7c6fe98600b8a72dc213294b39a",
    );
    assert_eq!(
        &other["view_private_key"],
        "afc9d176516ecbe8cc6b4bc0efea956b1f0c3ffede0ddd919a8486b143a09d0d",
    );
}
