#![allow(non_snake_case)]

use core::iter;

extern crate merlin;
use merlin::Transcript;

extern crate curve25519_dalek;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::RistrettoPoint;
use curve25519_dalek::traits::VartimeMultiscalarMul;

extern crate bulletproofs;
use bulletproofs::{LinearProof, PedersenGens};

extern crate rand;
use rand::rngs::ThreadRng;

pub mod prover;
use crate::prover::ProofEnv;

pub fn verify_proof(proof: LinearProof, env: ProofEnv, b: Vec<Scalar>) -> bool {

}
