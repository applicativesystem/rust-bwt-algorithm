use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct BWTArgs {
    /// please provide the path to the alignment file
    pub bwt_arg: String,
}
