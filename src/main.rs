use clap::Parser;
use clap::ArgGroup;
mod constants;

#[derive(Default, Parser)]
#[clap(group= ArgGroup::new("size").required(true).multiple(false))]
struct Arguments{
    #[clap(short='w', long="words", default_value="20", group="size")]
    words: Option<i32>,
    #[clap(short ='c', long="characters", default_value="100", group="size")]
    characters: Option<i32>
}

fn main() {
    let args = Arguments::parse();

    // print the args
    println!("words: {}", args.words.unwrap());
}