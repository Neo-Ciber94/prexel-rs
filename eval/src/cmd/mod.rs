pub mod commands;
pub mod error;

use std::collections::HashMap;
use std::rc::Rc;

/// Trait for a executable command.
pub trait Command<Arg, R> {
    /// Gets the name of the command.
    fn name(&self) -> &str;
    /// Gets the alias of the command.
    fn alias(&self) -> Option<&str>;
    /// Runs this command and gets a result.
    fn execute(&self, args: &[Arg]) -> R;
}

/// Stores and runs the cmd.
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
    pub fn get(&self, name: &str) -> Option<&Rc<dyn Command<Arg, R>>> {
        self.commands.get(name)
    }

    /// Checks if the `CommandExecutor` contains the specified command.
    #[inline]
    pub fn contains(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    /// Executes the `Command` with the specified name passing the given arguments,
    /// and returns the result of the call, if the command if not found `None` is returned.
    #[inline]
    pub fn run(&self, name: &str, args: &[Arg]) -> Option<R> {
        self.commands
            .get(name)
            .map_or(None, |command| Some(command.execute(args)))
    }
}
