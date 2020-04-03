use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::Borrow;
use std::ops::Index;
use std::slice::{SliceIndex, Iter};

/// Trait for a executable command.
pub trait Command<Arg, R> {
    /// Gets the name of the command.
    fn name(&self) -> &str;
    /// Gets the alias of the command.
    fn alias(&self) -> Option<&str>;
    /// Gets help information about this command;
    fn help_info(&self) -> &str;
    /// Runs this command and gets a result.
    fn execute(&self, args: CommandArgs<'_, Arg>) -> R;
}

/// Stores and runs the commands.
pub struct CommandExecutor<Arg, R> {
    commands: HashMap<String, Rc<dyn Command<Arg, R>>>,
}

impl<Arg, R> CommandExecutor<Arg, R> {
    /// Constructs a new `Executor`.
    #[inline]
    pub fn new() -> Self {
        CommandExecutor {
            commands: Default::default(),
        }
    }

    /// Adds a new command to the `Executor`.
    ///
    /// # Panics
    /// If the command already exists.
    pub fn add<C: Command<Arg, R> + 'static>(&mut self, command: C) {
        if self.commands.contains_key(command.name()) {
            panic!(
                "Executor already contains a command named `{}`",
                command.name()
            );
        }

        let name = command.name().to_string();
        let alias = command.alias().map(|s| s.to_string());
        let c = Rc::new(command);

        self.commands.insert(name, c.clone());
        alias.map(|s| self.commands.insert(s, c.clone()));
    }

    /// Gets the command with the specified name.
    #[inline]
    pub fn get(&self, command: &str) -> Option<&Rc<dyn Command<Arg, R>>> {
        self.commands.get(command)
    }

    /// Checks if the `CommandExecutor` contains the specified command.
    #[inline]
    pub fn contains(&self, command: &str) -> bool {
        self.commands.contains_key(command)
    }

    /// Gets help information about the given command.
    #[inline]
    pub fn help(&self, command: &str) -> Option<&str>{
        self.get(command)
            .map(|c| c.help_info())
    }

    /// Executes the `Command` with the specified name passing the given arguments,
    /// and returns the result of the call, if the command if not found `None` is returned.
    #[inline]
    pub fn exec(&self, command: &str, args: CommandArgs<'_, Arg>) -> Option<R> {
        self.get(command)
            .map_or(None, |command| Some(command.execute(args)))
    }
}

impl<Arg, R> Default for CommandExecutor<Arg, R>{
    #[inline]
    fn default() -> Self {
        CommandExecutor::new()
    }
}

/// Stores the arguments of a command.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommandArgs<'a, T>(pub &'a [T]);

impl<'a, T> CommandArgs<'a, T>{
    #[inline]
    pub fn new(args: &'a [T]) -> Self{
        CommandArgs(args)
    }

    #[inline]
    pub fn len(&self) -> usize{
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool{
        self.0.is_empty()
    }

    pub fn contains<U: ?Sized>(&self, value: &U) -> bool
        where T: Borrow<U>,
              U: Eq {

        for item in self.0.iter(){
            if item.borrow() == value{
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn iter(&self) -> Iter<'a, T>{
        self.0.iter()
    }
}

impl<'a, T, I: SliceIndex<[T]>> Index<I> for CommandArgs<'a, T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(self.0, index)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn command_args_indexer_test(){
        let arr = &[1, 2, 3];
        let args = CommandArgs::new(arr);

        assert_eq!(args[0], 1);
        assert_eq!(args[1], 2);
        assert_eq!(args[2], 3);
    }

    #[test]
    fn command_args_contains_test(){
        let arr = [String::from("one"), String::from("two"), String::from("three")];
        let args = CommandArgs::new(&arr);

        assert!(args.contains(&String::from("one")));
        assert!(args.contains("two"));
        assert!(args.contains("three"));
    }
}