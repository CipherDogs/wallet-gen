/*
 * ed25519/keys.rs
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

use openssl::bn::{BigNumContext, BigNumContextRef};
use ed25519::{bn_to_vec32, sc_reduce32};
use std::ops::Deref;
use super::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrivateKey([u8; 32]);

impl PrivateKey {
    pub fn from_bytes(mut bytes: [u8; 32], ctx: &mut BigNumContextRef) -> Result<Self> {
        let bn = sc_reduce32(&mut bytes, ctx)?;
        let vec = bn_to_vec32(&bn);

        {
            let dest = &mut bytes[..];
            dest.copy_from_slice(&vec[..]);
        }

        Ok(PrivateKey(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.clone()
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Deref for PrivateKey {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for PrivateKey {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PublicKey([u8; 32]);

impl PublicKey {
    pub fn from_private(_priv: &PrivateKey) -> Result<Self> {
        unimplemented!()
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.clone()
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Deref for PublicKey {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Keypair {
    pub public: PublicKey,
    pub private: PrivateKey,
}
