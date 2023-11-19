use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {

    if let Some(n) = encoded_value.
        strip_prefix('i').
        and_then(|rest| rest.split_once('e')). 
        and_then(|(digits, _)| digits.parse::<i64>().ok())
    {
        return n.into()
    } else if let Some((len, rest)) = encoded_value.
        split_once(':').
        and_then(|(len, rest)| {
            let len = len.parse::<usize>().ok()?;
            Some((len, rest))
        }) {
        return rest[..len].into()
    } 
    
    panic!("unhandled encoded value {}", encoded_value)
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        eprintln!("unknown command: {}", args[1])
    }
}
