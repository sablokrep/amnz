use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "amNZ",
    version = "1.0",
    about = "Bee Microbiome Kmer based Machine and Deep learning
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run Kmer based machine learning
    KmerMachine {
        /// identity value
        identity: String,
        /// path to the fasta file training
        fileopen: String,
        /// path to the predict file
        training: String,
        /// threads for the analysis
        thread: String,
    },
}
