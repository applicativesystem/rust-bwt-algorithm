mod args;
use ::std::collections::VecDeque;
use args::BWTArgs;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-14

rust-bwt-algorithm: The BWT algorithm that has been usually written
involves the three step, preorder, sort lexically and then take the bwt index.
I also used this while writing previously the BWT index during my C++ time, but
not in RUST, i am implementing a new way of using it by declaring a struct based
implementation and then you can sort the any implementation.
* */

fn main() {
    let args = BWTArgs::parse();
    let bwt_indices = bwt_deque(&args.bwt_arg).unwrap();
    println!("The bwt has been written: {}", bwt_indices);
}

fn bwt_deque(path: &str) -> Result<String, Box<dyn Error>> {
    struct ReadSeq {
        header: String,
        sequence: String,
    }

    let mut open_readfile = File::open(path).expect("file not present");
    let readfile = BufReader::new(open_readfile);
    let mut bwt_capture_seq: Vec<ReadSeq> = Vec::new();
    let mut bwt_header: Vec<String> = Vec::new();
    let mut bwt_seq: Vec<String> = Vec::new();
    for i in readfile.lines() {
        let line = i.expect("line not present");
        if line.starts_with("@") {
            bwt_header.push(line)
        } else {
            bwt_seq.push(line)
        }
    }
    for i in 0..bwt_header.len() {
        bwt_capture_seq.push(ReadSeq {
            header: bwt_header[i].clone(),
            sequence: bwt_seq[i].clone(),
        })
    }

    #[derive(Debug, Clone, PartialOrd, PartialEq)]
    struct BWTHold {
        back: String,
        front: String,
    }

    for i in bwt_capture_seq.iter() {
        let newstr = String::from(i.sequence.clone());
        let addnew = newstr.chars().map(|x| String::from(x)).collect::<Vec<_>>();
        let mut final_bwt: Vec<_> = Vec::new();
        for i in 0..addnew.len() {
            let mut map_bwt: Vec<BWTHold> = Vec::new();
            let front_push = &addnew[0..addnew.len() - i].to_vec().join("");
            let back_push = &addnew[addnew.len() - i..addnew.len()].to_vec().join("");
            map_bwt.push(BWTHold {
                back: back_push.to_string(),
                front: front_push.to_string(),
            });
            final_bwt.push(map_bwt);
        }
    }

   // implementing a sort step here and then a inverse BWT for the sequence construction.
   // By implementing the struct based approach, it allows me to index the hash faster and based on
   // the cyclic rotation, i can easily insert them into a BTreeHashMap.

    Ok("BWT has been written".to_string())
}
