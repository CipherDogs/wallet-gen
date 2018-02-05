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

use self::Coin::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Coin {
    #[cfg_attr(feature = "serde", serde(rename = "BTC"))]
    Bitcoin,

    #[cfg_attr(feature = "serde", serde(rename = "BCH"))]
    BitcoinCash,

    #[cfg_attr(feature = "serde", serde(rename = "BTG"))]
    BitcoinGold,

    #[cfg_attr(feature = "serde", serde(rename = "ETC"))]
    Electroneum,

    #[cfg_attr(feature = "serde", serde(rename = "ETH"))]
    Ethereum,

    #[cfg_attr(feature = "serde", serde(rename = "FTC"))]
    Feathercoin,
}

impl Coin {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "btc" | "BTC" => Some(Bitcoin),
            "bch" | "BCH" => Some(BitcoinCash),
            "btg" | "BTG" => Some(BitcoinGold),
            "etc" | "ETC" => Some(Electroneum),
            "eth" | "ETH" => Some(Ethereum),
            "ftc" | "FTC" => Some(Feathercoin),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Bitcoin => "BTC",
            BitcoinCash => "BCH",
            BitcoinGold => "BTG",
            Electroneum => "ETC",
            Ethereum => "ETH",
            Feathercoin => "FTC",
        }
    }
}
