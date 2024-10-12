pub struct Command {
    command: String,
    arg_count: usize,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        assert!(
            is_valid_command_name(&name),
            "string cannot be used as command name"
        );
        Self {
            command: name + "(",
            arg_count: 0,
        }
    }

    pub fn arg(mut self, arg: impl Arg) -> Self {
        if self.arg_count > 0 {
            self.command += ",";
        }
        arg.push_to_command(&mut self.command);
        self.arg_count += 1;
        self
    }

    pub fn build(self) -> String {
        self.command + ")\n"
    }
}

pub trait Arg {
    fn push_to_command(self, command: &mut String);
}

impl Arg for &str {
    fn push_to_command(self, command: &mut String) {
        for ch in self.chars() {
            match ch {
                '\n' => command.push(' '),
                '\t' | '\x20'..='\x7e' => command.push(ch),
                _ => (),
            }
        }
    }
}

impl Arg for i32 {
    fn push_to_command(self, command: &mut String) {
        command.push_str(&self.to_string());
    }
}

fn is_valid_command_name(name: &str) -> bool {
    if name.len() < 1 {
        return false;
    }
    let mut last = '\0';
    for (i, ch) in name.chars().enumerate() {
        if !matches!(ch, 'a'..='z'|'A'..='Z'|'.') {
            return false;
        }
        if i == 0 && ch == '.' {
            return false;
        }
        last = ch;
    }
    if last == '.' {
        return false;
    }
    true
}
