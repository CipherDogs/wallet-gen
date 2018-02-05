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

#[deny(missing_debug_implementations)]
#[warn(missing_docs)]

/// A Rust library to generate various cryptocurrency wallets.

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
