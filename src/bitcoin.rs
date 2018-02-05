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

pub fn new_wallet(coin: Coin, prefix: &[u8]) -> Result<Wallet> {
    let group = EcGroup::from_curve_name(Nid::SECP256K1)?;
    let mut bn_ctx = BigNumContext::new()?;
    let key = EcKey::generate(&group)?;
    let pub_key = key.public_key()
                     .to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut bn_ctx)?;
    let priv_key = key.private_key().to_vec();

    let digest = hash(MessageDigest::ripemd160(), &pub_key[..])?;
    let mut address: Vec<u8> = prefix.into();
    address.write(&digest).unwrap();
    let checksum = sha256(&sha256(&address));
    address.write(&checksum[..4]).unwrap();

    Ok(Wallet {
        coin: coin,
        address: address.to_base58(),
        public_key: format!("{:x}", &HexSlice::new(&pub_key)),
        private_key: format!("{:x}", &HexSlice::new(&priv_key[..])),
        other: None,
    })
}

#[test]
fn gen_btc_wallet() {
    println!("wallet: {:?}", &new_wallet(Coin::Bitcoin, 0x01).unwrap());
}
