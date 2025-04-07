use std::env;
use std::net::TcpListener;

pub mod passport;
use crate::passport::Passport;

pub mod prover;
use crate::prover::make_proof;
use crate::prover::verify_proof;

fn main() {
    println!("Example of a successful verification");
    successfull();
    println!("Example of an unsuccessful verification");
    unsuccessfull();
}

fn successfull() {
    let p = Passport::from_args(
        '>',
        String::from("CAN"),
        String::from("Jhon Doe"),
        String::from("AB0000000"),
        4,
        String::from("CAN"),
        String::from("010112"),
        5,
        'M',
        String::from("211204"),
        6,
        String::new(),
        0,
        6
    );
    println!("Passport to verify: ");
    println!("{}", p.to_string());
    let h = p.hash();
    let (proof, env, b) = make_proof(h);
    let res = verify_proof(proof, env, b);
    // if the proof is verified then print the result (true)
    println!("Verification result: {}", res);
}

fn unsuccessfull() {
    let p1 = Passport::from_args(
        '>',
        String::from("CAN"),
        String::from("Jhon Doe"),
        String::from("AB0000000"),
        4,
        String::from("CAN"),
        String::from("010112"),
        5,
        'M',
        String::from("211204"),
        6,
        String::new(),
        0,
        6
    );

    let p2 = Passport::from_args(
        '>',
        String::from("CAN"),
        String::from("Jane Doe"),
        String::from("AB1234567"),
        4,
        String::from("CAN"),
        String::from("010112"),
        5,
        'F',
        String::from("211204"),
        6,
        String::new(),
        0,
        6
    );
    println!("Passport to verify: ");
    println!("{}", p1.to_string());
    println!("Passport that will be used to create a proof to inject: ");
    println!("{}", p2.to_string());
    let h1 = p1.hash();
    let h2 = p2.hash();
    let (_proof1, env1, b1) = make_proof(h1);
    let (proof2, _env2, _b2) = make_proof(h2);
    println!("The program should return an error");
    let res = verify_proof(proof2, env1, b1);
    // if the proof is verified then print the result (true)
    println!("Verification result: {}", res);
}
