/*
 * lib.rs
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

extern crate base58;
extern crate either;
extern crate openssl;
extern crate tiny_keccak;

pub mod bitcoin;
pub mod coin;
pub mod ethereum;
pub mod error;
pub mod feathercoin;
pub mod prelude;
pub mod utils;
pub mod wallet;

pub use self::prelude::*;

pub fn new_wallet(coin: Coin) -> Result<Wallet> {
    use self::Coin::*;

    match coin {
        Bitcoin | BitcoinCash => bitcoin::new_wallet(coin, &[0x00], 0x00),
        BitcoinGold => unimplemented!(),
        Electroneum => unimplemented!(),
        Ethereum => ethereum::new_wallet(),
        Feathercoin => feathercoin::new_wallet(),
    }
}
