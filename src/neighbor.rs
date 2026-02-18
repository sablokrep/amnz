use crate::file::Neighbors;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

impl Neighbors {
    pub fn genomematch(&self) -> Result<Vec<HashSet<Vec<u8>>>, Box<dyn Error>> {
        let mut valuereturn: Vec<HashSet<Vec<u8>>> = Vec::new();
        let pathfile = Neighbors {
            neighboursfile: self.neighboursfile,
            valuefile: self.valuefile,
        };
        let filevalue = pathfile.neighboursfile;
        let filevalue_extract: usize = pathfile.valuefile.parse::<usize>().unwrap();
        let fileopen = File::open(filevalue).expect("file not present");
        let fileread = BufReader::new(fileopen);
        let vector_string: Vec<&[u8]> = Vec::new();
        for i in fileread.lines() {
            let line = i.expect("file not present");
            let lineslice = b"line";
            vector_string.push(lineslice);
        }
        for i in vector_string.iter() {
            let valueinsert = *i;
            let valuekmer = neighbors(valueinsert, filevalue_extract);
            valuereturn.push(valuekmer);
        }
    }
}

/*
Recursive with in function for the generation of the neighbors kmers
*/

pub fn neighbors(pattern: &[u8], d: usize) -> HashSet<Vec<u8>> {
    if d == 0 {
        return HashSet::from([pattern.to_vec()]);
    }
    let mut results = HashSet::new();
    fn geressn(
        prefix: &mut Vec<u8>,
        pos: usize,
        mismatches_left: usize,
        pattern: &[u8],
        results: &mut HashSet<Vec<u8>>,
    ) {
        if pos == pattern.len() {
            results.insert(prefix.clone());
            return;
        }
        prefix.push(pattern[pos]);
        geressn(prefix, pos + 1, mismatches_left, pattern, results);
        prefix.pop();
        if mismatches_left > 0 {
            for &base in BASES {
                if base != pattern[pos] {
                    prefix.push(base);
                    geressn(prefix, pos + 1, mismatches_left - 1, pattern, results);
                    prefix.pop();
                }
            }
        }
    }
    let mut prefix = Vec::with_capacity(pattern.len());
    geressn(&mut prefix, 0, d, pattern, &mut results);
    results
}
