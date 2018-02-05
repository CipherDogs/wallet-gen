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

use base58::ToBase58;
use openssl::bn::BigNumContext;
use openssl::ec::{EcGroup, EcKey, PointConversionForm};
use openssl::hash::{hash, MessageDigest};
use openssl::nid::Nid;
use openssl::sha::sha256;
use std::io::Write;
use super::prelude::*;
use utils::HexSlice;

pub fn new_wallet(coin: Coin, prefix: &[u8], version: u8) -> Result<Wallet> {
    let group = EcGroup::from_curve_name(Nid::SECP256K1)?;
    let mut bn_ctx = BigNumContext::new()?;
    let key = EcKey::generate(&group)?;
    let pub_key = key.public_key()
                     .to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut bn_ctx)?;
    let priv_key = key.private_key().to_vec();

    let digest = hash(MessageDigest::ripemd160(), &pub_key[..])?;
    let mut address: Vec<u8> = prefix.into();
    address.write(&digest).unwrap();

    Ok(Wallet {
        coin: coin,
        address: base58_check(&mut address, version),
        public_key: format!("{:x}", &HexSlice::new(&pub_key)),
        private_key: format!("{:x}", &HexSlice::new(&priv_key[..])),
        other: None,
    })
}

pub fn base58_check(bytes: &mut Vec<u8>, version: u8) -> String {
    bytes.insert(0, version);
    let head = &sha256(&sha256(&bytes))[..4];
    bytes.write(head).unwrap();
    bytes.as_slice().to_base58()
}

#[test]
fn gen_btc_wallet() {
    println!("wallet: {:?}", &new_wallet(Coin::Bitcoin, &[0x00], 0x00).unwrap());
}
