#![allow(dead_code, unused_variables)]
mod memory;

use std::fs;

const VALID_CHARS: [char; 7] = ['+', '-', '>', '<', '.', '[', ']'];
const TOTAL_MEM: usize = 256;

fn main() {
    let mut content = fs::read_to_string("main.bf").expect("file not good");
    content.retain(|c| VALID_CHARS.contains(&c));

    let program: Vec<char> = content.chars().collect();

    println!("{content:?}");

    let mut loop_stack = Vec::<usize>::new();
    let mut mem = memory::Memory::<TOTAL_MEM>::new();
    let mut ip = 0;

    while ip < program.len() {
        match program[ip] {
            '+' => mem.add(),
            '-' => mem.sub(),
            '>' => mem.next(),
            '<' => mem.prev(),
            '.' => println!("{}", mem.get()),
            '[' => {
                if mem.should_loop() {
                    loop_stack.push(ip);
                } else {
                    while ip < program.len() && program[ip] != ']' {
                        ip += 1;
                    }
                }
            }
            ']' => {
                if mem.should_loop() {
                    ip = *loop_stack
                        .last()
                        .expect("closing a loop that was never opened");
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }
        ip += 1
    }

    println!("memory status: {:?}", mem.inner)
}
