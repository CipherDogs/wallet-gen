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

use base58::ToBase58;
use ed25519_dalek::{Keypair, SecretKey, PublicKey};
use openssl::bn::{BigNum, BigNumRef, BigNumContext, BigNumContextRef};
use openssl::rand::rand_bytes;
use std::{iter, slice};
use super::prelude::*;
use tiny_keccak::keccak256;
use utils::{HexSlice, Sha512};

fn get_prefix(coin: Coin) -> Option<u8> {
    match coin {
        Coin::Testnet => Some(0x35),
        Coin::Monero => Some(0x12),
        Coin::Aeon => Some(0xb2),
        _ => None,
    }
}

lazy_static! {
    /* ed25519 constant: (2 ** 252) + 27742317777372353535851937790883648493 */
    static ref L: BigNum = BigNum::from_dec_str("7237005577332262213973186563042994240857116359379907606001950938285454250989").unwrap();
}

/// Perform the `sc_reduce32` procedure on the given bytestring, producing
/// a 256-bit scalar usable as an Ed25519 private key.
pub fn sc_reduce32(bytes: &mut [u8; 32], ctx: &mut BigNumContextRef) -> Result<BigNum> {
    // Fix byte ordering
    #[cfg(target_endian = "little")]
    bytes.reverse();

    // Perform modulo
    let number = BigNum::from_slice(&bytes[..])?;
    let mut reduced = BigNum::new()?;
    reduced.checked_rem(&number, &*L, ctx)?;
    Ok(reduced)
}

/// Converts an OpenSsl [`BigNumRef`] into a [`Vec<u8>`] in big-endian form,
/// padding it with zero bytes until it is at least 32 bytes long.
///
/// [`BigNumRef`]: https://docs.rs/openssl/0.10.2/openssl/bn/struct.BigNumRef.html
/// [`Vec<u8>`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
pub fn bn_to_vec32(number: &BigNumRef) -> Vec<u8> {
    // Adds leading zeros
    let mut result = number.to_vec();
    let missing = 32 - result.len();
    result.splice(..0, iter::repeat(0).take(missing));

    // Fix byte ordering
    #[cfg(target_endian = "little")]
    result.reverse();

    result
}

/// Creates an ed25519 keypair from the given seed. A [`sc_reduce32`] is run on the bytestring
/// to ensure that it can be properly transformed into a seed.
///
/// [`sc_reduce32`]: ./fn.sc_reduce32.html
pub fn keypair_from_bytes(bytes: &mut [u8; 32], ctx: &mut BigNumContextRef) -> Result<Keypair> {
    let num = sc_reduce32(bytes, ctx)?;
    let vec = bn_to_vec32(&num);
    let priv_key = SecretKey::from_bytes(vec.as_slice())?;
    let pub_key = PublicKey::from_secret::<Sha512>(&priv_key);

    Ok(Keypair {
        secret: priv_key,
        public: pub_key,
    })
}

/// Generate the Cryptonote wallet address from the two public keys
/// and the type of coin.
pub fn generate_address(coin: Coin, spend_key: &PublicKey, view_key: &PublicKey) -> Result<String> {
    let mut bytes = [0; 69];

    // Write prefix
    match get_prefix(coin) {
        Some(prefix) => bytes[0] = prefix,
        None => return Err(Error::CoinNotSupported(coin)),
    };

    {
        // Add spending public key
        let src = spend_key.as_bytes().as_ref();
        let dest = &mut bytes[1..33];
        dest.copy_from_slice(src);
    }

    {
        // Add viewing public key
        let src = view_key.as_bytes().as_ref();
        let dest = &mut bytes[33..65];
        dest.copy_from_slice(src);
    }

    {
        // Add checksum
        let src = &keccak256(&bytes[..])[..4];
        let dest = &mut bytes[65..69];
        dest.copy_from_slice(src);
    }

    let slice = &bytes[..];
    Ok(slice.to_base58())
}

/// Creates a new Cryptonote wallet using the Electrum/Deterministic style,
/// for use with coins like Monero or Aeon.
///
/// The "public" and "private" keys are only that of the spend keypair. To
/// determine the view keys, you must perform the [`keccak256`] hash on the
/// private key and use the [`sc_reduce32`] transformation to make the result
/// usable as an ed25519 private key.
///
/// [`keccak256`]: https://docs.rs/tiny-keccak/1.4.0/tiny_keccak/fn.keccak256.html
/// [`sc_reduce32`]: ./fn.sc_reduce32.html
pub fn new_wallet(coin: Coin) -> Result<Wallet> {
    let mut ctx = BigNumContext::new()?;

    let spend_keypair = {
        let mut buffer = [0; 32];
        rand_bytes(&mut buffer[..]);
        keypair_from_bytes(&mut buffer, &mut ctx)?
    };

    let view_keypair = {
        let mut buffer = keccak256(spend_keypair.secret.as_bytes().as_ref());
        keypair_from_bytes(&buffer, &mut ctx)?
    };

    let addr = generate_address(coin, &spend_keypair.public, &view_keypair.public)?;

    Ok(Wallet {
        coin: coin,
        address: addr,
        public_key: HexSlice::new(spend_keypair.public.as_bytes().as_ref()).format(),
        private_key: HexSlice::new(spend_keypair.secret.as_bytes().as_ref()).format(),
    })
}

#[test]
fn gen_xmr_wallet() {
    println!("XMR {:?}", &new_wallet(Coin::Monero).unwrap());
}
