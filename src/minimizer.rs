use crate::file::MachineMinimizers;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl MachineMinimizers {
    pub fn run_minimizers<'a>(&self) -> Result<Vec<Vec<(usize, usize)>>, Box<dyn Error>> {
        let filepathread = MachineMinimizers {
            pathfile: self.pathfile,
            value_k: self.value_k,
            value_w: self.value_w,
            can: self.can,
        };
        let readfile = filepathread.pathfile;
        let readfile = File::open(readfile).expect("file not present");
        let readread = BufReader::new(readfile);
        let mut vecstring: Vec<&[u8]> = Vec::new();
        for i in readread.lines() {
            let value = i.expect("line not present");
            let valuebytes = b"value";
            vecstring.push(valuebytes);
        }
        /*
        Making a vector of impl
        */
        let returnspec: Vec<Vec<(usize, u64)>> = Vec::new();
        for i in vecstring.iter() {
            let valueimpl = minimizers_machinelearning(
                i,
                self.value_k.parse::<usize>().unwrap(),
                self.value_w.parse::<usize>().unwrap(),
                self.can,
            );
            for i in valueimpl.into_iter() {
                let mut valuspec: Vec<(usize, u64)> = Vec::new();
                valuspec.push(i);
                returnspec.push(valuspec);
            }
        }
        Ok(returnspec)
    }
}

pub fn minimizers_machinelearning<'a>(
    seq: &'a [u8],
    k: usize,
    w: usize,
    canonical: bool,
) -> impl Iterator<Item = (usize, u64)> + 'a {
    assert!(w >= k);
    let encode = |b: u8| match b.to_ascii_uppercase() {
        b'A' => 0u64,
        b'C' => 1,
        b'G' => 2,
        b'T' => 3,
        _ => u64::MAX,
    };
    let revcomp = |x: u64, len: usize| -> u64 {
        let mut rc = 0u64;
        let mut t = x;
        for _ in 0..len {
            let base = t & 3;
            rc = (rc << 2) | (3 ^ base);
            t >>= 2;
        }
        rc
    };
    let mut queue = VecDeque::new();
    seq.windows(k).enumerate().map(move |(i, window)| {
        let mut kmer: u64 = 0;
        for &b in window {
            kmer = (kmer << 2) | encode(b);
        }
        let best = if canonical {
            let rc = revcomp(kmer, k);
            std::cmp::min(kmer, rc)
        } else {
            kmer
        };
        queue.push_back((i, best));
        while let Some(&(pos, _)) = queue.front() {
            if pos + w - 1 < i + k - 1 {
                queue.pop_front();
            } else {
                break;
            }
        }
        let min_val = queue.iter().map(|&(_, v)| v).min().unwrap();
        let min_pos = queue.iter().position(|&(_, v)| v == min_val).unwrap() + queue[0].0;

        (min_pos, min_val)
    })
}
