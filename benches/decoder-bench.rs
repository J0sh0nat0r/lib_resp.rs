#![feature(test)]
extern crate lib_resp;
extern crate test;

use test::Bencher;
use std::io::BufReader;
use lib_resp::{Decoder, Value};

mod bench_decode {
    use super::*;

    #[bench]
    fn int(b: &mut Bencher) {
        let bytes = Value::int(-3).encode_bytes();

        b.iter(|| {
            let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

            decoder.decode()
        });
    }

    #[bench]
    fn str(b: &mut Bencher) {
        let bytes = Value::str("OK").encode_bytes();

        b.iter(|| {
            let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

            decoder.decode()
        });
    }

    #[bench]
    fn err(b: &mut Bencher) {
        let bytes = Value::err("ERR").encode_bytes();

        b.iter(|| {
            let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

            decoder.decode()
        });
    }

    #[bench]
    fn b_str(b: &mut Bencher) {
        let bytes = Value::b_str(Some("foobar")).encode_bytes();

        b.iter(|| {
            let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

            decoder.decode()
        });
    }

    #[bench]
    fn array(b: &mut Bencher) {
        let bytes = &Value::Array(Some(vec![
            Value::int(-3),
            Value::str("OK"),
            Value::err("ERR"),
            Value::b_str(Some("foobar")),
        ])).encode_bytes();

        b.iter(|| {
            let mut decoder = Decoder::new(BufReader::new(bytes.as_slice()));

            decoder.decode()
        });
    }
}
