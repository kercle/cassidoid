use crate::pattern::program::Program;

pub struct Runtime<'p, A> {
    program: &'p Program<A>,
    // stacks, choicepoints, etc.
}
