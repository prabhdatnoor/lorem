use clap::Parser;
use clap::ArgGroup;

mod constants;
mod generator;

#[derive( Parser, Debug)]
#[clap(group = ArgGroup::new("size").required(true).multiple(false))]
struct Arguments {
    #[clap(short = 'w', long = "words", group = "size")]
    words: Option<i32>,
    #[clap(short = 'c', long = "characters", group = "size")]
    characters: Option<i32>,
}

fn main() {
    let args = Arguments::parse();

    // create a new instance of Generator
    let mut generator = generator::Generator::new();

    // if no arguments are passed
    if args.words.is_none() && args.characters.is_none() {
        // add words to the buffer
        generator.add_words(20);
    }

    // if words is not None
    else if let Some(words) = args.words {
        // add words to the buffer
        generator.add_words(words);
    }

    // if characters is not None
    else if let Some(characters) = args.characters {
        // add characters to the buffer
        generator.add_characters(characters);
    }

    // print the buffer
    println!("{}", generator.get_and_clear_buffer());
}