use std::process::Command;
pub struct AccountCommand {
    command: Command,
}

impl AccountCommand {
    pub fn new(command: Command) -> Self {
        Self { command }
    }

    pub fn public_key<S: Into<String>>(mut self, public_key: S) -> Self {
        self.command.arg(public_key.into());
        self
    }

    pub fn test_discrimination(mut self) -> Self {
        self.command.arg("--testing");
        self
    }

    pub fn prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.command.arg("--prefix").arg(prefix.into());
        self
    }

    pub fn build(self) -> Command {
        self.command
    }
}
