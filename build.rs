use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

fn main() {
    #[cfg(feature = "compact")]
    let compact_regions = [
        #[cfg(feature = "AS923_1")]
        ("AS923_1", "src/compact/AS923-1.res7.h3idx"),
        #[cfg(feature = "AS923_1B")]
        ("AS923_1B", "src/compact/AS923-1B.res7.h3idx"),
        #[cfg(feature = "AS923_2")]
        ("AS923_2", "src/compact/AS923-2.res7.h3idx"),
        #[cfg(feature = "AS923_3")]
        ("AS923_3", "src/compact/AS923-3.res7.h3idx"),
        #[cfg(feature = "AS923_4")]
        ("AS923_4", "src/compact/AS923-4.res7.h3idx"),
        #[cfg(feature = "AU915")]
        ("AU915", "src/compact/AU915.res7.h3idx"),
        #[cfg(feature = "CN470")]
        ("CN470", "src/compact/CN470.res7.h3idx"),
        #[cfg(feature = "EU433")]
        ("EU433", "src/compact/EU433.res7.h3idx"),
        #[cfg(feature = "EU868")]
        ("EU868", "src/compact/EU868.res7.h3idx"),
        #[cfg(feature = "IN865")]
        ("IN865", "src/compact/IN865.res7.h3idx"),
        #[cfg(feature = "KR920")]
        ("KR920", "src/compact/KR920.res7.h3idx"),
        #[cfg(feature = "RU864")]
        ("RU864", "src/compact/RU864.res7.h3idx"),
        #[cfg(feature = "US915")]
        ("US915", "src/compact/US915.res7.h3idx"),
    ];

    #[cfg(feature = "nocompact")]
    let nocompact_regions = [
        #[cfg(feature = "AS923_1")]
        ("AS923_1", "src/nocompact/AS923-1.res7.h3idx"),
        #[cfg(feature = "AS923_1B")]
        ("AS923_1B", "src/nocompact/AS923-1B.res7.h3idx"),
        #[cfg(feature = "AS923_2")]
        ("AS923_2", "src/nocompact/AS923-2.res7.h3idx"),
        #[cfg(feature = "AS923_3")]
        ("AS923_3", "src/nocompact/AS923-3.res7.h3idx"),
        #[cfg(feature = "AS923_4")]
        ("AS923_4", "src/nocompact/AS923-4.res7.h3idx"),
        #[cfg(feature = "AU915")]
        ("AU915", "src/nocompact/AU915.res7.h3idx"),
        #[cfg(feature = "CN470")]
        ("CN470", "src/nocompact/CN470.res7.h3idx"),
        #[cfg(feature = "EU433")]
        ("EU433", "src/nocompact/EU433.res7.h3idx"),
        #[cfg(feature = "EU868")]
        ("EU868", "src/nocompact/EU868.res7.h3idx"),
        #[cfg(feature = "IN865")]
        ("IN865", "src/nocompact/IN865.res7.h3idx"),
        #[cfg(feature = "KR920")]
        ("KR920", "src/nocompact/KR920.res7.h3idx"),
        #[cfg(feature = "RU864")]
        ("RU864", "src/nocompact/RU864.res7.h3idx"),
        #[cfg(feature = "US915")]
        ("US915", "src/nocompact/US915.res7.h3idx"),
    ];

    #[cfg(feature = "compact")]
    {
        let out_path: PathBuf = [std::env::var("OUT_DIR").unwrap().as_str(), "compact.rs"]
            .iter()
            .collect();
        let mut out = BufWriter::new(File::create(out_path).unwrap());

        gen_region_header(&compact_regions, &mut out);

        for (region_name, h3_file_path) in compact_regions.iter() {
            let h3_file_path = Path::new(h3_file_path).canonicalize().unwrap();
            println!("cargo:rerun-if-changed={}", h3_file_path.to_str().unwrap());
            let h3_file = BufReader::new(File::open(h3_file_path).unwrap());
            gen_region_array(region_name, h3_file, &mut out);
        }
    }

    #[cfg(feature = "nocompact")]
    {
        let out_path: PathBuf = [std::env::var("OUT_DIR").unwrap().as_str(), "nocompact.rs"]
            .iter()
            .collect();
        let mut out = BufWriter::new(File::create(out_path).unwrap());

        gen_region_header(&nocompact_regions, &mut out);

        for (region_name, h3_file_path) in nocompact_regions.iter() {
            let h3_file_path = Path::new(h3_file_path).canonicalize().unwrap();
            println!("cargo:rerun-if-changed={}", h3_file_path.to_str().unwrap());
            let h3_file = BufReader::new(File::open(h3_file_path).unwrap());
            gen_region_array(region_name, h3_file, &mut out);
        }
    }
}

fn gen_region_header(regions: &[(&str, &str)], out: &mut BufWriter<File>) {
    writeln!(out, "pub static REGIONS: &[(&str, &[u64])] = &[").unwrap();
    for (region_name, _) in regions {
        writeln!(out, "    (\"{}\", {}),", region_name, region_name).unwrap()
    }
    writeln!(out, "];").unwrap();
}

fn gen_region_array(region_name: &str, mut h3_indices: BufReader<File>, out: &mut BufWriter<File>) {
    writeln!(out, "pub static {}: &[u64] = &[", region_name).unwrap();
    while let Ok(h3_index) = h3_indices.read_u64::<LE>() {
        writeln!(out, "    {:#016x},", h3_index).unwrap()
    }
    writeln!(out, "];").unwrap();
}
