use crate::{
    mock::{new_test_ext, Arkworks, RuntimeOrigin},
    sub_ed_on_bls12_381_bandersnatch, ArkScale, ArkScaleProjective,
};
use ark_scale::scale::Encode;
use ark_std::{test_rng, UniformRand};
use polkadot_sdk::frame_support::assert_ok;

const MSM_LEN: u32 = 10;
const SCALAR_WORDS: u32 = 3;

pub fn make_msm_args<Group: ark_ec::VariableBaseMSM>(
    size: u32,
) -> (ArkScale<Vec<Group>>, ArkScale<Vec<Group::ScalarField>>) {
    let rng = &mut test_rng();
    let scalars = (0..size)
        .map(|_| Group::ScalarField::rand(rng))
        .collect::<Vec<_>>();
    let bases = (0..size).map(|_| Group::rand(rng)).collect::<Vec<_>>();
    (bases.into(), scalars.into())
}

// `words_count` is the scalar length in words, with 1 word assumed to be 64 bits.
// Most significant bit is set.
// Arkworks assumes scalar to be in **big endian**
fn make_scalar(words_count: u32) -> Vec<u64> {
    let mut scalar: Vec<_> = (0..words_count as usize)
        .map(|_| u64::rand(&mut test_rng()))
        .collect();
    scalar[0] |= 1 << 63;
    scalar
}

fn make_base<Group: UniformRand>() -> Group {
    Group::rand(&mut test_rng())
}

// `words_count` is the scalar length in words, with 1 word assumed to be 64 bits.
// Most significant bit is set.
pub fn make_scalar_args_projective<Group: UniformRand>(
    words_count: u32,
) -> (ArkScaleProjective<Group>, ArkScale<Vec<u64>>) {
    (make_base::<Group>().into(), make_scalar(words_count).into())
}

// `words_count` is the scalar length in words, with 1 word assumed to be 64 bits.
// Most significant bit is set.
pub fn make_scalar_args<Group: UniformRand>(
    words_count: u32,
) -> (ArkScale<Group>, ArkScale<Vec<u64>>) {
    (make_base::<Group>().into(), make_scalar(words_count).into())
}

// ---------------------------------------------
// Tests for ed-on-bls12-381-bandersnatch
// ---------------------------------------------

#[test]
fn ark_ed_on_bls12_381_bandersnatch_msm_sw() {
    let (bases, scalars) = make_msm_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(MSM_LEN);

    new_test_ext().execute_with(|| {
        assert_ok!(Arkworks::ark_ed_on_bls12_381_bandersnatch_msm_sw(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode()
        ));
    });
}
#[test]
fn sub_ed_on_bls12_381_bandersnatch_msm_sw() {
    let (bases, scalars) = make_msm_args::<sub_ed_on_bls12_381_bandersnatch::SWProjective>(MSM_LEN);

    new_test_ext().execute_with(|| {
        assert_ok!(Arkworks::sub_ed_on_bls12_381_bandersnatch_msm_sw(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode()
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_msm_te() {
    let (bases, scalars) =
        make_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_LEN);

    new_test_ext().execute_with(|| {
        assert_ok!(Arkworks::ark_ed_on_bls12_381_bandersnatch_msm_te(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode()
        ));
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_msm_te() {
    let (bases, scalars) =
        make_msm_args::<sub_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_LEN);

    new_test_ext().execute_with(|| {
        assert_ok!(Arkworks::sub_ed_on_bls12_381_bandersnatch_msm_te(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode()
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
    let (base, scalar) =
        make_scalar_args_projective::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            Arkworks::ark_ed_on_bls12_381_bandersnatch_mul_projective_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
    new_test_ext().execute_with(|| {
        let (base, scalar) = make_scalar_args_projective::<
            sub_ed_on_bls12_381_bandersnatch::SWProjective,
        >(SCALAR_WORDS);

        assert_ok!(
            Arkworks::sub_ed_on_bls12_381_bandersnatch_mul_projective_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
    let (base, scalar) =
        make_scalar_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(Arkworks::ark_ed_on_bls12_381_bandersnatch_mul_affine_sw(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode()
        ));
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
    let (base, scalar) =
        make_scalar_args::<sub_ed_on_bls12_381_bandersnatch::SWAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(Arkworks::sub_ed_on_bls12_381_bandersnatch_mul_affine_sw(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode()
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_projective_te() {
    let (base, scalar) = make_scalar_args_projective::<
        ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
    >(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            Arkworks::ark_ed_on_bls12_381_bandersnatch_mul_projective_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_projective_te() {
    let (base, scalar) = make_scalar_args_projective::<
        ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
    >(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            Arkworks::sub_ed_on_bls12_381_bandersnatch_mul_projective_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_affine_te() {
    let (base, scalar) =
        make_scalar_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(Arkworks::ark_ed_on_bls12_381_bandersnatch_mul_affine_te(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode()
        ));
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_affine_te() {
    let (base, scalar) =
        make_scalar_args::<sub_ed_on_bls12_381_bandersnatch::EdwardsAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(Arkworks::sub_ed_on_bls12_381_bandersnatch_mul_affine_te(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode()
        ));
    });
}
