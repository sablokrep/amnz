use crate::file::IterativeFasta;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl IterativeFasta {
    pub fn iterativeslide(&self) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
        let filepathuncover = IterativeFasta {
            fastafile: self.fastafile.clone(),
            kmer: self.kmer,
        };
        let filepath = filepathuncover.fastafile;
        let kmer = filepathuncover.kmer;

        let fileopen = File::open(filepath).expect("file not present");
        let fileread = BufReader::new(fileopen);
        let mut vecstring: Vec<String> = Vec::new();
        for i in fileread.lines() {
            let line = i.expect("file not present");
            if !line.starts_with(">") {
                vecstring.push(line.clone())
            }
        }

        /*
         * sliding cpature of the kmers a part of the Knuth morission algorithm
         */

        let slidevec: Vec<Vec<&str>> = Vec::new();
        for i in vecstring.iter() {
            let stringlen = i.clone();
            let mut stringvec: Vec<String> = Vec::new();
            for i in 0..stringlen.len() {
                let firstiter = stringlen[i..kmer].to_string();
                let seconditer = stringlen[i + 1..kmer + 1usize].to_string();
                stringvec.push(firstiter);
                stringvec.push(seconditer);
            }
        }

        let distance = editdistrance(slidevec).unwrap();
        Ok(distance)
    }
}

pub fn editdistrance(inputvec: Vec<Vec<&str>>) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let vecinput = inputvec.clone();
    let mut editdistance: Vec<Vec<usize>> = Vec::new();
    for i in vecinput.iter() {
        let vecclone = i.clone();
        let mut vecusize: Vec<usize> = Vec::new();
        for i in 0..vecclone.len() - 1 {
            let veca = vecclone[i].chars().collect::<Vec<_>>();
            let vecb = vecclone[i + 1].chars().collect::<Vec<_>>();
            for i in veca.iter() {
                for val in vecb.iter() {
                    if i == val {
                        vecusize.push(1usize);
                    } else if i != val {
                        vecusize.push(0usize);
                    } else {
                        continue;
                    }
                }
            }
            editdistance.push(vecusize.clone());
        }
    }
    Ok(editdistance)
}
