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

//! A representation of a cryptocurrency wallet. Stores in that
//! coin's native WIF, or "wallet import format".

use super::prelude::*;
use {bitcoin, ethereum, feathercoin};

/// The actual wallet structure.
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
    /// Generates a new random wallet for the given coin.
    /// The private key will be sourced from a source of
    /// cryptographically-secure randomness. See OpenSSL
    /// documentation for more details.
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

    /// Formats this object as a JSON-formatted string.
    /// Does not require `serde` to be enabled.
    pub fn to_json_str(&self) -> String {
        format!(
            "{{\"coin\":\"{}\",\"address\":{:?},\"public_key\":{:?},\"private_key\":{:?}}}",
            self.coin.id(),
            &self.address,
            &self.public_key,
            &self.private_key
        )
    }
}

#[test]
fn test_json() {
    let wal = Wallet {
        coin: Coin::Bitcoin,
        address: "1DpsvYcuTn9v77UJHcXUFFGUCE1xtqTD49".into(),
        public_key: "047E05C068B8DC841D231EB874A60B4C11993A3954224F867CA8F0624F7D356A5252F7E474834321F49F978DFE10597F6435A7C065D320E8C06008BB6BA32AE1F0".into(),
        private_key: "5JCobnL5QoLwfvSauWCjtPnMa6nmno7hBm8LrwgSmW14SUX1q4u".into(),
    };

    assert_eq!(&wal.to_json_str(), "{\"coin\":\"BTC\",\"address\":\"1DpsvYcuTn9v77UJHcXUFFGUCE1xtqTD49\",\"public_key\":\"047E05C068B8DC841D231EB874A60B4C11993A3954224F867CA8F0624F7D356A5252F7E474834321F49F978DFE10597F6435A7C065D320E8C06008BB6BA32AE1F0\",\"private_key\":\"5JCobnL5QoLwfvSauWCjtPnMa6nmno7hBm8LrwgSmW14SUX1q4u\"}");
}
