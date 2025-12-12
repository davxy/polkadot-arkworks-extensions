use crate::{
    mock::{new_test_ext, ArkHostcalls, RuntimeOrigin},
    sub_ed_on_bls12_381_bandersnatch,
    utils::*,
};
use ark_scale::scale::Encode;
use frame_support::assert_ok;

const MSM_ITEMS: u32 = 500;
const SCALAR_WORDS: u32 = 3;

// ---------------------------------------------
// Tests for ed-on-bls12-381-bandersnatch
// ---------------------------------------------

fn ed_on_bls12_381_bandersnatch_msm_sw(optimized: bool) {
    let (bases, scalars) =
        make_msm_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_msm_sw(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode(),
            optimized,
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_msm_sw() {
    ed_on_bls12_381_bandersnatch_msm_sw(false);
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_msm_sw() {
    ed_on_bls12_381_bandersnatch_msm_sw(true);
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_msm_te() {
    let (bases, scalars) =
        make_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_msm_te(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode(),
            false
        ));
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_msm_te() {
    let (bases, scalars) =
        make_msm_args::<sub_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_msm_te(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode(),
            true
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
    let (base, scalar) =
        make_scalar_args_projective::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_mul_projective_sw(
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
            ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_mul_projective_sw(
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
        assert_ok!(
            ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_mul_affine_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
    let (base, scalar) =
        make_scalar_args::<sub_ed_on_bls12_381_bandersnatch::SWAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_mul_affine_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_projective_te() {
    let (base, scalar) = make_scalar_args_projective::<
        ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
    >(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_mul_projective_te(
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
            ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_mul_projective_te(
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
        assert_ok!(
            ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_mul_affine_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_affine_te() {
    let (base, scalar) =
        make_scalar_args::<sub_ed_on_bls12_381_bandersnatch::EdwardsAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_mul_affine_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}
