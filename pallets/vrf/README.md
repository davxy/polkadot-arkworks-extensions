## Ring VRF Verification

### Pure WASM

Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=    29950
    + x        0
              µs

Reads = 2 + (0 * x)
Writes = 0 + (0 * x)
Recorded proof Size = 465 + (0 * x)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    x   mean µs  sigma µs       %
    1     30130     659.4    2.1%
    2     30280     939.3    3.1%
    3     30310     953.5    3.1%
    4     30300     957.1    3.1%
    5     30290     905.7    2.9%
    6     30020     660.9    2.2%
    7     30780      1353    4.3%
    8     29930       672    2.2%
    9     30940      1291    4.1%
   10     29820     20.61    0.0%

Quality and confidence:
param     error
x         16.98

Model:
Time ~=    30130
    + x    38.19
              µs

Reads = 2 + (0 * x)
Writes = 0 + (0 * x)
Recorded proof Size = 465 + (0 * x)


### Substrate Hostcalls

Median Slopes Analysis
========
2025-12-13 17:41:31 [  0 % ] Running  benchmark: pallet_ark_vrf::sub_ring_vrf_verify(1 args) 50/50 1/1
-- Extrinsic Time --

Model:
Time ~=    17210
    + x        0
              µs

Reads = 2 + (0 * x)
Writes = 0 + (0 * x)
Recorded proof Size = 465 + (0 * x)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    x   mean µs  sigma µs       %
    1     17150     10.16    0.0%
    2     17180     11.71    0.0%
    3     17220     8.858    0.0%
    4     17270     9.337    0.0%
    5     17110     10.51    0.0%
    6     17070     11.04    0.0%
    7     17000     7.905    0.0%
    8     17060     10.27    0.0%
    9     17340     431.8    2.4%
   10     16870     34.89    0.2%

Quality and confidence:
param     error
x         2.936

Model:
Time ~=    17190
    + x        0
              µs

Reads = 2 + (0 * x)
Writes = 0 + (0 * x)
Recorded proof Size = 465 + (0 * x)

## IETF VRF Verification

### Pure WASM

### Substrate Hostcalls
