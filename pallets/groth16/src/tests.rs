use crate::{
    mock::{new_test_ext, ArkGroth16, RuntimeOrigin},
    utils,
};
use frame_support::assert_ok;

// ---------------------------------------------
// Tests for bls12-381
// ---------------------------------------------

fn bls12_381_groth16_verify(optimized: bool, pregen: bool) {
    let (vk, public_input, proof) = if pregen {
        // For parameters generation we use the built-in arkworks implementation,
        // the hostcalls are relevant only during the verification phase.
        utils::groth16_verify_params_gen::<ark_bls12_381::Bls12_381>()
    } else {
        utils::bls12_381_groth16_verify_params_get_pregen()
    };

    new_test_ext().execute_with(|| {
        assert_ok!(ArkGroth16::bls12_381_groth16_verify(
            RuntimeOrigin::none(),
            vk.0,
            public_input.0,
            proof.0,
            optimized
        ));
    });
}

#[test]
fn ark_bls12_381_groth16_verify() {
    bls12_381_groth16_verify(false, false);
}

#[test]
fn sub_bls12_381_groth16_verify() {
    bls12_381_groth16_verify(true, false);
}

#[test]
fn ark_bls12_381_groth16_verify_pregen() {
    bls12_381_groth16_verify(false, true);
}

#[test]
fn sub_bls12_381_groth16_verify_pregen() {
    bls12_381_groth16_verify(true, true);
}

// ---------------------------------------------
// Tests for bls12-377
// ---------------------------------------------

fn bls12_377_groth16_verify(optimized: bool) {
    let (vk, public_input, proof) = utils::groth16_verify_params_gen::<ark_bls12_377::Bls12_377>();

    new_test_ext().execute_with(|| {
        assert_ok!(ArkGroth16::bls12_377_groth16_verify(
            RuntimeOrigin::none(),
            vk.0,
            public_input.0,
            proof.0,
            optimized
        ));
    });
}

#[test]
fn ark_bls12_377_groth16_verify() {
    bls12_377_groth16_verify(false);
}

#[test]
fn sub_bls12_377_groth16_verify() {
    bls12_377_groth16_verify(true);
}

// ---------------------------------------------
// Tests for BW6-761
// ---------------------------------------------

fn bw6_761_groth16_verify(optimized: bool) {
    let (vk, public_input, proof) = utils::groth16_verify_params_gen::<ark_bw6_761::BW6_761>();

    new_test_ext().execute_with(|| {
        assert_ok!(ArkGroth16::bw6_761_groth16_verify(
            RuntimeOrigin::none(),
            vk.0,
            public_input.0,
            proof.0,
            optimized
        ));
    });
}

#[test]
fn ark_bw6_761_groth16_verify() {
    bw6_761_groth16_verify(false);
}

#[test]
fn sub_bw6_761_groth16_verify() {
    bw6_761_groth16_verify(true);
}
