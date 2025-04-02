pub fn ctob(c: char) -> Vec<u8> {
    (String::from(c)).into_bytes()
}

pub fn stob(s: String) -> Vec<u8> {
    s.into_bytes()
}

pub fn utob(i: u32) -> Vec<u8> {
    Vec::from(i.to_ne_bytes())
}
