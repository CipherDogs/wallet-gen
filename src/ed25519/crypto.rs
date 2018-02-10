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

use openssl::bn::{BigNum, BigNumRef, BigNumContextRef};
use std::iter;
use super::prelude::*;

lazy_static! {
    /* ed25519 constants: */

    /* (2 ** 255) - 19 */
    static ref Q: BigNum = BigNum::from_dec_str(
        "57896044618658097711785492504343953926634992332820282019728792003956564819949",
    ).unwrap();

    /* (2 ** 252) + 27742317777372353535851937790883648493 */
    static ref L: BigNum = BigNum::from_dec_str(
        "7237005577332262213973186563042994240857116359379907606001950938285454250989",
    ).unwrap();

    /* -121665 * inv(121666) */
    static ref D: BigNum = BigNum::from_dec_str(
        "-4513249062541557337682894930092624173785641285191125241628941591882900924598840740",
    ).unwrap();

    /*
     * B_y: 4 * inv(5)
     * B_x: xrecover(B_y)
     * B: (B_x, B_y)
     */
    static ref B: Point = Point {
        x: BigNum::from_dec_str(
           "15112221349535400772501151409588531511454012693041857206046113283949847762202",
        ).unwrap(),
        y: BigNum::from_dec_str(
            "46316835694926478169428394003475163141307993866256225615783033603165251855960",
        ).unwrap(),
    };

    /* Other constants */
    static ref ZERO: BigNum = BigNum::from_u32(0).unwrap();
    static ref ONE: BigNum = BigNum::from_u32(1).unwrap();
}

#[derive(Debug)]
pub struct Point {
    pub x: BigNum,
    pub y: BigNum,
}

impl Point {
    pub fn to_owned(&self) -> Result<Self> {
        Ok(Point {
            x: self.x.to_owned()?,
            y: self.y.to_owned()?,
        })
    }
}

pub fn sc_reduce32(bytes: &mut [u8; 32], ctx: &mut BigNumContextRef) -> Result<BigNum> {
    // To big endian
    #[cfg(target_endian = "little")]
    bytes.reverse();

    // Perform modulo
    let number = BigNum::from_slice(&bytes[..])?;
    let mut reduced = BigNum::new()?;
    reduced.checked_rem(&number, &*L, ctx)?;
    Ok(reduced)
}

pub fn derive_pubkey(bytes: &mut [u8; 32], ctx: &mut BigNumContextRef) -> Result<()> {
    // To big endian
    #[cfg(target_endian = "little")]
    bytes.reverse();

    // Run scalar multiplication
    let number = BigNum::from_slice(&bytes[..])?;
    let pt = scalar_mult(&*B, &number, ctx)?;
    encode_point(bytes, &pt);

    // Back to native byte order
    #[cfg(target_endian = "little")]
    bytes.reverse();

    Ok(())
}

fn encode_point(bytes: &mut [u8; 32], point: &Point) {
    let mut bits = [false; 256];
    for i in 0..255 {
        bits[i] = point.y.is_bit_set(i as i32);
    }
    bits[255] = point.x.is_bit_set(0);

    for i in 0..32 {
        let mut byte = 0;
        for j in 0..8 {
            byte |= 1 << bits[i * 8 + j] as u8;
        }
        bytes[i] = byte;
    }
}

pub fn inv(x: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<BigNum> {
    /* x ** (Q - 2) % Q */

    let mut q2 = Q.to_owned()?;
    q2.sub_word(2)?;
    let mut result = BigNum::new()?;
    result.mod_exp(x, &q2, &*Q, ctx)?;
    Ok(result)
}

pub fn edwards(pt1: &Point, pt2: &Point, ctx: &mut BigNumContextRef) -> Result<Point> {
    /*
     * d_mul = D * x1 * x2 * y1 * y2
     * x3 = (x1 * y2 + x2 * y1) * inv(1 + d_mul)
     * y3 = (y1 * y2 + x1 * x2) * inv(1 - d_mul)
     * -> (x3 % Q, y3 % Q)
     */

    let &Point {x: ref x1, y: ref y1} = pt1;
    let &Point {x: ref x2, y: ref y2} = pt2;

    let x_mul = {
        /* x_mul = x1 * y2 + x2 * y1 */
        let mut a = BigNum::new()?;
        a.checked_mul(&x1, &y2, ctx)?;
        let mut b = BigNum::new()?;
        b.checked_mul(&x2, &y1, ctx)?;
        let mut result = BigNum::new()?;
        result.checked_add(&a, &b)?;
        result
    };

    let y_mul = {
        /* y_mul = y1 * y2 + x1 * x2 */
        let mut a = BigNum::new()?;
        a.checked_mul(&y1, &y2, ctx)?;
        let mut b = BigNum::new()?;
        b.checked_mul(&x1, &x2, ctx)?;
        let mut result = BigNum::new()?;
        result.checked_add(&a, &b)?;
        result
    };

    let d_mul = {
        /* d_mul = D * x1 * x2 * y1 * y2 */
        let mut a = BigNum::new()?;
        a.checked_mul(&*D, &x1, ctx)?;
        let mut b = BigNum::new()?;
        b.checked_mul(&a, &x2, ctx)?;
        let mut c = BigNum::new()?;
        c.checked_mul(&b, &y1, ctx)?;
        let mut result = BigNum::new()?;
        result.checked_mul(&c, &y2, ctx)?;
        result
    };

    let x3 = {
        /* x_mul * inv(1 + d_mul) */
        let mut a = BigNum::new()?;
        a.checked_add(&*ONE, &d_mul)?;
        let b = inv(&a, ctx)?;
        let mut result = BigNum::new()?;
        result.checked_mul(&x_mul, &b, ctx)?;
        result
    };

    let y3 = {
        /* y_mul * inv(1 - d_mul) */
        let mut a = BigNum::new()?;
        a.checked_sub(&*ONE, &d_mul)?;
        let b = inv(&a, ctx)?;
        let mut result = BigNum::new()?;
        result.checked_mul(&y_mul, &b, ctx)?;
        result
    };

    let x3_q = {
        let mut result = BigNum::new()?;
        result.checked_rem(&x3, &*Q, ctx)?;
        result
    };

    let y3_q = {
        let mut result = BigNum::new()?;
        result.checked_rem(&y3, &*Q, ctx)?;
        result
    };

    Ok(Point {
        x: x3_q,
        y: y3_q,
    })
}

pub fn scalar_mult(p: &Point, e: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<Point> {
    if e == &*ZERO {
        return Ok(Point {
            x: ONE.to_owned()?,
            y: ZERO.to_owned()?,
        });
    }

    let mut e2 = e.to_owned()?;
    e2.div_word(2)?;
    let mut q = scalar_mult(p, &e2, ctx)?;
    q = edwards(&q, &q, ctx)?;

    if e.is_bit_set(0) {
        q = edwards(&q, &p, ctx)?;
    }

    q.to_owned()
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
