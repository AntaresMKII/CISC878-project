extern crate sha3;
use sha3::{Digest,Sha3_256};

pub mod util;
use crate::passport::util::*;

pub struct Passport {
    p: char,
    p_type: char,
    issuing_country: String,
    name: String,
    p_number: String,
    check_digit_1: u8,
    nat: String,
    dob: String,
    check_digit_2: u8,
    sex: char,
    expr_date: String,
    check_digit_3: u8,
    pers_num: String,
    check_digit_4: u8,
    check_digit_5: u8,
}

impl Passport {
    // creates an empty passport
    pub fn new() -> Passport {

        Passport {
            p: 'p',
            p_type: '>',
            issuing_country: String::new(),
            name: String::new(),
            p_number: String::new(),
            check_digit_1: 0,
            nat: String::new(),
            dob: String::new(),
            check_digit_2: 0,
            sex: '>',
            expr_date: String::new(),
            check_digit_3: 0,
            pers_num: String::new(),
            check_digit_4: 0,
            check_digit_5: 0,
        }
    }

    //creates a passport from provided arguments
    pub fn from_args(
        p_type: char,
        issuing_country: String,
        name: String,
        p_number: String,
        check_digit_1: u8,
        nat: String,
        dob: String,
        check_digit_2: u8,
        sex: char,
        expr_date: String,
        check_digit_3: u8,
        pers_num: String,
        check_digit_4: u8,
        check_digit_5: u8
    ) -> Passport {

        Passport {
            p: 'p',
            p_type,
            issuing_country,
            name,
            p_number,
            check_digit_1,
            nat,
            dob,
            check_digit_2,
            sex,
            expr_date,
            check_digit_3,
            pers_num,
            check_digit_4,
            check_digit_5,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut p_vec = ctob(self.p);
        p_vec.append(&mut ctob(self.p_type));
        p_vec.append(&mut stob(self.issuing_country.clone()));
        p_vec.append(&mut stob(self.name.clone()));
        p_vec.append(&mut stob(self.p_number.clone()));
        p_vec.push(self.check_digit_1);
        p_vec.append(&mut stob(self.nat.clone()));
        p_vec.append(&mut stob(self.dob.clone()));
        p_vec.push(self.check_digit_2);
        p_vec.append(&mut ctob(self.sex));
        p_vec.append(&mut stob(self.expr_date.clone()));
        p_vec.push(self.check_digit_3);
        p_vec.append(&mut stob(self.pers_num.clone()));
        p_vec.push(self.check_digit_4);
        p_vec.push(self.check_digit_5);
        p_vec
    }

    pub fn hash(&self) -> [u8;32] {
        let mut hasher = Sha3_256::new();
        let v = self.to_vec();
        let b = v.as_slice();
        hasher.update(b);

        hasher.finalize().into()
    }
}

impl ToString for Passport {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s = s + &self.issuing_country;
        s.push_str(" ");
        s = s + &self.name;
        s.push_str(" ");
        s = s + &self.p_number;
        s
    }
}
