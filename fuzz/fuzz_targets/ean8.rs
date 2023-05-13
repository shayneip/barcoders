#![no_main]
use libfuzzer_sys::fuzz_target;
use barcoders::sym::ean8::EAN8;

fuzz_target!(|input: ([u8; 8], bool)| {
    let mut data: Vec<_> = input.0.iter().map(|x| (x % 10).to_string()).collect();
    if input.1 {
        data = data[..7].to_vec();
    }
    let data = data.join("");
    let _ = EAN8::new(data).unwrap();
});