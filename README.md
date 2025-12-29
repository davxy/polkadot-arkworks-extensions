# Polkadot Arkworks Hostcalls

Integration of [Arkworks](https://github.com/arkworks-rs) cryptographic primitives as
Substrate host functions, demonstrating efficient elliptic curve operations within the
Polkadot/Substrate ecosystem.

## Overview

This project serves two primary purposes:

1. **On-chain Arkworks Demonstration**: Shows practical integration of cryptographic
   operations from the [Arkworks](https://github.com/arkworks-rs) ecosystem into Substrate
   based runtimes through custom pallets and host functions.

2. **Arkworks-extensions Backend**: Demonstrates how Polkadot SDK host calls can serve as
   the computational backend for [arkworks-extensions](https://github.com/paritytech/arkworks-extensions/),
   offloading expensive elliptic curve operations from Wasm to native host execution for
   significant performance improvements.

## Features

### Pallet Ark Hostcalls (`pallet-ark-hostcalls`)

Provides cryptographic operations on the Ed-on-BLS12-381-Bandersnatch curve with two
curve representations:

**Short Weierstrass (SW) Curve Operations:**
- `ed_on_bls12_381_bandersnatch_msm_sw` - Multi-Scalar Multiplication
- `ed_on_bls12_381_bandersnatch_mul_projective_sw` - Projective scalar multiplication
- `ed_on_bls12_381_bandersnatch_mul_affine_sw` - Affine scalar multiplication

**Twisted Edwards (TE) Curve Operations:**
- `ed_on_bls12_381_bandersnatch_msm_te` - Multi-Scalar Multiplication
- `ed_on_bls12_381_bandersnatch_mul_projective_te` - Projective scalar multiplication
- `ed_on_bls12_381_bandersnatch_mul_affine_te` - Affine scalar multiplication

Each operation supports both native Arkworks and Substrate-optimized implementations
via the `optimized` parameter.

### Pallet Ark VRF (`pallet-ark-vrf`)

Implements Verifiable Random Function verification operations using the Bandersnatch suite:

**IETF VRF:**
- `ietf_verify` - Verify RFC-compliant VRF proof

**Ring-based VRF:**
- `ring_reset` - Initialize ring verifier key builder
- `push_members` / `push_member_buffered` - Add ring members
- `ring_commit` - Finalize ring and build verifier key
- `ring_verify` - Verify ring-based VRF proof

Configuration options:
- Configurable ring sizes (2^11 with `small-ring` feature; 2^16 default)
- Pregenerated Universal Reference String (URS) (from zcash ceremony)

## Arkworks-Extensions Integration

This project demonstrates how Polkadot SDK host calls can act as a performance-critical
backend for the [arkworks-extensions](https://github.com/paritytech/arkworks-extensions/)
library. The integration works as follows:

- **Wasm Runtime**: Runtime code uses arkworks-extensions API for elliptic curve operations
- **Host Call Backend**: Expensive operations are dispatched to native host functions
  (`sp-crypto-ec` polkadot-sdk crate)
- **Performance Gain**: Native execution avoids Wasm overhead for computationally intensive
  cryptographic primitives (MSM, scalar multiplication, etc.)

## Project Structure

```
polkadot-ark-hostcalls/
├── pallets/
│   ├── hostcalls/          # Arkworks cryptographic hostcalls pallet
│   └── vrf/                # VRF verification pallet
├── runtime/                # Substrate based runtime
├── node/                   # Substrate based node
├── benchmark.sh            # Benchmark execution script
```

## Running

### Start the Node

```bash
cargo run --release -- --dev
```

### Development Mode

```bash
cargo run -- --dev --tmp
```

## Benchmarking

The project includes comprehensive benchmarking infrastructure

Examples:

```bash
# Benchmark all extrinsics from ark_hostcalls pallet
./benchmark.sh pallet_ark_hostcalls all

# Benchmark substrate based (optimized) VRF verification from ark_vrf pallet
./benchmark.sh pallet_ark_vrf sub_ring_vrf_verify

# Benchmark all extrinsics from ark_vrf pallet
./benchmark.sh pallet_ark_vrf all
```

Benchmark results are shown in the terminal and stored in the `results/` directory.

## Feature Flags

- `std` - Standard library support
- `runtime-benchmarks` - Enable benchmark implementations
- `small-ring` - Reduced ring sizes for testing (2^11 instead of 2^16)

## Repository

GitHub: [davxy/polkadot-arkworks-extensions](https://github.com/davxy/polkadot-arkworks-extensions)
