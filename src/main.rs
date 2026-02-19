mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::xgboost::xgb;
use clap::Parser;
use figlet_rs::FIGfont;
mod allkmers;
mod edit;
mod file;
mod ham;
mod jump;
mod minimizer;
mod neighbor;

/*
Gaurav Sablok
codeprog@icloud.com
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("amNZ");
    println!("{}", repgenerate.unwrap());
    let args = CommandParse::parse();
    match &args.command {
        Commands::KmerMachine {
            identity,
            fileopen,
            training,
            thread,
        } => {
            let n_threads = thread.parse::<usize>().expect("value not found");
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("thread build failed");
            pool.install(|| {
                let tensorrun = xgb(identity, fileopen, training).unwrap();
                println!("The xgboost classification is finished: {}", tensorrun);
            });
        }
    }
}
