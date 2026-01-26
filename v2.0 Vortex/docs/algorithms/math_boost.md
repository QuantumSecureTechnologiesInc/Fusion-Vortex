# Math Boost

The **Math Boost layer** provides higher‑level operations built atop the fixed‑point quaternion core.  These functions are available on both the host and device and form the building blocks for the post‑quantum schemes in HyperCycle.

## Quaternion Arithmetic

Quaternion operations are used extensively to mix and rotate state.  The library implements four basic primitives:

| Function | Description |
| --- | --- |
| `hc_quat_add(a,b,r)` | Compute \(r = a + b\) component‑wise.  This operation is naturally vectorisable. |
| `hc_quat_sub(a,b,r)` | Compute \(r = a − b\). |
| `hc_quat_mul(a,b,r)` | Compute the Hamilton product of `a` and `b`.  Intermediate products use 64‑bit accumulators and a single division by `HC_SCALE` at the end to maintain precision. |
| `hc_quat_conjugate(a,r)` | Compute the conjugate of `a`: \(w → w, x → −x, y → −y, z → −z\). |

These primitives avoid branching and allocate no dynamic memory, making them suitable for high‑throughput GPU execution.

## Rotation Composition

Sequential quaternion rotations can be merged into a single rotation using the Hamilton product.  The function `hc_quat_compose_rotations(second_rot, first_rot, combined)` computes \(combined = second\_rot × first\_rot\), meaning that applying `combined` to a vector has the same effect as applying `first_rot` followed by `second_rot`.  Pre‑computing combined rotations reduces the number of Hamilton products required when applying multiple rotations to many vectors (see the KEM implementation).

## Sandwich Rotation

To rotate a vector represented as a quaternion \(v = (0,x,y,z)\) by a unit quaternion `q` you compute \(q · v · q^∗\).  The `hc_quat_rotate()` helper performs exactly this and avoids the gimbal lock problems associated with Euler angles.  Because all four components are updated via Hamilton products, the rotation is smooth across the 4‑D sphere.

## Sponge Construction

Beyond rotations, the Math Boost layer implements a simple sponge based on the chaos map.  The sponge maintains two quaternions: a rate `q_rate` and a capacity `q_capacity`.  To *absorb* data the caller XORs a block of bytes into `q_rate` and then permutes the state.  To *squeeze* output the sponge is permuted again and bytes are read out of `q_rate`.

The permutation function `hc_sponge_permute()` performs cross‑coupling between `q_rate` and `q_capacity` and calls the chaos map on both states.  An additional addition every seventh iteration breaks residual symmetries.  The sponge is used in the ChaosCode KEM to derive shared secrets from secret seeds.

## Header‑Only Implementation

All functions in the Math Boost layer are defined `static inline` or `__device__` so that they can be inlined into host code or device kernels without incurring call overhead.  There is no associated source file or global state.  This design promotes portability and simplifies auditing.


