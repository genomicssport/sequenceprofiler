# graph-kmers
 - rust graph kmers which allows based on the similarity of the shared unique kmers filtering of the sequences so that you can build anative index faster graph.

 ```
 cargo build 
 
 ```
 ```
 12:33:48 gauravsablok@genome graph-kmer main ? ./target/debug/graph-kmers -h
 graphkmers

 Usage: graph-kmers <COMMAND>

 Commands:
  sequence  identity kmer similarity index
  filter    identity kmer filter
  help      Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 12:33:53 gauravsablok@genome graph-kmer main ? ./target/debug/graph-kmers sequence -h
 identity kmer similarity index

 Usage: graph-kmers sequence <SEQUENCEPATH> <SEQUENCEKMER>

 Arguments:
  <SEQUENCEPATH>  provide the path to sequence file
  <SEQUENCEKMER>  provide the kmer to be profiled for the sequence similarity

 Options:
  -h, --help  Print help
 12:33:57 gauravsablok@genome graph-kmer main ? ./target/debug/graph-kmers filter -h
 identity kmer filter

 Usage: graph-kmers filter <SEQUENCE> <KMER> <THRESHOLD>

 Arguments:
  <SEQUENCE>   provide the path to the sequence file
  <KMER>       sequence kmer for the identity kmer
  <THRESHOLD>  provide the threshold

 Options:
  -h, --help  Print help
 
 ```
 - to run the compiled library 
 ```
 12:53:04 gauravsablok@genome graph-kmer main ? \ 
           ./target/debug/graph-kmers filter \
                           ./samplefile/sample.fasta 4 10
 
 12:56:27 gauravsablok@genome samplefile main ? 
           ./target/debug/graph-kmers sequence 
                          ./samplefile/sample.fasta 4


 12:55:34 gauravsablok@genome samplefile main ? head -n 4 frequencies-t*
 ==> frequencies-tab.txt <== filtering without the threshold
 "chr11:66478458"        "chr11:66478459"        22      44      50.0
 "chr11:66478458"        "chr11:66478460"        12      44      27.272728
 "chr11:66478458"        "chr11:66478461"        8       42      19.047619
 "chr11:66478458"        "chr11:66478462"        9       41      21.95122

 ==> frequencies-threshold.txt <== filtering after the threshold
 "chr11:66478458"        "chr11:66478459"        22      44      50.0
 "chr11:66478458"        "chr11:66478460"        12      44      27.272728
 "chr11:66478458"        "chr11:66478461"        8       42      19.047619
 "chr11:66478458"        "chr11:66478462"        9       41      21.95122
 ```

 Gaurav Sablok

