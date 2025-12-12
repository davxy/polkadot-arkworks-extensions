#![allow(dead_code)]

use crate::{
    ark_bandersnatch, CompressedPoint, IetfSignatureRaw, InputRaw, OutputRaw, PublicKeyRaw,
};
use ark_vrf::ietf::Prover;
use ark_vrf::reexports::ark_serialize::CanonicalSerialize;
use ark_vrf::reexports::ark_std::vec::Vec;

pub fn ring_members_gen(size: u32) -> Vec<PublicKeyRaw> {
    (0..size)
        .map(|i| {
            let pk = ark_bandersnatch::Secret::from_seed(&[i as u8]).public();
            let mut buf = PublicKeyRaw::default();
            pk.0.serialize_compressed(&mut buf.0[..]).unwrap();
            buf
        })
        .collect()
}

pub fn ietf_verify_params_gen() -> (PublicKeyRaw, InputRaw, OutputRaw, IetfSignatureRaw) {
    let secret = ark_bandersnatch::Secret::from_seed(b"secret");
    let public = secret.public();
    let input = ark_bandersnatch::Input::new(b"input").unwrap();
    let output = secret.output(input);
    let proof = secret.prove(input, output, &[]);

    (
        CompressedPoint(public.get_raw()),
        CompressedPoint(input.get_raw()),
        CompressedPoint(output.get_raw()),
        IetfSignatureRaw(proof.get_raw()),
    )
}

pub trait GetRaw<const N: usize>: CanonicalSerialize {
    fn get_raw(&self) -> [u8; N] {
        let mut buf = [0_u8; N];
        self.serialize_compressed(&mut buf[..]).unwrap();
        buf
    }
}

impl<T, const N: usize> GetRaw<N> for T where T: CanonicalSerialize {}
