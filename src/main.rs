#[derive(Debug, Default, Clone)]
pub struct Vm {
    data_stack: Vec<i64>,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: i64) {
        self.data_stack.push(value);
    }

    pub fn pop(&mut self) -> Result<i64, &'static str> {
        self.data_stack.pop().ok_or("stack underflow")
    }

    
    fn eval(&mut self, input: &str) -> Result<(), &'static str> {
        for token in input.split_whitespace() {
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

                token => {
                    let value = token.parse::<i64>().map_err(|_| "unknown word")?;
                    self.push(value);
                }
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

        assert_eq!(vm.eval("42"), Ok(()));
        assert_eq!(vm.pop(), Ok(42));
    }

    #[test]
    fn eval_negative_integer_token() {
        let mut vm = Vm::new();

        assert_eq!(vm.eval("-7"), Ok(()));
        assert_eq!(vm.pop(), Ok(-7));
    }

    #[test]
    fn eval_unknown_token() {
        let mut vm = Vm::new();

        assert_eq!(vm.eval("hello"), Err("unknown word"));
        assert_eq!(vm.pop(), Err("stack underflow"));
    }

    #[test]
    fn evaluates_addition() {
        let mut vm = Vm::new();

        vm.eval("20 22 +").unwrap();

        assert_eq!(vm.pop().unwrap(), 42);
    }

    #[test]
    fn evaluates_subtraction() {
        let mut vm = Vm::new();

        vm.eval("20 12 -").unwrap();

        assert_eq!(vm.pop().unwrap(), 8);
    }

    #[test]
    fn evaluates_multiple_operations() {
        let mut vm = Vm::new();

        vm.eval("10 20 + 12 +").unwrap();

        assert_eq!(vm.pop().unwrap(), 42);
    }
}

fn main() {
    let mut vm = Vm::new();

    vm.eval("20 22 +").unwrap();

    println!("{:?}", vm.data_stack);
}
