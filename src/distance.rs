use crate::file::Filereadpath;
use editdistancek::edit_distance as kdistance;
use editdistancek::mismatch;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl Filereadpath {
    pub fn editall(
        &self,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>)>, Box<dyn Error>> {
        let value = File::open(self.filereadpath.clone()).expect("file not present");
        let valueread = BufReader::new(value);
        let mut vectstring: Vec<String> = Vec::new();
        for i in valueread.lines() {
            let line = i.expect("file not present");
            if !line.starts_with(">") {
                vectstring.push(line);
            }
        }
        let editdistance_original = editmake(vectstring.clone()).unwrap();
        let reconstructed = generate_mers(vectstring.clone()).unwrap();
        let editdistance_unique = editmake(reconstructed).unwrap();

        let mut finalvec: Vec<(Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>)> = Vec::new();
        for i in editdistance_original.iter() {
            for val in editdistance_unique.iter() {
                let value: (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) =
                    (i.0.clone(), i.1.clone(), val.0.clone(), val.1.clone());
                finalvec.push(value);
            }
        }
        Ok(finalvec)
    }
}

/*
 Generate unique kmer hash table and then construct the sequence and then estimate the Fischer value.
*/

pub fn generate_mers(pathvec: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let valuevec = pathvec;
    let mut kmerstring: Vec<Vec<&str>> = Vec::new();
    for i in valuevec.iter() {
        let stringvec = i
            .as_bytes()
            .windows(3)
            .map(|x| str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        kmerstring.push(stringvec)
    }
    let allkmers: HashSet<String> = kmerstring
        .iter()
        .flatten()
        .cloned()
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();
    let mut reconstructed_unique_string: Vec<String> = Vec::new();
    for i in kmerstring.iter() {
        for val in i.iter() {
            for coop in allkmers.iter() {
                if val == coop {
                    continue;
                } else {
                    let mut vecstringcontruct: Vec<String> = Vec::new();
                    vecstringcontruct.push(val.to_string());
                    reconstructed_unique_string.push(vecstringcontruct.concat());
                }
            }
        }
    }
    Ok(reconstructed_unique_string)
}

/*
estimate of the fishcer value and the edit distance
*/

pub fn editmake(inputvec: Vec<String>) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>> {
    let mut value: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
    for i in 0..inputvec.len() - 1 {
        let stringsearch = inputvec[i].clone();
        let restvec = inputvec[i + 1..inputvec.len() - 1].to_vec();
        let mut restvecalter: Vec<Vec<String>> = Vec::new();
        for i in restvec.iter() {
            let mut stringsearch: Vec<String> = Vec::new();
            stringsearch.push(i.clone());
            restvecalter.push(stringsearch);
        }
        restvecalter.push(inputvec[0..i].to_vec());
        let flattenrestvec = restvecalter.iter().flatten().cloned().collect::<Vec<_>>();
        let mut editdistance_k: Vec<usize> = Vec::new();
        for i in flattenrestvec.iter() {
            let bytesas = stringsearch.as_bytes();
            let iteratorbytes = i.as_bytes();
            editdistance_k.push(kdistance(bytesas, iteratorbytes));
        }
        let mut editdistance_dist: Vec<usize> = Vec::new();
        for i in flattenrestvec.iter() {
            let bytesas = stringsearch.as_bytes();
            let iteratirbytes = i.as_bytes();
            editdistance_dist.push(mismatch(bytesas, iteratirbytes));
        }
        let valueinsert: (Vec<usize>, Vec<usize>) = (editdistance_k, editdistance_dist);
        value.push(valueinsert);
    }
    Ok(value)
}
