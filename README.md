# graph-kmers
 - rust graph kmers which allows based on the similarity of the shared unique kmers filtering of the sequences so that you can build anative index faster graph.

 ```
 cargo build 
 
 ```
 ```
 ➜  graph-clusters git:(main) ✗ ./target/debug/graph-kmers -h                                                                                       
 graphkmers                                                                                                                                         
                                                                                                                                                   
 Usage: graph-kmers <COMMAND>                                                                                                                       
                                                                                                                                                   
 Commands:                                                                                                                                          
  sequence  profile the similarity based on the observed kmer                                                                                      
  help      Print this message or the help of the given subcommand(s)                                                                              
                                                                                                                                                   
 Options:                                                                                                                                           
  -h, --help     Print help                                                                                                                        
  -V, --version  Print version                                                                                                                     
 ➜  graph-clusters git:(main) ✗ ./target/debug/graph-kmers sequence -h                                                                              
 profile the similarity based on the observed kmer                                                                                                  
                                                                                                                                                   
 Usage: graph-kmers sequence <SEQUENCEPATH> <SEQUENCEKMER>                                                                                          
                                                                                                                                                   
 Arguments:                                                                                                                                         
  <SEQUENCEPATH>  provide the path to sequence file                                                                                                
  <SEQUENCEKMER>  provide the kmer to be profiled for the sequence similarity                                                                      
                                                                                                                                                   
 Options:                                                                                                                                           
  -h, --help  Print help 

 ```

