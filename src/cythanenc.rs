fn compress(v: &Vec<u32>) -> Vec<u8> {
    v.iter()
        .map(|x| x.to_be_bytes())
        .fold(Vec::with_capacity(v.len() * 4), |mut a, b| {
            a.extend(b.iter());
            a
        })
}

fn decompress(v: &Vec<u8>) -> Vec<u32> {
    (0..(v.len() / 4))
        .map(|i| {
            let i = i * 4;
            u32::from_be_bytes([v[i], v[i + 1], v[i + 2], v[i + 3]])
        })
        .collect()
}

#[test]
fn check_encode_validity() {
    let v: Vec<u32> = vec![
        12, 58, 23, 0, 2, 14, 25, 78, 52, 63, 60456, 4894, 91509, 4509090, 848554965, 645569511,
        489488, 845546154,
    ];
    let o1 = compress(&v);
    let o2 = decompress(&o1);
    assert_eq!(v, o2);
}

#[test]
fn check_file_encode_decode() {
    let v: Vec<u32> = (0..100_000).collect();

    std::fs::write(
        "raw.ctt",
        v.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" "),
    );

    write_file_and_compress("compressed.cct", &v);
    let p = read_file_and_decompress("compressed.cct");
    assert_eq!(v, p.unwrap());
}

use std::fs::File;
use std::io::Read;
use std::io::Write;

use compression::prelude::*;

pub fn read_file_and_decompress(filename: &str) -> anyhow::Result<Vec<u32>> {
    if filename.ends_with(".cct") {
        let mut f = File::open(&filename)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        Ok(decompress(
            &buffer
                .into_iter()
                .decode(&mut BZip2Decoder::new())
                .collect::<Result<Vec<u8>, _>>()?,
        ))
    } else {
        Ok(std::fs::read_to_string(filename)?
            .split_whitespace()
            .flat_map(|x| x.parse::<u32>())
            .collect())
    }
}

pub fn write_file_and_compress(filename: &str, data: &Vec<u32>) -> anyhow::Result<()> {
    if filename.ends_with(".cct") {
        let mut f = File::create(filename)?;
        f.write_all(
            &compress(data)
                .into_iter()
                .encode(&mut BZip2Encoder::new(9), Action::Finish)
                .collect::<Result<Vec<u8>, _>>()?,
        )?;
        Ok(())
    } else {
        std::fs::write(
            filename,
            data.iter()
                .map(|x| x.to_string())
                .fold(String::new(), |a, b| a + &b),
        )?;
        Ok(())
    }
}
