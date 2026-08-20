use std::io::{self, BufRead, Write};

#[derive(Debug, Default, Clone)]
pub struct Vm {
    data_stack: Vec<i64>,
    output: String,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            data_stack: Vec::new(),
            output: String::new(),
        }
    }

    pub fn push(&mut self, value: i64) {
        self.data_stack.push(value);
    }

    pub fn pop(&mut self) -> Result<i64, &'static str> {
        self.data_stack.pop().ok_or("stack underflow")
    }

    fn output(&self) -> &str {
        &self.output
    }

    pub fn eval_line(&mut self, line: &str) -> Result<(), &'static str> {
        for token in line.split_whitespace() {
            self.eval_token(token)?;
        }

        Ok(())
    }

    pub fn eval_token(&mut self, token: &str) -> Result<(), &'static str> {
        match token {
            "+" => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a + b);
            }

            "-" => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a - b);
            }

            "." => {
                let value = self.pop()?;
                self.output.push_str(&format!("{} ", value));
            }

            token => {
                let value = token.parse::<i64>().map_err(|_| "unknown word")?;
                self.push(value);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Vm;

    #[test]
    fn push_and_pop() {
        let mut vm = Vm::new();

        vm.push(42);
        vm.push(7);

        assert_eq!(vm.pop(), Ok(7));
        assert_eq!(vm.pop(), Ok(42));
    }

    #[test]
    fn stack_underflow() {
        let mut vm = Vm::new();

        assert_eq!(vm.pop(), Err("stack underflow"));
    }

    #[test]
    fn eval_positive_integer_token() {
        let mut vm = Vm::new();

        assert_eq!(vm.eval_token("42"), Ok(()));
        assert_eq!(vm.pop(), Ok(42));
    }

    #[test]
    fn eval_negative_integer_token() {
        let mut vm = Vm::new();

        assert_eq!(vm.eval_token("-7"), Ok(()));
        assert_eq!(vm.pop(), Ok(-7));
    }

    #[test]
    fn eval_unknown_token() {
        let mut vm = Vm::new();

        assert_eq!(vm.eval_token("hello"), Err("unknown word"));
        assert_eq!(vm.pop(), Err("stack underflow"));
    }

    #[test]
    fn evaluates_addition() {
        let mut vm = Vm::new();

        vm.eval_token("20").unwrap();
        vm.eval_token("22").unwrap();
        vm.eval_token("+").unwrap();

        assert_eq!(vm.pop().unwrap(), 42);
    }

    #[test]
    fn evaluates_subtraction() {
        let mut vm = Vm::new();

        vm.eval_token("20").unwrap();
        vm.eval_token("12").unwrap();
        vm.eval_token("-").unwrap();

        assert_eq!(vm.pop().unwrap(), 8);
    }

    #[test]
    fn evaluates_multiple_operations() {
        let mut vm = Vm::new();

        vm.eval_token("10").unwrap();
        vm.eval_token("20").unwrap();
        vm.eval_token("+").unwrap();
        vm.eval_token("12").unwrap();
        vm.eval_token("+").unwrap();

        assert_eq!(vm.pop().unwrap(), 42);
    }

    #[test]
    fn dot_prints_and_removes_top_value() {
        let mut vm = Vm::new();

        vm.eval_token("42").unwrap();
        vm.eval_token(".").unwrap();

        assert_eq!(vm.output(), "42 ");
        assert_eq!(vm.pop(), Err("stack underflow"));
    }

    #[test]
    fn dot_fails_on_empty_stack() {
        let mut vm = Vm::new();

        assert_eq!(vm.eval_token("."), Err("stack underflow"));
    }

    #[test]
    fn eval_line_processes_multiple_tokens() {
        let mut vm = Vm::new();

        vm.eval_line("20 22 + .").unwrap();

        assert_eq!(vm.output(), "42 ");
        assert_eq!(vm.pop(), Err("stack underflow"));
    }
}



fn main() {
    let stdin = io::stdin();
    let mut vm = Vm::new();

    println!("mini-forth v0.1");

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read input");
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "quit" || trimmed == "exit" {
            break;
        }

        if let Err(err) = vm.eval_line(trimmed) {
            eprintln!("{err}");
            continue;
        }

        if !vm.output.is_empty() {
            print!("{}", vm.output);
            io::stdout().flush().unwrap();
            vm.output.clear();
            println!();
        }
    }
}
