use crate::filestruct::Collector;
use crate::filestruct::Genomeiter;
use crate::filestruct::ProfileKmer;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
/*

 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-7

  profiling the kmer on the sequences and then based on the
  sequence kmers implementing a similarity ratio and that says

  kmerratio = observer unique kmer /
          number of the total unique kmer * 100
*/

pub fn profilesimilarity(path: &str, kmer: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut sequencevector: Vec<String> = Vec::new();
    let mut headervector: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not found");
        if line.starts_with(">") {
            headervector.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequencevector.push(line);
        }
    }
    let mut combinedinfo: Vec<Genomeiter> = Vec::new();
    for i in 0..headervector.len() {
        combinedinfo.push(Genomeiter {
            header: headervector[i].clone(),
            sequence: sequencevector[i].clone(),
        })
    }

    let mut seqbtreemap: Vec<(String, Vec<String>)> = Vec::new();
    for i in combinedinfo.iter() {
        let windowkmer: Vec<_> = i
            .sequence
            .chars()
            .map(String::from)
            .collect::<Vec<_>>()
            .windows(kmer.parse::<usize>().unwrap())
            .map(|x| x.join("").to_string())
            .collect::<Vec<_>>();
        let sequencehash: Vec<String> = windowkmer
            .into_iter()
            .collect::<HashSet<String>>()
            .into_iter()
            .collect::<Vec<_>>();
        seqbtreemap.push((i.sequence.clone(), sequencehash));
    }

    /*
      This is the change for the one to many comparison across the chained iteration and
      then listing out the iteration identity.

    */

    let mut newbase: Vec<ProfileKmer> = Vec::new();
    let mut shared: Vec<Collector> = Vec::new();
    for i in 0..seqbtreemap.len() - 1 {
        let mut countkmer: usize = 0usize;
        let vec = seqbtreemap[i].clone();
        let restvec: Vec<_> = seqbtreemap[i + 1usize..seqbtreemap.len()]
            .iter()
            .collect::<Vec<_>>();
        for restvectiter in 0..restvec.len() {
            for itercount in 0..vec.1.len() {
                if restvec[restvectiter]
                    .1
                    .contains(&vec.1[itercount].to_string())
                {
                    countkmer += 1usize;
                }
            }
            shared.push(Collector {
                name: vec.0.clone(),
                id: restvec[restvectiter].0.clone(),
                count: countkmer,
            });
        }

        let sharedvalue: usize = seqbtreemap[i].1.len() + seqbtreemap[i + 1].1.len();
        newbase.push(ProfileKmer {
            name: seqbtreemap[i].0.clone(),
            sequence: seqbtreemap[i].1.clone(),
            count: countkmer,
            shared: sharedvalue,
            ratio: countkmer / sharedvalue * 100,
        });
    }
    let mut filewrite = File::create("sequence-clusters.fasta").expect("file not found");
    for i in newbase.iter() {
        writeln!(
            filewrite,
            "{:?}\t{:?}\t{:?}\t{:?}",
            i.name, i.count, i.shared, i.ratio
        )
        .expect("file not found");
    }
    Ok(
        "The sequence similarity scores and the cluster of the sequences have been written"
            .to_string(),
    )
}
