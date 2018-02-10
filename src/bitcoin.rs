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
    let bitcoin_data = match wif_data(coin) {
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
        other:       None,
    })
}

/// Performs a "base58 check", a modified base 58 byte conversion
/// that appends a double SHA256 checksum.
///
/// This function modifies the buffer, prepending the
/// "application/version" byte.
pub fn base58_check(bytes: &mut Vec<u8>, prefix: &[u8]) -> String {
    bytes.splice(..0, prefix.iter().cloned());
    let hash = sha256(bytes);
    let checksum = &sha256(&hash[..])[..4];
    bytes.write_all(checksum).unwrap();
    bytes.as_slice().to_base58()
}

/// Constants for generating addresses and private keys in Bitcoin-based
/// coins, when converting to Wallet Import Format (WIF).
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BitcoinWifData(&'static [u8], u8, &'static str, &'static str);

impl BitcoinWifData {
    /// Gets the network version used in WIF conversion.
    #[inline]
    pub fn network_version(self) -> &'static [u8] { self.0 }

    /// Gets the prefix to prepend when creating the WIF private key.
    #[inline]
    pub fn private_key_prefix(self) -> u8 { self.1 }

    /// Returns the WIF character set for the beginning of the string.
    #[inline]
    pub fn wif_start(self) -> &'static str { self.2 }

    /// Returns the CWIF character set for the beginning of the string.
    #[inline]
    pub fn cwif_start(self) -> &'static str { self.3 }

    /// See if the given string matches the character set returned by `wif_start()`.
    pub fn check_wif(self, s: &str) -> bool {
        for c in self.wif_start().chars() {
            if s.starts_with(c) {
                return true;
            }
        }

        false
    }

    /// See if the given string matches the character set returned by `cwif_start()`.
    pub fn check_cwif(self, s: &str) -> bool {
        for c in self.cwif_start().chars() {
            if s.starts_with(c) {
                return true;
            }
        }

        false
    }
}

/// Gets the constant data used to generate WIF addresses.
#[allow(match_same_arms)]
pub fn wif_data(coin: Coin) -> Option<BitcoinWifData> {
    Some(match coin {
        Coin::Bitcoin => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
        Coin::Testnet => BitcoinWifData(&[0x6f], 0xef, "9", "c"),
        Coin::Litecoin => BitcoinWifData(&[0x30], 0xb0, "6", "T"),
        Coin::Dogecoin => BitcoinWifData(&[0x1e], 0x9e, "6", "Q"),
        Coin::Reddcoin => BitcoinWifData(&[0x3d], 0xbd, "7", "UV"),
        Coin::Dash => BitcoinWifData(&[0x4c], 0xcc, "7", "X"),
        Coin::Peercoin => BitcoinWifData(&[0x37], 0xb7, "7", "U"),
        Coin::Namecoin => BitcoinWifData(&[0x34], 0x80, "5", "LK"),
        Coin::Feathercoin => BitcoinWifData(&[0x0e], 0x8e, "5", "N"),
        Coin::Blackcoin => BitcoinWifData(&[0x19], 0x99, "6", "P"),
        Coin::NuBits => BitcoinWifData(&[0x19], 0xbf, "7", "V"),
        Coin::Mazacoin => BitcoinWifData(&[0x32], 0xe0, "8", "a"),
        Coin::Viacoin => BitcoinWifData(&[0x47], 0xc7, "7", "W"),
        Coin::Rubycoin => BitcoinWifData(&[0x3c], 0xbc, "7", "U"),
        Coin::Digitalcoin => BitcoinWifData(&[0x1e], 0x9e, "6", "Q"),
        Coin::Cannacoin => BitcoinWifData(&[0x1c], 0x9c, "6", "Q"),
        Coin::DigiByte => BitcoinWifData(&[0x1e], 0x9e, "6", "Q"),
        Coin::Primecoin => BitcoinWifData(&[0x17], 0x97, "6", "P"),
        Coin::Neoscoin => BitcoinWifData(&[0x35], 0xb1, "6", "T"),
        Coin::Jumbucks => BitcoinWifData(&[0x2b], 0xab, "6", "S"),
        Coin::Vertcoin => BitcoinWifData(&[0x47], 0x80, "5", "LK"),
        Coin::MonetaryUnit => BitcoinWifData(&[0x10], 0x7e, "5", "K"),
        Coin::CanadaeCoin => BitcoinWifData(&[0x1c], 0x9c, "6", "Q"),
        Coin::ParkByte => BitcoinWifData(&[0x37], 0xb7, "7", "U"),
        Coin::Pandacoin => BitcoinWifData(&[0x37], 0xb7, "7", "U"),
        Coin::Particl => BitcoinWifData(&[0x38], 0x6c, "4", "HG"),
        Coin::Novacoin => BitcoinWifData(&[0x08], 0x88, "5", "M"),
        Coin::Bitcoindark => BitcoinWifData(&[0x3c], 0xbc, "7", "U"),
        Coin::Syscoin => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
        Coin::Smileycoin => BitcoinWifData(&[0x19], 0x99, "6", "P"),
        Coin::FujiCoin => BitcoinWifData(&[0x24], 0xa4, "6", "R"),
        Coin::ElectronicGulden => BitcoinWifData(&[0x30], 0xb0, "6", "T"),
        Coin::Potcoin => BitcoinWifData(&[0x37], 0xb7, "7", "U"),
        Coin::Quarkcoin => BitcoinWifData(&[0x3a], 0xba, "7", "U"),
        Coin::Terracoin => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
        Coin::Gridcoin => BitcoinWifData(&[0x3e], 0xbe, "7", "V"),
        Coin::Auroracoin => BitcoinWifData(&[0x17], 0x97, "6", "T"),
        Coin::Gulden => BitcoinWifData(&[0x26], 0xa6, "6", "R"),
        Coin::Myriadcoin => BitcoinWifData(&[0x32], 0xb2, "6", "T"),
        Coin::Unobtanium => BitcoinWifData(&[0x82], 0xe0, "8", "a"),
        Coin::Stratis => BitcoinWifData(&[0x3f], 0xbf, "7", "V"),
        Coin::MarsCoin => BitcoinWifData(&[0x32], 0xb2, "6", "T"),
        Coin::Pesetacoin => BitcoinWifData(&[0x2f], 0xaf, "6", "ST"),
        Coin::Pinkcoin => BitcoinWifData(&[0x03], 0x83, "RQP", "L"),
        Coin::PiggyCoin => BitcoinWifData(&[0x76], 0xf6, "9", "d"),
        Coin::Pivx => BitcoinWifData(&[0x1e], 0xd4, "8", "Y"),
        Coin::BitZeny => BitcoinWifData(&[0x51], 0x80, "5", "LK"),
        Coin::StealthCoin => BitcoinWifData(&[0x3e], 0xbe, "7", "V"),
        Coin::Vcash => BitcoinWifData(&[0x47], 0xc7, "7", "W"),
        Coin::NavCoin => BitcoinWifData(&[0x35], 0x96, "6", "P"),
        Coin::Zcash => BitcoinWifData(&[0x1c, 0xb8], 0x80, "5", "LK"),
        Coin::LBRYCredits => BitcoinWifData(&[0x55], 0x80, "5", "LK"),
        Coin::Riecoin => BitcoinWifData(&[0x3c], 0x80, "5", "LK"),
        Coin::BitcoinCash => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
        Coin::BitcoinGold => BitcoinWifData(&[0x26], 0x80, "5", "LK"),
        Coin::Bitcore => BitcoinWifData(&[0x00], 0x80, "5", "LK"),
        Coin::Ember => BitcoinWifData(&[0x5c], 0x32, "2", "8"),
        Coin::HTMLCOIN => BitcoinWifData(&[0x29], 0xa9, "6", "S"),
        Coin::MarteXcoin => BitcoinWifData(&[0x32], 0xb2, "6", "T"),
        Coin::Omni => BitcoinWifData(&[0x73], 0xf3, "9", "cd"),
        Coin::BoxyCoin => BitcoinWifData(&[0x4b], 0xcb, "7", "X"),
        Coin::Blocknet => BitcoinWifData(&[0x1a], 0x9a, "6", "P"),
        Coin::HOdlcoin => BitcoinWifData(&[0x28], 0xa8, "5", "LK"),
        Coin::Axe => BitcoinWifData(&[0x4b], 0xcb, "7", "X"),
        Coin::__Nonexhaustive => unreachable!(),
        _ => return None,
    })
}

#[test]
fn gen_wallets() {
    use coin::COINS;

    println!("Generating wallets for Bitcoin variants...");
    for &coin in COINS.iter() {
        if wif_data(coin).is_some() {
            let wallet = new_wallet(coin).unwrap();
            println!("Coin: {:?} ({})", coin, coin.symbol());
            println!("Address: {}", &wallet.address);
            println!("Public key: {}", &wallet.public_key);
            println!("Private key: {}", &wallet.private_key);
            println!();
        }
    }
}
