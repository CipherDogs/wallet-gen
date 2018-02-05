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

//! A representation of a cryptocurrency wallet in WIF (Wallet Import Format).
//! Can be converted to JSON if you enable the `serde` feature.

use {bitcoin, ethereum, feathercoin};
use super::prelude::*;

/// A representation of a cryptocurrency wallet. Stores in that
/// coin's native WIF, or "wallet import format".
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Wallet {
    /// Which cryptocurrency this wallet is for
    pub coin: Coin,

    /// The wallet address as a human-readable string
    pub address: String,

    /// The wallet's public key in WIF format
    pub public_key: String,

    /// The wallet's private key in WIF format
    pub private_key: String,
}

impl Wallet {
    /// Generates a new random cryuptocurrency wallet for
    /// the given coin.
    pub fn generate(coin: Coin) -> Result<Self> {
        use self::Coin::*;

        match coin {
            Bitcoin | BitcoinCash | Litecoin => bitcoin::new_wallet(coin),
            BitcoinGold => unimplemented!(),
            Electroneum => unimplemented!(),
            Ethereum => ethereum::new_wallet(),
            Feathercoin => feathercoin::new_wallet(),
            __Nonexhaustive => unreachable!(),
        }
    }
}
