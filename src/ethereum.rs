/*
 * ethereum.rs
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

//! Various functions related to Ethereum wallet generation and validation.

use super::prelude::*;
use hex_slice::HexSlice;
use openssl::bn::BigNumContext;
use openssl::ec::{EcGroup, EcKey, PointConversionForm};
use openssl::nid::Nid;
use tiny_keccak::keccak256;

/// Generate a new Ethereum or Ethereum derivative or smart contract wallet.
pub fn new_wallet(coin: Coin) -> Result<Wallet> {
    let group = EcGroup::from_curve_name(Nid::SECP256K1)?;
    let key = EcKey::generate(&group)?;
    let pub_key = {
        let mut bn_ctx = BigNumContext::new()?;
        let mut vec =
            key.public_key()
                .to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut bn_ctx)?;
        vec.remove(0);
        vec
    };
    let priv_key = key.private_key().to_vec();
    let hash_bytes = keccak256(&pub_key);

    Ok(Wallet {
        coin:        coin,
        address:     format!("0x{:x}", &HexSlice::new(&hash_bytes[12..])),
        public_key:  format!("{:x}", &HexSlice::new(&pub_key)),
        private_key: format!("{:x}", &HexSlice::new(&priv_key[..])),
        other:       None,
    })
}

#[test]
fn gen_eth_wallet() {
    println!("{:#?}", &new_wallet(Coin::Ethereum).unwrap());
}

#[test]
fn gen_etc_wallet() {
    println!("{:#?}", &new_wallet(Coin::EthereumClassic).unwrap());
}
