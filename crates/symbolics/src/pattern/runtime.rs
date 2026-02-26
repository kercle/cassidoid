use crate::pattern::program::Program;

pub struct Runtime<'p, A: Clone + PartialEq> {
    program: &'p Program<A>,
    // stacks, choicepoints, etc.
}
