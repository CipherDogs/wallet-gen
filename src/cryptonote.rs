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
use ed25519::{PublicKey, keypair_from_bytes};
use openssl::bn::BigNumContext;
use openssl::rand::rand_bytes;
use super::prelude::*;
use tiny_keccak::keccak256;
use utils::HexSlice;

fn get_prefix(coin: Coin) -> Option<u8> {
    match coin {
        Coin::Testnet => Some(0x35),
        Coin::Monero => Some(0x12),
        Coin::Aeon => Some(0xb2),
        _ => None,
    }
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
        rand_bytes(&mut buffer[..])?;
        keypair_from_bytes(buffer, &mut ctx)?
    };

    let view_keypair = {
        let mut buffer = keccak256(spend_keypair.private.as_ref());
        keypair_from_bytes(buffer, &mut ctx)?
    };

    let addr = generate_address(coin, &spend_keypair.public, &view_keypair.public)?;

    Ok(Wallet {
        coin: coin,
        address: addr,
        public_key: HexSlice::new(spend_keypair.public.as_ref()).format(),
        private_key: HexSlice::new(spend_keypair.private.as_ref()).format(),
    })
}

#[test]
fn gen_xmr_wallet() {
    println!("XMR {:?}", &new_wallet(Coin::Monero).unwrap());
}
