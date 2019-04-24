mod opt;
mod run;
use structopt::StructOpt;

fn main() {
    opt::Opt::from_args().run();
}
