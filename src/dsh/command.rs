use std::collections::HashSet;

use super::shell::dsh;

pub trait Command {
    fn get_aliases(&self) -> HashSet<&str>;
    fn execute(&self, dsh: &dsh, args: Vec<&str>);
    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
}
