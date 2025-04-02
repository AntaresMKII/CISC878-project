pub mod passport;
use crate::passport::Passport;

pub mod prover;
use crate::prover::make_proof;
use crate::prover::verify_proof;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let mop = args[1].clone();

        if mop == "-c" {
            client();
        }
        else if mop == "-s" {
            server();
        }
        else if mop == "-h" {
            help();
        }
        else {
            println!("Argument {} not recognized. Use -h for usage.", mop);
        }
    }
    else {
        println!("Program must have at least 1 argument!");
    }
}

fn client() {
    let p = Passport::from_args(
        '>',
        String::from("ITA"),
        String::from("Yannick Abouem"),
        String::from("YB0987655"),
        4,
        String::from("ITA"),
        String::from("010112"),
        5,
        'M',
        String::from("211204"),
        6,
        String::new(),
        0,
        6
    );
    let h = p.hash();
    let (proof, env, b) = make_proof(h);
}

fn server() {
    //let res = verify_proof(proof, env, b);
}

fn help() {
    println!("Usage: ");
    println!("-c\tRun in client mode");
    println!("-s\tRun in server mode");
    println!("-h\tPrint this help message");
}
