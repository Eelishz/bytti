pub enum Op {
    Add,          // pop two values, add them, and push the result
    Sub,          // pop two values, subtract them, and push the result
    Mul,          // pop two values, multiply them, and push the result
    Div,          // pop two values, divide them, and push the result
    Lit(i64),     // push a literal onto the stack
    Load(usize),  // load a value from memory and push it onto the stack
    Store(usize), // pop a value and store it in memory
    Label(usize), // create a label to jump to later
    Jmp(usize),   // unconditional jump to a label
    CJmp(usize),  // pop a value off the stack and jump if the value is non-zero
    Put,          // pop a value off the stack and write it to stdout
    Dup,          // duplicate the top value onto the stack.
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
                Op::Load(label) => {
                    let a = self.memory[*label];
                    self.stack.push(a);
                }
                Op::Store(label) => {
                    let a = self.stack.pop()?;
                    assert!(*label <= self.memory.len());
                    if self.memory.len() <= *label {
                        self.memory.push(a);
                    } else {
                        self.memory[*label] = a;
                    }
                }
                Op::Label(_) => (),
                Op::Jmp(label) => {
                    i = self.jump_table[*label];
                }
                Op::CJmp(label) => {
                    let a = self.stack.pop()?;
                    if a != 0 {
                        i = self.jump_table[*label];
                    }
                }
                Op::Put => println!("{}", self.stack.pop()?),
                Op::Dup => {
                    let a = self.stack.pop()?;
                    self.stack.push(a);
                    self.stack.push(a);
                }
            }
            i += 1;
        }
        self.stack.pop()
    }
}

fn main() {
    let mut vm = VM::new();
    let program = vec![
        Op::Lit(10),
        Op::Store(0),
        Op::Label(0),
        Op::Load(0),
        Op::Put,
        Op::Lit(1),
        Op::Load(0),
        Op::Sub,
        Op::Store(0),
        Op::Load(0),
        Op::CJmp(0),
        Op::Lit(0),
    ];

    let top = vm.excecute(&program).unwrap();
    println!("{top}");
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: prop tests

    #[test]
    fn addition() {
        let mut vm = VM::new();
        let ops = vec![Op::Lit(1), Op::Lit(2), Op::Add];

        let top = vm.excecute(&ops).unwrap();
        assert_eq!(top, 3)
    }
}
