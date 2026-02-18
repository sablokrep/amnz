use crate::file::Fastakmer;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn jumping_kmers<'a>(
    seq: &'a [u8],
    k: usize,
    step: usize,
) -> Result<impl Iterator<Item = u64> + 'a, Box<dyn Error>> {
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
