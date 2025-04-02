pub mod passport;
use crate::passport::Passport;

pub mod prover;
use crate::prover::make_proof;
use crate::prover::verify_proof;

fn main() {
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

    println!("{}", p.to_string());

    let arr = p.hash();
    println!("{}",arr[0]);

    let (proof, env, b) = make_proof(arr);
    println!("Proof created");

    println!("Verifying proof");
    let res = verify_proof(proof, env, b);

    println!("{}", res);
}
