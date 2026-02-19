use crate::file::Fasta;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn all_kmers(seq: &[u8], k: usize) -> impl Iterator<Item = u64> + '_ {
    let encode = |b: u8| match b {
        b'A' | b'a' => 0,
        b'C' | b'c' => 1,
        b'G' | b'g' => 2,
        b'T' | b't' => 3,
        _ => 4,
    };
    seq.windows(k).map(move |window| {
        let mut kmer: u64 = 0;
        for &b in window {
            kmer = (kmer << 2) | encode(b) as u64;
        }
        kmer
    })
}

impl Fasta {
    pub fn readimport(&self, kmer: &str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
        let valuefile = File::open(self.fastafile.clone()).expect("file not present");
        let valuread = BufReader::new(valuefile);
        let mut valuestring: Vec<String> = Vec::new();
        for i in valuread.lines() {
            let line = i.expect("line not present");
            if !line.starts_with(">") {
                valuestring.push(line);
            }
        }
        let mut stringadd: Vec<Vec<u64>> = Vec::new();
        for i in valuestring.iter() {
            let value = i.as_bytes();
            let valueunwrap = all_kmers(value, kmer.parse::<usize>().unwrap());
            let mut valusec: Vec<u64> = Vec::new();
            for i in valueunwrap.into_iter() {
                valusec.push(i);
            }
            stringadd.push(valusec);
        }
        Ok(stringadd)
    }
}
