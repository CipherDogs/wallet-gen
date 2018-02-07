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

use super::prelude::*;
use base58::ToBase58;
use openssl::bn::BigNumContext;
use openssl::ec::{EcGroup, EcKey, PointConversionForm};
use openssl::hash::{hash, MessageDigest};
use openssl::nid::Nid;
use openssl::sha::sha256;
use std::io::Write;
use utils::HexSlice;

/// Generate a new wallet based on the Bitcoin style of producing [`Wallet`]s.
/// For Bitcoin and Bitcoin variants.
///
/// [`Wallet`]: ../wallet/struct.Wallet.html
pub fn new_wallet(coin: Coin) -> Result<Wallet> {
    let bitcoin_data = match coin.bitcoin_wif_data() {
        Some(data) => data,
        None => return Err(Error::CoinNotSupported(coin)),
    };

    let group = EcGroup::from_curve_name(Nid::SECP256K1)?;
    let mut bn_ctx = BigNumContext::new()?;
    let key = EcKey::generate(&group)?;
    let pub_key =
        key.public_key()
            .to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut bn_ctx)?;
    let mut priv_key = key.private_key().to_vec();

    let pub_hash = sha256(&pub_key);
    let ripe_hash = hash(MessageDigest::ripemd160(), &pub_hash[..])?;
    let mut address = ripe_hash.to_vec();

    Ok(Wallet {
        coin:        coin,
        address:     base58_check(&mut address, bitcoin_data.network_version()),
        public_key:  HexSlice::new(&pub_key).format(),
        private_key: base58_check(&mut priv_key, &[bitcoin_data.private_key_prefix()]),
    })
}

/// Performs a "base58 check", a modified base 58 byte conversion
/// that appends a double SHA256 checksum.
///
/// This function modifies the buffer, prepending the
/// "application/version" byte.
pub fn base58_check(bytes: &mut Vec<u8>, prefix: &[u8]) -> String {
    bytes.splice(..0, prefix.iter().map(|s| *s));
    let hash = sha256(bytes);
    let checksum = &sha256(&hash[..])[..4];
    bytes.write_all(checksum).unwrap();
    bytes.as_slice().to_base58()
}

#[test]
fn gen_wallets() {
    use coin::COINS;

    for coin in COINS.iter() {
        if coin.bitcoin_wif_data().is_some() {
            let wallet = new_wallet(*coin).unwrap();
            println!("Coin: {:?} ({})", coin, coin.symbol());
            println!("Address: {}", &wallet.address);
            println!("Public key: {}", &wallet.public_key);
            println!("Private key: {}", &wallet.private_key);
            println!();
        }
    }
}
