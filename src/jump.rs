use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::file::Fastakmer;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn jumping_kmers<'a>(seq: &'a [u8], k: usize, step: usize) -> impl Iterator<Item = u64> + 'a {
    let encode = |b: u8| match b {
        b'A' | b'a' => 0u64,
        b'C' | b'c' => 1,
        b'G' | b'g' => 2,
        b'T' | b't' => 3,
        _ => 4,
    };

    (0..seq.len().saturating_sub(k))
        .step_by(step)
        .map(move |i| {
            let mut kmer: u64 = 0;
            for j in 0..k {
                kmer = (kmer << 2) | encode(seq[i + j]);
            }
            kmer
        })
}

impl Fastakmer {
    pub fn jumping_kmer_run(&self) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
        let fileopen = File::open(self.fastafile.clone()).expect("file not present");
        let fileread = BufReader::new(fileopen);
        let mut vecstring: Vec<String> = Vec::new();
        for i in fileread.lines() {
            let linevalue = i.expect("fline not present");
            if !linevalue.starts_with(">") {
                vecstring.push(linevalue);
            }
        }

        let mut valuevec: Vec<Vec<u64>> = Vec::new();
        for i in vecstring.iter() {
            let value = i.as_bytes();
            let kmersearch = jumping_kmers(
                value,
                self.kmer.parse::<usize>().unwrap(),
                self.step.parse::<usize>().unwrap(),
            );
            let vecinsert = kmersearch.into_iter().collect::<Vec<_>>();
            valuevec.push(vecinsert);
        }
        Ok(valuevec)
    }
}
