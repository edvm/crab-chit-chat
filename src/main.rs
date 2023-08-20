use std::collections::HashMap;

use phf::phf_map;

const CRAB: &str = "🦀";
const CRAB_MAP: phf::Map<&'static str, u8> = phf_map! {
    "a" => 1,
    "b" => 2,
    "c" => 3,
    "d" => 4,
    "e" => 5,
    "f" => 6,
    "g" => 7,
    "h" => 8,
    "i" => 9,
    "j" => 10,
    "k" => 11,
    "l" => 12,
    "m" => 13,
    "n" => 14,
    "o" => 15,
    "p" => 16,
    "q" => 17,
    "r" => 18,
    "s" => 19,
    "t" => 20,
    "u" => 21,
    "v" => 22,
    "w" => 23,
    "x" => 24,
    "y" => 25,
    "z" => 26,
}; 

fn main() {
    let input = String::from("rust is cool !");
    println!("{}", encode(input.clone()));

    println!("Decoding");
    println!("{}", decode(input));
}


fn encode(input: String) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match CRAB_MAP.get(&c.to_string()) {
            Some(n) => {
                for _ in 0..*n {
                    output.push_str(CRAB);
                }
            }
            None => output.push_str(&c.to_string()),
        }
    }
    output
}


fn decode(input: String) -> String {

    let decode_map = invert_crab_map();
    let mut output = String::new();
    let mut crab_count = 0;

    for c in input.chars() {
        match c {
            '🦀' => crab_count += 1,
            _ => {
                match decode_map.get(&crab_count) {
                    Some(n) => output.push_str(n),
                    None => output.push_str(&c.to_string()),
                }
                crab_count = 0;
            }
        }
    }

    output

}


fn invert_crab_map() -> HashMap<u8, &'static str> {
    let mut inverted_crab_map: HashMap<u8, &str> = HashMap::new();

    for (k, v) in CRAB_MAP.entries() {
        inverted_crab_map.insert(*v, *k);
    } 

    inverted_crab_map
}