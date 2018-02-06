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
use {bitcoin, ethereum};

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
            Bitcoin | Testnet | Litecoin | Dogecoin | Reddcoin | Dash | Peercoin | Namecoin
            | Feathercoin | Counterparty | Blackcoin | NuShares | NuBits | Mazacoin | Viacoin
            | ClearingHouse | Rubycoin | Groestlcoin | Digitalcoin | Cannacoin | DigiByte
            | Monacoin | Clams | Primecoin | Neoscoin | Jumbucks | ZiftCoin | NXT
            | MonetaryUnit | Zoom | Vpncoin | CanadaEcoin | ShadowCash | ParkByte | Pandacoin
            | StartCoin | GcrCoin | Novacoin | Asiacoin | Bitcoindark | Dopecoin | Templecoin
            | OkCash | ElectronicGulden | Potcoin | Ripple | Monero | Zcash => {
                bitcoin::new_wallet(coin)
            },
            Ethereum | EthereumClassic => ethereum::new_wallet(coin),
            _ => Err(Error::CoinNotSupported(coin)),
        }
    }

    /// Formats this object as a JSON-formatted string.
    /// Does not require `serde` to be enabled.
    pub fn to_json_str(&self) -> String {
        format!(
            "{{\"coin\":\"{}\",\"address\":{:?},\"public_key\":{:?},\"private_key\":{:?}}}",
            self.coin.symbol(),
            &self.address,
            &self.public_key,
            &self.private_key
        )
    }
}

#[test]
fn test_json() {
    let wal = Wallet {
        coin:        Coin::Bitcoin,
        address:     "addr".into(),
        public_key:  "pub".into(),
        private_key: "priv".into(),
    };

    assert_eq!(
        &wal.to_json_str(),
        "{\"coin\":\"BTC\",\"address\":\"addr\",\"public_key\":\"pub\",\"private_key\":\"priv\"}"
    );
}
