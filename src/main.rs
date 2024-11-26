use lexer::lex;

mod errors;
mod lexer;

fn main() {
    println!("{:?}", lex("".to_string()));
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
