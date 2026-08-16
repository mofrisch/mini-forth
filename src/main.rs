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
}

fn main() {
    let mut vm = Vm::new();
    vm.push(10);
    vm.push(20);

    if let Ok(value) = vm.pop() {
        println!("{value}");
    }
}
