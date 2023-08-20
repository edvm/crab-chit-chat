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
    println!("{}", encode(input));
}


fn encode(input: String) -> String {
    let mut output = String::new();
    for c in input.chars() {
        let c = c.to_lowercase().to_string();
        match CRAB_MAP.get(&c[..]) {
            Some(n) => {
                for _ in 0..*n {
                    output.push_str(CRAB);
                }
            }
            None => output.push_str(&c),
        }
    }
    output
}
