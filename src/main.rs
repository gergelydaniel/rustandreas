use std::fs::File;
use std::io::{BufReader, Read};
use serde_json::{Value, Map};
use std::collections::BTreeMap;
use rayon::prelude::IntoParallelRefIterator;

struct Cheat {
    name: String,
    hash: u32,
}

struct CheatMatch {
    hash: u32,
    value: String,
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


static MIN_LENGTH: usize = 6;
static MAX_LENGTH: usize = 29;

fn crc32_compute_table() -> [u32; 256] {
    let mut crc32_table = [0; 256];

    for n in 0..256 {
        crc32_table[n as usize] = (0..8).fold(n as u32, |acc, _| {
            match acc & 1 {
                1 => 0xedb88320 ^ (acc >> 1),
                _ => acc >> 1,
            }
        });
    }

    crc32_table
}

fn crc32(crc32_table: &[u32; 256], buf: &str) -> u32 {
    buf.bytes().fold(!0, |acc, octet| {
        (acc >> 8) ^ crc32_table[((acc & 0xff) ^ octet as u32) as usize]
    })
}

//fn find_matches(start_char: char, cheats: BTreeMap<u32, String>) -> Vec<CheatMatch> {
//
//}

fn main() {
    println!("Hello, world!");

    let br = BufReader::new(File::open("cheats.json").unwrap());

    let cheats = parse_cheats(br).unwrap();
    for hash in cheats.keys() {
        println!("Cheat: name: {0}, hash: {1}", cheats[hash], hash);
    }

    let crc32_table = crc32_compute_table();

    let test_str = "TOHNMADOOT";
    let test_hash = crc32(&crc32_table, test_str);
    println!("Hash of \"{0}\" is {1}", test_str, format!("{:#X}", test_hash));

    //let results: Vec<Vec<CheatMatch>> =
    //    CHARS
    //        .par_iter()
    //        .map(|c| find_matches(c, cheats))
    //        .collect();
}
