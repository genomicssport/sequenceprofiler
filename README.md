# sequenceprofiler

<img src="https://github.com/IBCHgenomic/sequenceprofiler/blob/main/sequenceprofiler.png" width="100" />

 - This crate has the following features: fasta file should be a linear fasta and not a multi line fasta just like long-read.
 - Sequence, which allows based on the similarity of the shared unique kmers and also allows for the filtering of the sequences so that you can build a native index graph faster.
 - SequenceSeq, which allows for the sequence similarity on a sequence to next iter sequence.
 - longread: finding the origin of the kmers.Back to sequences:Find the origin of 𝑘-mers DOI: 10.21105/joss.07066. Output a table for the direct ingestion into any graphs. Outputs a sam type file with the distinct count of the kmers and can be used for the jellyfish count.Support both the genome and the longread fasta file.
 - Jellyfish: a rust implementation of the jellyfish for the counts.Outputs both the unique counts, all counts.It will produce allkmers, uniquekmers, countkmers


 ```
 cargo build

 ```
 ```
 ___    ___    __ _   _   _    ___   _ __     ___    ___   _ __    _ __    ___    / _| (_) | |   ___   _ __
/ __|  / _ \  / _` | | | | |  / _ \ | '_ \   / __|  / _ \ | '_ \  | '__|  / _ \  | |_  | | | |  / _ \ | '__|
\__ \ |  __/ | (_| | | |_| | |  __/ | | | | | (__  |  __/ | |_) | | |    | (_) | |  _| | | | | |  __/ | |
|___/  \___|  \__, |  \__,_|  \___| |_| |_|  \___|  \___| | .__/  |_|     \___/  |_|   |_| |_|  \___| |_|
                 |_|                                      |_|

sequenceprofiler
   ************************************************
       Gaurav Sablok, IBCH, PAN, Poznan, Poland,
       https://portal.ichb.pl/laboratory-of-genomics/.
       Email: gsablok@ibch.poznan.pl
   ************************************************

Usage: sequenceprofiler <COMMAND>

Commands:
 sequence      identity kmer similarity index
 filter        identity kmer filter
 sequence-seq  compare seq to other seq 1-1 iteration
 jellyfish     jellyfish counter for the long reads
 origin-kmer   finding the origin of kmers
 help          Print this message or the help of the given subcommand(s)

Options:
 -h, --help     Print help
 -V, --version  Print version
 ```
 - to run the compiled library

 ```
./target/debug/sequenceprofiler sequence
     ./samplefile/sequence-sample-files/sample.fasta 4
./target/debug/sequenceprofiler filter
       ./samplefile/sequence-sample-files/sample.fasta 4 10
./target/debug/sequenceprofiler origin-kmer
           ./samplefile/longread-sample-files/fastafile.fasta 4
./target/debug/sequenceprofiler jellyfish
          ./samplefile/jellyfish-sample-files/test.fastq 4
```

 - To install windows version:
 ```
rustup component add llvm-tools
rustup target add x86_64-pc-windows-msvc
git clone https://github.com/IBCHgenomic/ensemblcov.git
cd ensemblcov
cargo xwin build --target x86_64-pc-windows-msvc
 ```

Gaurav Sablok \
Instytut Chemii Bioorganicznej Polskiej \
Akademii Nauk ul. Noskowskiego 12/14 \
| 61-704, Poznań Poland
