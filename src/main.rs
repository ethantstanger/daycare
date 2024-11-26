use lexer::lex;
use lexer::Token;
use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

mod errors;
mod lexer;

const PATH_SUFFIX: &str = ".dc";

fn main() {
    let filepath = {
        let path_string = args().filter(|x| x.ends_with(PATH_SUFFIX)).nth(0);

        match path_string {
            None => panic!("No valid filepath given"),
            Some(x) => PathBuf::from_str(x.as_str()).unwrap(),
        }
    };

    let file =
        File::open(&filepath).expect(format!("Failed to open {}", filepath.display()).as_str());

    let tokens: Vec<Token> = BufReader::new(file)
        .lines()
        .flat_map(|x| lex(x.unwrap()).unwrap())
        .collect();

    println!("{:?}", tokens);
}

/**
 * kids can break toys? 0/0
 *
 * TOYS:
 *
 * - save/print_line
 * - load/read_line
 * - add - pencil?
 * - subtract - eraser?
 * - if
 * - hot-potato
 *
 * PERSONALITIES:
 *
 * - shy - in memory
 * - talkative - standard io
 * - empathetic - another kid
 * - pessimistic - reverse operations
 * - smart-alec - higher precision
 * - articulate - string operations
 *
 *
 * kid [kid_name] is pessimistic {
 *     share [toy] with [kid]
 *     share david with charlie
 *     pass hot-potato to charlie
 *     give charlie david
 *     give charlie hot-potato
 *
 *     imagine "y" is use [toy] for x
 *     remember
 *
 *     imagine x is 3
 *     tell x to [kid]
 *     forget x
 *
 *     x is 4
 *     imagine "x" is "3"
 *     tell x to charlie
 *     lose "x"
 *
 *     share self
 *     share toy1
 *     pass hot-potato
 * }
 *
 */
fn shut_up() -> u32 { 3 }
