//! Helpers shared by the `ark-poly-commit` tests and benchmarks.
//!
//! The hashers below are always available. The benchmark harness, which needs
//! `criterion`, is behind the `bench` feature so that test builds do not link it.

use ark_crypto_primitives::crh::{sha256::digest::Digest, CRHScheme};
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use rand_chacha::rand_core::RngCore;
use std::{borrow::Borrow, marker::PhantomData};

use ark_poly_commit::to_bytes;

#[cfg(feature = "bench")]
mod bench;
#[cfg(feature = "bench")]
pub use bench::*;

/// Needed for benches and tests.
pub struct LeafIdentityHasher;

impl CRHScheme for LeafIdentityHasher {
    type Input = Vec<u8>;
    type Output = Vec<u8>;
    type Parameters = ();

    fn setup<R: RngCore>(_: &mut R) -> Result<Self::Parameters, ark_crypto_primitives::Error> {
        Ok(())
    }

    fn evaluate<T: Borrow<Self::Input>>(
        _: &Self::Parameters,
        input: T,
    ) -> Result<Self::Output, ark_crypto_primitives::Error> {
        Ok(input.borrow().to_vec())
    }
}

/// Needed for benches and tests.
pub struct FieldToBytesColHasher<F, D>
where
    F: PrimeField + CanonicalSerialize,
    D: Digest,
{
    _phantom: PhantomData<(F, D)>,
}

impl<F, D> CRHScheme for FieldToBytesColHasher<F, D>
where
    F: PrimeField + CanonicalSerialize,
    D: Digest,
{
    type Input = Vec<F>;
    type Output = Vec<u8>;
    type Parameters = ();

    fn setup<R: RngCore>(_rng: &mut R) -> Result<Self::Parameters, ark_crypto_primitives::Error> {
        Ok(())
    }

    fn evaluate<T: Borrow<Self::Input>>(
        _parameters: &Self::Parameters,
        input: T,
    ) -> Result<Self::Output, ark_crypto_primitives::Error> {
        let mut dig = D::new();
        dig.update(to_bytes!(input.borrow()).unwrap());
        Ok(dig.finalize().to_vec())
    }
}
