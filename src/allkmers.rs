use crate::file::Fasta;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl Fasta {
    pub fn all_kmers(
        seq: &[u8],
        k: usize,
    ) -> Result<impl Iterator<Item = u64> + '_, Box<dyn Error>> {
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
}
