use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn u16() -> u16 {
    thread_rng().gen::<u16>()
}

pub fn isize() -> isize {
    rand::random::<isize>()
}
