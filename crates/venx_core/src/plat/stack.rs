// TODO: Needs optimization. (Options not working)
pub struct EStack<T: Clone> {
    stack: [T; 21],
    pointer: usize,
}

impl<T: Clone> EStack<T> {
    pub fn push(&mut self, vars: T) {
        self.pointer += 1;
        self.stack[self.pointer] = vars;
    }
    pub fn read(&mut self) -> &mut T {
        &mut self.stack[self.pointer]
    }
    pub fn pop(&mut self) {
        self.pointer -= 1;
    }
    pub fn new(vars: T) -> Self {
        EStack {
            stack: [
                // TODO: Improve
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
                vars.clone(),
            ],
            pointer: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EStack;

    #[test]
    fn test_estack() {
        let mut stack = EStack::new(0);
        assert_eq!(*stack.read(), 0);

        stack.push(1);
        assert_eq!(*stack.read(), 1);

        stack.push(2);
        assert_eq!(*stack.read(), 2);
        assert_eq!(*stack.read(), 2);
        assert_eq!(*stack.read(), 2);

        stack.pop();
        assert_eq!(*stack.read(), 1);

        stack.push(3);
        assert_eq!(*stack.read(), 3);
        assert_eq!(*stack.read(), 3);
        assert_eq!(*stack.read(), 3);

        stack.pop();
        stack.pop();
        assert_eq!(*stack.read(), 0);
    }
}
