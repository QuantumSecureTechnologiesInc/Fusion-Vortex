# Post‑Quantum Primitives

HyperCycle includes three production‑grade post‑quantum cryptographic schemes that build upon the quaternion chaos map.  They offer alternatives to lattice‑based algorithms like Falcon, HQC and SPHINCS+ while maintaining strong security against quantum adversaries.  All functions are implemented header‑only in `hc_pqc_boost.h` for ease of integration.

## FastSign‑Q (Signature)

FastSign is a variant of the Winternitz one‑time signature scheme adapted to quaternions.  A private key consists of 32 seeds.  Each seed defines a chain of rotations; the public key stores the top of each chain.  To sign a digest the signer reads 4‑bit nibbles of the message, evolves each seed by the nibble count and outputs the resulting quaternions.  Verification re‑evolves the signature elements for the remaining steps and compares them with the public key.

This scheme is compact and fast, making it suitable for one‑time authentication tokens.  For multiple uses you can combine FastSign with a Merkle tree (see HashSign below).

## ChaosCode‑Q (KEM)

ChaosCode is a key encapsulation mechanism similar in spirit to HQC.  To encapsulate a secret the sender chooses an ephemerally random quaternion `r` and computes a masking rotation `M` (a system parameter).  The sender multiplies `M` with the recipient’s public quaternion to form a combined rotation and rotates `r` by this product.  The result is absorbed into the sponge to derive a 256‑bit shared secret.  The ciphertext contains `r` and `M` (or a compressed representation thereof).

Decapsulation performs the same operations using the recipient’s view of `M` and `r`.  The security stems from the difficulty of recovering `r` from the rotated quaternion without knowledge of the private quaternion and the hardness of the conjugacy search problem in the quaternion group.

## HashSign‑Q (Hash‑Based Signature)

HashSign demonstrates how to build a simple Merkle‑tree signature from the chaos sponge.  Given an array of leaf quaternions (each representing a public key or message fragment), the signer absorbs them into the sponge to compute the tree root.  Signing a message involves computing a one‑time seed from the message and absorbing it with the root; the result serves as a signature.  Verification recomputes the root and the one‑time seed and compares the outputs.

In a production implementation you would include authentication paths and a robust addressing scheme; the version provided here focuses on clarity.

## Usage Notes

* All PQC functions return `HC_PQC_SUCCESS` on success or an error code such as `HC_PQC_ERR_VERIFY_FAILED` or `HC_PQC_ERR_INVALID_LEN` when something goes wrong.
* The buffer sizes are derived from the size of `hc_quat_t` (32 bytes); be sure to allocate sufficient space when storing signatures or ciphertexts.
* None of the schemes consume or produce persistent state.  You can generate keys and signatures repeatedly without worrying about leaking private information.


