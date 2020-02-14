use std::fs;
use std::env;
use std::io::*;

#[derive(Debug)]
pub enum BfSym {
    IncrData,
    DecrData,
    IncrPtr,
    DecrPtr,
    LoopStart,
    LoopEnd,
    Input,
    Output,
    Comment,
}

fn match_sym(character: char) -> BfSym {
    match character {
        '+' => BfSym::IncrData,
        '-' => BfSym::DecrData,
        '>' => BfSym::IncrPtr,
        '<' => BfSym::DecrPtr,
        ',' => BfSym::Input,
        '.' => BfSym::Output,
        '[' => BfSym::LoopStart,
        ']' => BfSym::LoopEnd,
        _ => BfSym::Comment
    }
}

#[derive(Debug)]
struct BfRunTape {
    tape: Vec<u8>,
    index: usize,
}

trait BfTapeOps {
    fn input(&mut self);

    fn output(&self);

    fn incr_ptr(&mut self);

    fn decr_ptr(&mut self);

    fn incr_data(&mut self);

    fn decr_data(&mut self);
}

fn sym_fun<T: BfTapeOps>(character: char, ops: &mut T) {
    match character {
        '+' => ops.incr_data(),
        '-' => ops.decr_data(),
        '>' => ops.incr_ptr(),
        '<' => ops.decr_ptr(),
        ',' => ops.input(),
        '.' => ops.output(),
        /*
        '[' => ops.,
        ']' => ops.,
        */
        _ => (),
    }
}

impl BfRunTape {
    fn new() -> BfRunTape {
        let mut tape = Vec::new();
        tape.push(0);
        BfRunTape {
            tape: tape,
            index: 0
        }
    }
}

impl BfTapeOps for BfRunTape {
    fn input(&mut self) {
        self.tape[self.index] = std::io::stdin().bytes().nth(0).unwrap().unwrap();
    }

    fn output(&self) {
        print!("{}", self.tape[self.index] as char);
    }

    fn incr_ptr(&mut self) {
        if self.index == self.tape.capacity() - 1 {
            self.tape.resize(self.tape.capacity() * 2, 0);
        }
        self.index += 1;
    }

    fn decr_ptr(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn incr_data(&mut self) {
        // use Vec::get
        if self.tape[self.index] < 255 {
            self.tape[self.index] += 1;
        }
    }

    fn decr_data(&mut self) {
        if self.tape[self.index] > 0 {
            self.tape[self.index] -= 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_name;

    // bad
    if args.len() >= 2 {
        input_file_name = &args[1];
    } else {
        return;
    }

    let input_file = fs::read_to_string(input_file_name).expect("error");

    let mut tape = BfRunTape::new();

    for i in 0..input_file.len() {
        let charact = input_file.chars().nth(i).unwrap();
        // println!("{:?}", match_sym(charact));
        sym_fun(charact, &mut tape);
    }

    // println!("{:?}", tape);


    /*
    println!("{:?}", tape);

    tape.incr_data();
    println!("{:?}", tape);

    tape.decr_data();
    println!("{:?}", tape);

    tape.decr_data();
    println!("{:?}", tape);

    tape.incr_ptr();
    println!("{:?}", tape);

    tape.incr_data();
    println!("{:?}", tape);

    tape.input();
    println!("{:?}", tape);
    
    tape.output();
    */
}
