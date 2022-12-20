use std::collections::{BTreeSet, BTreeMap};
use lib_utilities::random::string;

fn main() {


    println!("random token: {}", string(10));
    println!("random id: {}", string(16));

    let key: &str = "12345Abc";
    let value: &str = "apple";
    // let keyword: &str = "pp";
    let keyword: &str = "Ab";
    let mut m: BTreeMap<String, String> = BTreeMap::new();
    m.insert(key.to_string(), value.to_string());
    m.retain(|_, v| v.contains(keyword));
    println!("m {:?}", &m);

    let mut s: BTreeSet<String> = BTreeSet::new();
    s.insert(key.to_string());
    s.retain(|v| v.contains(keyword));
    println!("s {:?}", &s);
}