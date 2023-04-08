mod args;
use args::Args;

fn main() {
    let args = Args::get_args();

    println!("{:?}", args);
}
