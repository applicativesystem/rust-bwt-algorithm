mod args;
use args::BWTArgs;
use clap::Parser;
use std::cmp::Ord;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-15

rust-bwt-algorithm: The BWT algorithm that has been usually written
involves the three step, preorder, sort lexically and then take the bwt index.
I also used this while writing previously the BWT index during my C++ time, but
not in RUST, i am implementing a new way of using it by declaring a struct based
implementation, so that you can easily, fetch the read based on the BWT, you dont
have to revisit the index of the read.
* */

fn main() {
    let args = BWTArgs::parse();
    let bwt_longread_output = bwt_longread(&args.bwt_arg).unwrap();
    println!("The bwt has been written: {}", bwt_longread_output);
}

fn bwt_longread(path: &str) -> Result<String, Box<dyn Error>> {
    struct ReadSeq {
        header: String,
        sequence: String,
    }

    let open_readfile = File::open(path).expect("file not present");
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
        id: String,
        bwt: String,
    }

    let mut final_bwt_write: Vec<BWTHold> = Vec::new();
    for i in bwt_capture_seq.iter() {
        let newstr = String::from(i.sequence.clone());
        let addnew = newstr.chars().map(|x| String::from(x)).collect::<Vec<_>>();
        let mut final_bwt: Vec<_> = Vec::new();
        let mut add_bwt: Vec<_> = Vec::new();
        for i in 0..addnew.len() {
            let mut map_bwt: Vec<_> = Vec::new();
            let front_push = &addnew[0..addnew.len() - i].to_vec().join("");
            let back_push = &addnew[addnew.len() - i..addnew.len()].to_vec().join("");
            map_bwt.push(back_push);
            map_bwt.push(front_push);
            final_bwt.push(format!("{}{}", map_bwt[0], map_bwt[1]));
        }
        final_bwt.sort();
        add_bwt.push(final_bwt);
        final_bwt_write.push(BWTHold {
            id: i.header.clone(),
            bwt: add_bwt[0]
                .iter()
                .map(|x| &x[x.len() - 1..x.len()])
                .collect::<Vec<_>>()
                .concat(),
        })
    }

    let mut bwt_write_long = File::create("string-bwt-indices.txt").expect("file not present");
    for i in final_bwt_write.iter() {
        writeln!(bwt_write_long, "{}\t{}\n", i.id, i.bwt).expect("line not present");
    }

    Ok("BWT has been written".to_string())
}
