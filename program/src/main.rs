//! A simple program to be proven inside the zkVM.

#![no_main]

use lambdaworks_math::field::element::FieldElement;
use lambdaworks_plonk::{prover::Prover, setup::setup, test_utils::{circuit_1::{test_common_preprocessed_input_1, test_witness_1}, utils::{test_srs, TestRandomFieldGenerator, KZG}}, verifier::Verifier};
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!

    let n = sp1_zkvm::io::read::<u64>();

    let common_preprocessed_input = test_common_preprocessed_input_1();
    let srs = test_srs(common_preprocessed_input.n);

    // public input 
    let x = FieldElement::from(n);
    let y = FieldElement::from(12_u64);

    // private variable 
    let e = FieldElement::from(3_u64);

    let public_input = vec![x.clone(), y];
    let witness = test_witness_1(x, e);

    let kzg = KZG::new(srs);
    let verifying_key = setup(&common_preprocessed_input, &kzg);
    let random_generator = TestRandomFieldGenerator {};

    let prover = Prover::new(kzg.clone(), random_generator);
    let proof = prover.prove(&witness, &public_input, &common_preprocessed_input, &verifying_key);
    let verifier = Verifier::new(kzg);
    assert!(verifier.verify(&proof, &public_input, &common_preprocessed_input, &verifying_key));

    sp1_zkvm::io::commit(&n);
}
