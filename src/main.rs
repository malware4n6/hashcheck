mod utils;
use log::{debug, error, info, trace, LevelFilter};
use md5::Md5;
use sha2::{Digest, Sha256, Sha512};
use std::env;
use std::fs::File;
use std::io::Seek;
use std::process::exit;

use std::io;

fn get_hashes(filename: &String) -> Option<()> {
    debug!("Have to consider {}", filename);
    let f = File::open(filename);
    match f {
        Ok(mut fd) => {
            let mut h_md5 = Md5::new();
            let mut h_sha256 = Sha256::new();
            let mut h_sha512 = Sha512::new();

            io::copy(&mut fd, &mut h_sha512).ok()?;
            let _ = fd.rewind();
            let hash = h_sha512.finalize();
            let s_hash = format!("{:x}", hash);
            if filename.contains(&s_hash) {
                info!("OK - SHA512\t{}", filename);
                return None;
            }

            io::copy(&mut fd, &mut h_md5).ok()?;
            let _ = fd.rewind();
            let hash = h_md5.finalize();
            let s_hash = format!("{:x}", hash);
            if filename.contains(&s_hash) {
                info!("OK - MD5   \t{}", filename);
                return None;
            }

            io::copy(&mut fd, &mut h_sha256).ok()?;
            let _ = fd.rewind();
            let hash = h_sha256.finalize();
            let s_hash = format!("{:x}", hash);
            if filename.contains(&s_hash) {
                info!("OK - SHA256\t{}", filename);
                return None;
            }

            error!("FAIL\t{}", filename);
        }
        Err(_) => {
            error!("Cannot read {}", filename);
        }
    }
    None
}

fn main() {
    utils::init_logger(LevelFilter::Info); // Trace
    trace!("Starting hashcheck");
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args.iter() {
        trace!("- arg \"{}\"", arg);
    }

    match args.len() {
        0 => {
            error!("You need to specify at least one file.");
            error!("Use env var RUST_LOG=trace|debug|info if needed (default is error).");
            exit(1);
        }
        _ => {
            debug!("Considering {} file(s)", args.len());
        }
    }
    for filename in args.iter() {
        get_hashes(filename);
    }
}
