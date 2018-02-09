/*
 * utils/sha512.rs
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

//! Wrapper around OpenSsl's SHA512 implementation that implements the
//! [`Digest`] trait for use by the [`ed25519_dalek`] crate.
//!
//! [`Digest`]: https://docs.rs/digest/0.7.2/digest/trait.Digest.html
//! [`ed25519_dalek`]: https://docs.rs/ed25519-dalek/0.6.1/ed25519_dalek/

use digest::{BlockInput, FixedOutput, Input};
use digest::generic_array::GenericArray;
use digest::generic_array::typenum::{U64, U128};
use openssl::sha::sha512;
use std::fmt::{self, Debug};
use std::ops::Deref;

/// Wrapper struct that implements [`Digest`] for OpenSsl's SHA512.
#[derive(Clone)]
pub struct Sha512([u8; 64]);

impl Deref for Sha512 {
    type Target = [u8; 64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Sha512 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sha512([ ... ])")
    }
}

impl Default for Sha512 {
    fn default() -> Self {
        Sha512([0; 64])
    }
}

impl BlockInput for Sha512 {
    type BlockSize = U128;
}

impl Input for Sha512 {
    fn process(&mut self, msg: &[u8]) {
        self.0 = sha512(msg);
    }
}

impl FixedOutput for Sha512 {
    type OutputSize = U64;

    fn fixed_result(self) -> GenericArray<u8, Self::OutputSize> {
        GenericArray::clone_from_slice(&self.0[..])
    }
}
