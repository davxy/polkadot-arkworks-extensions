use super::*;

#[allow(unused)]
use crate::Pallet as ArkGroth16;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    // ---------------------------------------------
    // Calls for bls12-381
    // ---------------------------------------------

    #[benchmark]
    fn ark_bls12_381_groth16_verify() {
        let (vk, public_input, proof) =
            utils::groth16_verify_params_gen::<ark_bls12_381::Bls12_381>();
        // utils::bls12_381_groth16_verify_params_get_pregen()

        #[extrinsic_call]
        bls12_381_groth16_verify(RawOrigin::None, vk.0, public_input.0, proof.0, false);
    }

    #[benchmark]
    fn sub_bls12_381_groth16_verify() {
        let (vk, public_input, proof) =
            utils::groth16_verify_params_gen::<ark_bls12_381::Bls12_381>();

        #[extrinsic_call]
        bls12_381_groth16_verify(RawOrigin::None, vk.0, public_input.0, proof.0, true);
    }

    // ---------------------------------------------
    // Calls for bls12-377
    // ---------------------------------------------

    #[benchmark]
    fn ark_bls12_377_groth16_verify() {
        let (vk, public_input, proof) =
            utils::groth16_verify_params_gen::<ark_bls12_377::Bls12_377>();

        #[extrinsic_call]
        bls12_377_groth16_verify(RawOrigin::None, vk.0, public_input.0, proof.0, false);
    }

    #[benchmark]
    fn sub_bls12_377_groth16_verify() {
        let (vk, public_input, proof) =
            utils::groth16_verify_params_gen::<ark_bls12_377::Bls12_377>();

        #[extrinsic_call]
        bls12_377_groth16_verify(RawOrigin::None, vk.0, public_input.0, proof.0, true);
    }

    // ---------------------------------------------
    // Calls for bls12-377
    // ---------------------------------------------

    #[benchmark]
    fn ark_bw6_761_groth16_verify() {
        let (vk, public_input, proof) = utils::groth16_verify_params_gen::<ark_bw6_761::BW6_761>();

        #[extrinsic_call]
        bw6_761_groth16_verify(RawOrigin::None, vk.0, public_input.0, proof.0, false);
    }

    #[benchmark]
    fn sub_bw6_761_groth16_verify() {
        let (vk, public_input, proof) = utils::groth16_verify_params_gen::<ark_bw6_761::BW6_761>();

        #[extrinsic_call]
        bw6_761_groth16_verify(RawOrigin::None, vk.0, public_input.0, proof.0, true);
    }

    impl_benchmark_test_suite!(ArkGroth16, crate::mock::new_test_ext(), crate::mock::Test);
}
