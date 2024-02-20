use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "random_groups", about = "A tool to generate random groups from a CSV file.")]
pub(crate) struct Opt {
    /// Path to the CSV file
    #[structopt(short = "f", long)]
    pub(crate) file_path: String,

    /// Number of groups to generate
    #[structopt(short = "n", long)]
    pub(crate) n_groups: usize,
}