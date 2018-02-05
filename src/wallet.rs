/*
 * wallet.rs
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

use {bitcoin, ethereum, feathercoin};
use std::collections::HashMap;
use super::prelude::*;

/// A representation of a cryptocurrency wallet.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Wallet {
    pub coin: Coin,
    pub address: String,
    pub public_key: String,
    pub private_key: String,
    pub other: Option<HashMap<String, String>>,
}

impl Wallet {
    pub fn new(coin: Coin) -> Result<Self> {
        use self::Coin::*;

        match coin {
            Bitcoin | BitcoinCash => bitcoin::new_wallet(coin),
            BitcoinGold => unimplemented!(),
            Electroneum => unimplemented!(),
            Ethereum => ethereum::new_wallet(),
            Feathercoin => feathercoin::new_wallet(),
        }
    }
}
