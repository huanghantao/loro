use std::{io::Read, time::Instant};

use flate2::read::GzDecoder;
use loro_core::{configure::Configure, container::registry::ContainerWrapper, LoroCore};
use serde_json::Value;
const RAW_DATA: &[u8; 901823] = include_bytes!("../benches/automerge-paper.json.gz");

fn main() {
    let mut d = GzDecoder::new(&RAW_DATA[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    let json: Value = serde_json::from_str(&s).unwrap();
    let txns = json.as_object().unwrap().get("txns");
    let mut loro = LoroCore::default();
    let text = loro.get_text("text");
    text.with_container(|text| {
        for txn in txns.unwrap().as_array().unwrap() {
            let patches = txn
                .as_object()
                .unwrap()
                .get("patches")
                .unwrap()
                .as_array()
                .unwrap();
            for patch in patches {
                let pos = patch[0].as_u64().unwrap() as usize;
                let del_here = patch[1].as_u64().unwrap() as usize;
                let ins_content = patch[2].as_str().unwrap();
                text.delete(&loro, pos, del_here);
                text.insert(&loro, pos, ins_content);
            }
        }
    });
    let start = Instant::now();
    let buf = loro.encode_snapshot();
    println!(
        "{} bytes, overhead {} bytes. used {}ms",
        buf.len(),
        0,
        start.elapsed().as_millis()
    );
    let start = Instant::now();
    let loro = LoroCore::decode_snapshot(&buf, None, Configure::default());
    println!("decode used {}ms", start.elapsed().as_millis());
    let buf2 = loro.encode_snapshot();
    assert_eq!(buf, buf2);
    let mut last = 100;
    let mut count = 0;
    let mut max_count = 0;
    for &byte in buf.iter() {
        if byte == last {
            count += 1;
            if count > max_count {
                max_count = count;
            }
        } else {
            count = 0;
        }
        last = byte;
    }

    println!("Longest continuous bytes length {}", max_count);
}