use std::fs::File;
use std::io::{BufReader, Read};
use serde_json::{Value, Map};
use std::collections::BTreeMap;

struct Cheat {
    name: String,
    hash: u32,
}

fn parse_hash(value: &String) -> u32 {
    let without_prefix = value.trim_start_matches("0x");
    u32::from_str_radix(without_prefix, 16).unwrap()
}

fn parse_cheat(cheat: &Map<String, Value>) -> Result<Cheat, &'static str> {
    if let Option::Some(Value::String(name)) = cheat.get("name") {
        if let Option::Some(Value::String(hash_str)) = cheat.get("hash") {
            return Result::Ok(
                Cheat {
                    name: name.to_string(),
                    hash: parse_hash(hash_str),
                }
            );
        }
    }

    Result::Err("Can't parse cheat object!")
}

fn parse_cheats<R>(reader: R) -> Result<BTreeMap<u32, String>, &'static str>
    where R: Read
{
    let root: Value = serde_json::from_reader(reader).unwrap();

    match root {
        Value::Array(values) => {
            let mut map: BTreeMap<u32, String> = BTreeMap::new();

            for v in values {
                if let Value::Object(cheat_map) = v {
                    let cheat = parse_cheat(&cheat_map).unwrap();
                    map.insert(cheat.hash, cheat.name);
                } else {
                    return Result::Err("Not an object!");
                }
            }

            Result::Ok(map)
        }

        _ => Result::Err("Not an array")
    }
}

fn read_file(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    contents
}


fn main() {
    println!("Hello, world!");

    let br = BufReader::new(File::open("cheats.json").unwrap());

    let cheats = parse_cheats(br).unwrap();
    for hash in cheats.keys() {
        println!("Cheat: name: {0}, hash: {1}", cheats[hash], hash);
    }
}
