//! A simple script to generate and verify the proof of a given program.

use std::{fs::File, io::{BufReader, Read, Write}};

use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 2u64;

    let file = File::open("/Users/swopnilparajuli/playground/lambdaworks/provers/plonk/src/proof.proof").unwrap();
    let mut buffer = BufReader::new(file);
    let mut proof_read: Vec<u8>= Vec::new();

    buffer.read_to_end(&mut proof_read).unwrap();

    println!("{:?}", proof_read);

    stdin.write(&n);
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove_compressed(&pk, stdin).expect("proving failed");

    // Read output.
    let a = proof.public_values.read::<u128>();
    let b = proof.public_values.read::<u128>();
    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof.
    client
        .verify_compressed(&proof, &vk)
        .expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!");

    let mut file = File::create("done.verifying").unwrap();
    let buf_file = "successfully generated and verified proof for the program!";

    file.write_all(buf_file.as_bytes()).unwrap();
}
