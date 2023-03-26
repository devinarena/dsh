use std::collections::HashSet;

pub trait Command {
    fn get_aliases(&self) -> HashSet<&str>;
    fn execute(&self);
}
