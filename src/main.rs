pub enum Op {
    Add,          // pop two values, add them, and push the result
    Sub,          // pop two values, subtract them, and push the result
    Mul,          // pop two values, multiply them, and push the result
    Div,          // pop two values, divide them, and push the result
    Lit(i64),     // push a literal onto the stack
    Load,         // load a value from memory and push it onto the stack
    Store,        // pop a value and store it in memory
    Label(usize), // create a label to jump to later
    Jmp,          // unconditional jump to a label
    CJmp,         // pop a value off the stack and jump if the value is non-zero
    Put,          // pop a value off the stack and write it to stdout
    Dup,          // duplicate the top value onto the stack
    Swap,         // swap the top two values on the stack
    Eq,           // pop two values and put a one onto the stack if a == b, otherwise put zero
    Lt,           // pop two values and put a one onto the stack if a < b, otherwise put zero
    Gt,           // pop two values and put a one onto the stack if a > b, otherwise put zero
}

pub struct VM {
    stack: Vec<i64>,
    memory: Vec<i64>,
    jump_table: Vec<usize>,
}

impl VM {
    pub const fn new() -> VM {
        VM {
            stack: Vec::new(),
            memory: Vec::new(),
            jump_table: Vec::new(),
        }
    }

    pub fn dump(&self) {
        println!("stack {:?}", self.stack);
        println!("memory {:?}", self.memory);
        println!("jmp table {:?}", self.jump_table);
    }

    pub fn excecute(&mut self, program: &Vec<Op>) -> Option<i64> {
        // Populate jump table
        for (i, op) in program.iter().enumerate() {
            match op {
                Op::Label(label) => {
                    assert!(*label <= self.jump_table.len());
                    if self.jump_table.len() <= *label {
                        self.jump_table.push(i);
                    } else {
                        self.jump_table[*label] = i;
                    }
                }
                _ => (),
            }
        }

        let mut i = 0; // Instruction pointer
        loop {
            if i >= program.len() {
                break;
            }
            let op = &program[i];
            match op {
                Op::Add => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a + b);
                }
                Op::Sub => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a - b);
                }
                Op::Mul => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a * b);
                }
                Op::Div => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a / b); // TODO: this can fail
                }
                Op::Lit(x) => self.stack.push(*x),
                Op::Load => {
                    let ptr = self.stack.pop()? as usize; // TODO: deal with negatives
                    let a = self.memory[ptr];
                    self.stack.push(a);
                }
                Op::Store => {
                    let ptr = self.stack.pop()? as usize;
                    let a = self.stack.pop()?;
                    assert!(ptr <= self.memory.len());
                    if self.memory.len() <= ptr {
                        self.memory.push(a);
                    } else {
                        self.memory[ptr] = a;
                    }
                }
                Op::Label(_) => (),
                Op::Jmp => {
                    let label = self.stack.pop()? as usize;
                    i = self.jump_table[label];
                }
                Op::CJmp => {
                    let label = self.stack.pop()? as usize;
                    let a = self.stack.pop()?;
                    if a != 0 {
                        i = self.jump_table[label];
                    }
                }
                Op::Put => println!("{}", self.stack.pop()?),
                Op::Dup => {
                    let a = self.stack.pop()?;
                    self.stack.push(a);
                    self.stack.push(a);
                }
                Op::Swap => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a);
                    self.stack.push(b);
                }
                Op::Eq => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(if a == b { 1 } else { 0 });
                }
                Op::Lt => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(if a < b { 1 } else { 0 });
                }
                Op::Gt => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(if a < b { 1 } else { 0 });
                }
            }
            i += 1;
        }
        self.stack.pop()
    }
}

struct Lexer {}

impl Lexer {
    fn codegen(program: &str) -> Vec<Op> {
        let mut tokens = Vec::new();
        for x in program.split_whitespace() {
            let op = match x {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                "load" => Op::Load,
                "store" => Op::Store,
                "jmp" => Op::Jmp,
                "cjmp" => Op::CJmp,
                "." => Op::Put,
                "dup" => Op::Dup,
                "swap" => Op::Swap,
                "=" => Op::Eq,
                "<" => Op::Lt,
                ">" => Op::Gt,
                lit => {
                    let parse_res = lit.parse::<i64>();
                    if parse_res.is_ok() {
                        Op::Lit(parse_res.unwrap())
                    } else {
                        let mut label = lit.chars();
                        if label.next_back() != Some(':') {
                            println!("{lit}");
                            assert!(false);
                        }
                        let label = label.as_str();
                        Op::Label(label.parse().unwrap())
                    }
                }
            };
            tokens.push(op);
        }
        tokens
    }
}

fn main() {
    let mut args = std::env::args();
    let program = args.next().unwrap();
    let input_path = args.next().unwrap();
    let input = std::fs::read_to_string(input_path).unwrap();

    let bytecode = Lexer::codegen(&input);
    let mut vm = VM::new();
    let _exit_code = vm.excecute(&bytecode).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: prop tests

    #[test]
    fn addition() {
        let mut vm = VM::new();
        let program = vec![Op::Lit(1), Op::Lit(2), Op::Add];

        let top = vm.excecute(&program).unwrap();
        assert_eq!(top, 3)
    }
}
