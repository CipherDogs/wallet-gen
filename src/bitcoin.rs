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
    let (wif_version, p2pkh_version, _p2sh_version) = match get_mainnet_versions(coin) {
        Some((wif, p2pkh, p2sh)) => (wif, p2pkh, p2sh),
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
        address:     base58_check(&mut address, p2pkh_version),
        public_key:  HexSlice::new(&pub_key).format(),
        private_key: base58_check(&mut priv_key, wif_version),
    })
}

/// Gets the mainnet version fields for the given coin.
/// Only supports coins that Bitcoin-derived addressing.
/// Returns `(mainnet_version_WIF, mainnet_version_p2pkh, mainnet_version_p2sh)`.
pub fn get_mainnet_versions(coin: Coin) -> Option<(u8, u8, u8)> {
    Some(match coin {
        Coin::Bitcoin => (128, 0, 5),
        Coin::Testnet => (239, 111, 196),
        Coin::Litecoin => (176, 48, 5),
        Coin::Dogecoin => (158, 30, 22),
        Coin::Reddcoin => (189, 61, 5),
        Coin::Dash => (204, 76, 16),
        Coin::Peercoin => (183, 55, 117),
        Coin::Namecoin => (180, 52, 13),
        Coin::Feathercoin => (142, 14, 5),
        Coin::Blackcoin => (153, 25, 85),
        Coin::NuShares => (149, 63, 64),
        Coin::NuBits => (150, 25, 26),
        Coin::Mazacoin => (224, 50, 9),
        Coin::Viacoin => (199, 71, 33),
        Coin::Rubycoin => (189, 61, 85),
        Coin::Groestlcoin => (128, 36, 5),
        Coin::Digitalcoin => (128, 36, 5),
        Coin::Cannacoin => (156, 28, 5),
        Coin::DigiByte => (158, 30, 5),
        Coin::Monacoin => (176, 50, 5),
        Coin::Clams => (133, 137, 13),
        Coin::Primecoin => (151, 23, 83),
        Coin::Jumbucks => (171, 43, 105),
        Coin::ZiftCoin => (208, 80, 5),
        Coin::Vertcoin => (199, 71, 5),
        Coin::MonetaryUnit => (143, 15, 9),
        Coin::Zoom => (231, 103, 92),
        Coin::Vpncoin => (199, 71, 5),
        Coin::CanadaEcoin => (156, 28, 5),
        Coin::ShadowCash => (191, 63, 125),
        Coin::ParkByte => (183, 55, 28),
        Coin::Pandacoin => (183, 55, 22),
        Coin::StartCoin => (253, 125, 5),
        Coin::GcrCoin => (154, 38, 97),
        Coin::Novacoin => (136, 8, 20),
        Coin::Asiacoin => (151, 23, 8),
        Coin::Bitcoindark => (188, 60, 85),
        Coin::Dopecoin => (136, 8, 5),
        Coin::Templecoin => (193, 65, 5),
        Coin::OkCash => (183, 55, 28),
        Coin::ElectronicGulden => (176, 48, 5),
        Coin::Potcoin => (183, 55, 5),
        Coin::Zcash => (128, 28, 28),
        _ => return None,
    })
}

/// Performs a "base58 check", a modified base 58 byte conversion
/// that appends a double SHA256 checksum.
///
/// This function modifies the buffer, prepending the
/// "application/version" byte.
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

#[test]
fn gen_ltc_wallet() {
    println!("{:?}", &new_wallet(Coin::Litecoin).unwrap());
}

#[test]
fn gen_ftc_wallet() {
    println!("{:?}", &new_wallet(Coin::Feathercoin).unwrap());
}

#[test]
fn gen_zec_wallet() {
    println!("{:?}", &new_wallet(Coin::Zcash).unwrap());
}
