/*
 * coin.rs
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

//! Represents the various cryptocurrencies supported by this crate.

use self::Coin::*;

/// The actual enum that represents a cryptocurrency.
/// This enum is not intended to be matched exhaustively,
/// as new coins may be added in the future.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Coin {
    /// Bitcoin, symbol "BTC"
    #[cfg_attr(feature = "serde", serde(rename = "BTC"))]
    Bitcoin,

    /// Bitcoin Cash, symbol "BCH" (not to be confused with Bitcoin)
    #[cfg_attr(feature = "serde", serde(rename = "BCH"))]
    BitcoinCash,

    /// Bitcoin Gold, symbol "BTG" (not to be confused with Bitcoin)
    #[cfg_attr(feature = "serde", serde(rename = "BTG"))]
    BitcoinGold,

    /// Electroneum, symbol "ETC"
    #[cfg_attr(feature = "serde", serde(rename = "ETC"))]
    Electroneum,

    /// Ethereum, symbol "ETH"
    #[cfg_attr(feature = "serde", serde(rename = "ETH"))]
    Ethereum,

    /// Feathercoin, symbol "FTC"
    #[cfg_attr(feature = "serde", serde(rename = "FTC"))]
    Feathercoin,

    /// Litecoin, symbol "LTC"
    #[cfg_attr(feature = "serde", serde(rename = "LTC"))]
    Litecoin,

    #[doc(hidden)] __Nonexhaustive,
}

impl Coin {
    /// Converts a coin ID (e.g. `"BTC"`) into its appropriate enum value.
    /// Supports both fully lower-case and fully upper-case variants, but no
    /// mixed-case IDs.
    ///
    /// ```
    /// assert_eq!(Coin::from("LTC"), Some(Coin::Litecoin));
    /// assert_eq!(Coin::from("ltc"), Some(Coin::Litecoin));
    /// assert_eq!(Coin::from("Ltc"), None);
    /// assert_eq!(Coin::from("???"), None);
    /// ```
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "btc" | "BTC" => Some(Bitcoin),
            "bch" | "BCH" => Some(BitcoinCash),
            "btg" | "BTG" => Some(BitcoinGold),
            "etc" | "ETC" => Some(Electroneum),
            "eth" | "ETH" => Some(Ethereum),
            "ftc" | "FTC" => Some(Feathercoin),
            "ltc" | "LTC" => Some(Litecoin),
            _ => None,
        }
    }

    /// Gets the uppercase coin ID for an enum value.
    /// This is the opposite of the `from_id()` constructor method.
    ///
    /// ```
    /// let coin = Coin::Ethereum;
    /// assert_eq!(coin.id(), "ETH");
    /// assert_eq!(Some(coin), Coin::from(coin.id()));
    /// ```
    pub fn id(self) -> &'static str {
        match self {
            Bitcoin => "BTC",
            BitcoinCash => "BCH",
            BitcoinGold => "BTG",
            Electroneum => "ETC",
            Ethereum => "ETH",
            Feathercoin => "FTC",
            Litecoin => "LTC",
            __Nonexhaustive => unreachable!(),
        }
    }
}
