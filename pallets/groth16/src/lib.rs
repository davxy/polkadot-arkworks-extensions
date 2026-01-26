#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod utils;
mod weights;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use ark_ec::pairing::Pairing;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_std::vec::Vec;

pub use sp_crypto_ec_utils::{
    bls12_377 as sub_bls12_377, bls12_381 as sub_bls12_381, bw6_761 as sub_bw6_761,
};

pub type ArkScaleWire<T> = ark_scale::ArkScale<T, { ark_scale::WIRE }>;
pub type ArkScaleHost<T> = ark_scale::ArkScale<T, { ark_scale::HOST_CALL }>;

pub use pallet::*;
pub use weights::*;

use crate::utils::deserialize_uncompressed_host;

const DEFAULT_WEIGHT: u64 = 10_000;

pub type ScalarFieldFor<PairingT> = <PairingT as Pairing>::ScalarField;

pub type VerifierKeyFor<PairingT> =
    <Groth16<PairingT> as SNARK<<PairingT as Pairing>::ScalarField>>::VerifyingKey;

pub type ProverKeyFor<PairingT> =
    <Groth16<PairingT> as SNARK<<PairingT as Pairing>::ScalarField>>::ProvingKey;

pub type ProofFor<PairingT> =
    <Groth16<PairingT> as SNARK<<PairingT as Pairing>::ScalarField>>::Proof;

pub fn groth16_verify<P: Pairing>(vk: Vec<u8>, c: Vec<u8>, proof: Vec<u8>) {
    let vk = deserialize_uncompressed_host::<VerifierKeyFor<P>>(vk);
    let c = deserialize_uncompressed_host::<ScalarFieldFor<P>>(c);
    let proof = deserialize_uncompressed_host::<ProofFor<P>>(proof);
    let result = Groth16::<P>::verify(&vk, &[c], &proof).unwrap();
    assert!(result);
}

#[frame_support::pallet]
pub mod pallet {

    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Extrinsic weights
        type WeightInfo: WeightInfo;
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// BLS12-381 Groth16 proof verification
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_groth16_verify(
            _: OriginFor<T>,
            vk: Vec<u8>,
            c: Vec<u8>,
            proof: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                groth16_verify::<sub_bls12_381::Bls12_381>(vk, c, proof);
            } else {
                groth16_verify::<ark_bls12_381::Bls12_381>(vk, c, proof);
            }
            Ok(())
        }

        /// BLS12-377 Groth16 proof verification
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_377_groth16_verify(
            _: OriginFor<T>,
            vk: Vec<u8>,
            c: Vec<u8>,
            proof: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                groth16_verify::<sub_bls12_377::Bls12_377>(vk, c, proof);
            } else {
                groth16_verify::<ark_bls12_377::Bls12_377>(vk, c, proof);
            }
            Ok(())
        }

        /// BW6-761 Groth16 proof verification
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bw6_761_groth16_verify(
            _: OriginFor<T>,
            vk: Vec<u8>,
            c: Vec<u8>,
            proof: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                groth16_verify::<sub_bw6_761::BW6_761>(vk, c, proof);
            } else {
                groth16_verify::<ark_bw6_761::BW6_761>(vk, c, proof);
            }
            Ok(())
        }
    }
}
