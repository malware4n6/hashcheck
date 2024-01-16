mod utils;
use log::{debug, info, error, trace, LevelFilter};
use std::env;
use std::fs::File;
use std::io;
use std::process::exit;
use md5::digest::FixedOutputReset;
use md5::Md5;
use sha2::{Digest, Sha256, Sha512};
use std::io::Write;

#[derive(Debug)]
struct HashChecker {
    int_md5: Md5,
    int_sha256: Sha256,
    int_sha512: Sha512,
    md5: String,
    sha256: String,
    sha512: String,
}

impl HashChecker {
    /// generate a new HashChecker
    fn new() -> Self {
        Self {
            int_md5: Md5::new(),
            int_sha256: Sha256::new(),
            int_sha512: Sha512::new(),
            md5: String::from(""),
            sha256: String::from(""),
            sha512: String::from(""),
        }
    }

    /// show MD5, SHA256 and SHA512 in debug mode
    fn show(&self) {
        debug!("{:#?}", &self);
    }

    /// finalize the internal hashes and generate the strings accordingly
    fn finalize(&mut self) {
        let h1 = self.int_md5.finalize_fixed_reset();
        self.md5 = format!("{:x}", h1);
        let h2 = self.int_sha256.finalize_fixed_reset();
        self.sha256 = format!("{:x}", h2);
        let h3 = self.int_sha512.finalize_fixed_reset();
        self.sha512 = format!("{:x}", h3);
    }

    /// check that a filename contains the hash of said file.
    /// no result; a single log is shown (info! if OK / error! in other cases)
    fn check(&self, filename: &String) {
        let name = filename.to_lowercase();
        if name.contains(&self.md5) || name.contains(&self.sha256) || name.contains(&self.sha512) {
            info!("OK\t{}", filename);
        } else {
            error!("KO\t{}", filename);
        }
    }
}

/// impl Write trait so that we can use io::copy later
impl Write for HashChecker {
    /// update all internal hash states while reading the buffer only once
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        trace!("write is called: {}", buf.len());
        self.int_md5.update(buf);
        self.int_sha256.update(buf);
        self.int_sha512.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        trace!("flush is called");
        Ok(())
    }
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
        let f = File::open(filename);
        match f {
            Ok(mut fd) => {
                let mut hc = HashChecker::new();
                let _ = io::copy(&mut fd, &mut hc);
                let _ = hc.flush();
                hc.finalize();
                hc.show();
                hc.check(filename);
            }
            Err(_) => {
                error!("Cannot read {}", filename);
            }
        }
    }
    trace!("hashcheck end");
}
