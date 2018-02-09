/*
 * ed25519/crypto.rs
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

use openssl::bn::{BigNum, BigNumRef, BigNumContext, BigNumContextRef};
use std::iter;
use super::prelude::*;

lazy_static! {
    /* ed25519 constant: (2 ** 252) + 27742317777372353535851937790883648493 */
    static ref L: BigNum = BigNum::from_dec_str("7237005577332262213973186563042994240857116359379907606001950938285454250989").unwrap();
}

/// Perform the `sc_reduce32` procedure on the given bytestring, producing
/// a 256-bit scalar usable as an Ed25519 private key.
pub fn sc_reduce32(bytes: &mut [u8; 32], ctx: &mut BigNumContextRef) -> Result<BigNum> {
    // Fix byte ordering
    #[cfg(target_endian = "little")]
    bytes.reverse();

    // Perform modulo
    let number = BigNum::from_slice(&bytes[..])?;
    let mut reduced = BigNum::new()?;
    reduced.checked_rem(&number, &*L, ctx)?;
    Ok(reduced)
}

/// Converts an OpenSsl [`BigNumRef`] into a [`Vec<u8>`] in big-endian form,
/// padding it with zero bytes until it is 32 bytes long.
///
/// [`BigNumRef`]: https://docs.rs/openssl/0.10.2/openssl/bn/struct.BigNumRef.html
/// [`Vec<u8>`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
pub fn bn_to_vec32(number: &BigNumRef) -> Vec<u8> {
    // Adds leading zeros
    let mut result = number.to_vec();
    let missing = 32 - result.len();
    result.splice(..0, iter::repeat(0).take(missing));
    assert_eq!(result.len(), 32);

    // Fix byte ordering
    #[cfg(target_endian = "little")]
    result.reverse();

    result
}
