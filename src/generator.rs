use std::io::Write;
use rand::Rng;
use crate::constants;

pub(crate) struct Generator {
    // random number generator
    rng: rand::rngs::ThreadRng,
    // new line character
    new_line: String,
    // a buffer to store the generated text
    buffer: Vec<u8>,
}

// impl for Generator make public
impl Generator {
    // constructor
    pub fn new() -> Self {
        let new_line = String::from("\n");
        // create a new random number generator
        let rng = rand::thread_rng();
        // create a new buffer
        let buffer = Vec::new();
        // return a new instance of Generator
        Self {
            rng,
            new_line,
            buffer,
        }
    }
    // generate a random word
    fn generate_word(&mut self) -> String {
        // get a random index
        let index = self.rng.gen_range(0..constants::WORDS_LENGTH);
        // return the word at the index
        constants::WORDS[index as usize].to_string()
    }

    // write the string to the buffer
    fn write(&mut self, s: &str) {
        // add the string to the buffer
        write!(&mut self.buffer, "{}", s).unwrap();
    }

    // add x words to the buffer
    pub fn add_words(&mut self, x: i32){
        // loop x times
        for i in 0..x {
            // generate a word
            let word = self.generate_word();
            // write the word to the buffer
            self.write(&word);
            // if we are not at the last word
            if i < x - 1 {
                // add a space
                self.write(" ");
            }
        }
    }

    // add x characters to the buffer
    pub fn add_characters(&mut self, mut x: i32){
        // loop while we still have characters to add
        while x > 0 {
            // generate a word
            let word = self.generate_word();
            let word_len = word.len() as i32;
            // get substring of word
            let sub = &word[0..x.min(word_len) as usize];

            // write the word to the buffer
            self.write(sub);

            // if we have more characters to add
            if x > word_len {
                // add a space
                self.write(" ");
                // subtract the length of the word and the space from x
                x -= word_len + 1;
            }
        }
    }

    // return and clear the buffer as string
    pub fn get_and_clear_buffer(&mut self) -> String {
        // create a new string from the buffer
        let s = String::from_utf8_lossy(&self.buffer).to_string();
        // clear the buffer
        self.buffer.clear();
        // return the string
        s
    }
}