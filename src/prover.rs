#![allow(non_snake_case)]

use core::iter;

extern crate merlin;
use merlin::Transcript;

extern crate curve25519_dalek;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::RistrettoPoint;
use curve25519_dalek::traits::VartimeMultiscalarMul;
use curve25519_dalek::ristretto::CompressedRistretto;

extern crate bulletproofs;
use bulletproofs::{LinearProof, PedersenGens, BulletproofGens};

extern crate rand;
use rand::rngs::ThreadRng;

pub struct ProofEnv {
    C: CompressedRistretto,
    G: Vec<RistrettoPoint>,
    F: RistrettoPoint,
    B: RistrettoPoint,
}

//pub struct ProofEnvSerialized {
//    C_bytes: [u8; 32],
//    G_bytes: Vec<[u8; 32]>,
//    F_bytes: [u8; 32],
//    B_bytes: [u8; 32],
//}

//impl ProofEnv {
//    pub fn serialize(&self) -> ProofEnvSerialized {
//        ProofEnvSerialized {
//            C_bytes = self.C.to_bytes(),
//            G_bytes = self.G.map(|point| point.to_bytes()).collect(),
//            F_bytes = self.F.compress().to_bytes(),
//            B_bytes = self.B.compress().to_bytes()
//        }
//    }
//}


pub fn make_proof(priv_vec: [u8;32]) -> (LinearProof, ProofEnv, Vec<Scalar>) {
    let mut rng = ThreadRng::default();
    let bp_gens = BulletproofGens::new(32,1);

    let G: Vec<RistrettoPoint> = bp_gens.share(0).G(32).cloned().collect();

    let p_gen = PedersenGens::default();
    let F = p_gen.B;
    let B = p_gen.B_blinding;

    let b: Vec<_> = (0..32).map(|_| Scalar::random(&mut rng)).collect();
    let a: Vec<_> = (0..32).map(|_| Scalar::from_bytes_mod_order(priv_vec)).collect();

    let mut transcript = Transcript::new(b"Passport Prover");

    let r = Scalar::random(&mut rng);
    let c = inner_product(&a, &b);
    let C = RistrettoPoint::vartime_multiscalar_mul(
        a.iter().chain(iter::once(&r)).chain(iter::once(&c)),
        G.iter().chain(iter::once(&B)).chain(iter::once(&F)),
    ).compress();


    let res = LinearProof::create(
&mut transcript,
        &mut rng,
        &C,
        r,
        a.clone(),
        b.clone(),
        G.clone(),
        &F,
        &B,
    );

    let proof = res.expect("Failed to create proof");
    let env = ProofEnv {C: C.clone(), G: G.clone(), F: F.clone(), B: B.clone()};
    (proof, env, b)
}

pub fn verify_proof(proof: LinearProof, env: ProofEnv, b: Vec<Scalar>) -> bool {
    let mut tran = Transcript::new(b"Passport Prover");
    let _res = proof.verify(&mut tran, &env.C, &env.G, &env.F, &env.B, b.clone()).unwrap();
    true
}

fn inner_product(a: &[Scalar], b: &[Scalar]) -> Scalar {
    let mut out = Scalar::ZERO;
    if a.len() != b.len() {
        panic!("Inner product error!");
    }
    for i in 0..a.len() {
        out += a[i] * b[i];
    }
    out
}
