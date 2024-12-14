# rust-bwt-algorithm
 
 - rust implementation of Burrows-Wheeler transform for long range sequencing as well as the short reads.
 - writing the Burrows-Wheeler transform in RUST all parts separate later for merge into RUST-bowtie.
 - in this i implement a struct based approach so that based on the sorted keys i can push them into a BTreeMap, which will make them on a log scale much faster.  
 - general note: Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version. 
```
cargo build
```

```
➜  rust-bwt-algorithm git:(main) ✗ ./target/debug/rust-bwt-algorithm -h
Usage: rust-bwt-algorithm <BWT_ARG>

Arguments:
  <BWT_ARG>  please provide the path to the fastq file

Options:
  -h, --help     Print help
  -V, --version  Print version
``` 

 Gaurav Sablok
