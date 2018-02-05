/*
 * bitcoin.rs
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

//! Various functions related to Bitcoin wallet generation and validation.

use base58::ToBase58;
use openssl::bn::BigNumContext;
use openssl::ec::{EcGroup, EcKey, PointConversionForm};
use openssl::hash::{hash, MessageDigest};
use openssl::nid::Nid;
use openssl::sha::sha256;
use std::io::Write;
use super::prelude::*;
use utils::HexSlice;

/// Generate a new Bitcoin (or variant) wallet.
/// The `coin` parameter is not checked, but is merely passed
/// through to the resultant [`Wallet`].
///
/// [`Wallet`] ../wallet/struct.Wallet.html
pub fn new_wallet(coin: Coin) -> Result<Wallet> {
    let group = EcGroup::from_curve_name(Nid::SECP256K1)?;
    let mut bn_ctx = BigNumContext::new()?;
    let key = EcKey::generate(&group)?;
    let pub_key = key.public_key()
                     .to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut bn_ctx)?;
    let mut priv_key = key.private_key().to_vec();

    let pub_hash = sha256(&pub_key);
    let ripe_hash = hash(MessageDigest::ripemd160(), &pub_hash[..])?;
    let mut address = ripe_hash.to_vec();

    Ok(Wallet {
        coin: coin,
        address: base58_check(&mut address, 0x00),
        public_key: format!("{:x}", &HexSlice::new(&pub_key)),
        private_key: base58_check(&mut priv_key, 0x80),
    })
}

/// Performs a "base58 check", a modified base 58 byte conversion
/// that appends a double SHA256 checksum.
pub fn base58_check(bytes: &mut Vec<u8>, version: u8) -> String {
    bytes.insert(0, version);
    let hash = sha256(bytes);
    let checksum = &sha256(&hash[..])[..4];
    bytes.write_all(checksum).unwrap();
    bytes.as_slice().to_base58()
}

#[test]
fn gen_btc_wallet() {
    println!("{:?}", &new_wallet(Coin::Bitcoin).unwrap());
}
