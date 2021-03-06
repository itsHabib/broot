use {regex::Regex, std::fmt};

/// the verb and its arguments, making the invocation.
/// When coming from parsing, the args is Some as soon
/// as there's a separator (i.e. it's "" in "cp ")
#[derive(Clone, Debug, PartialEq)]
pub struct VerbInvocation {
    pub name: String,
    pub args: Option<String>,
    pub bang: bool,
}

impl fmt::Display for VerbInvocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":")?;
        if self.bang {
            write!(f, "!")?;
        }
        write!(f, "{}", &self.name)?;
        if let Some(args) = &self.args {
            write!(f, " {}", &args)?;
        }
        Ok(())
    }
}

impl VerbInvocation {
    pub fn new<T: Into<String>>(name: T, args: Option<T>, bang: bool) -> Self {
        Self {
            name: name.into(),
            args: args.map(|s| s.into()),
            bang,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
    /// build a new String
    pub fn complete_name(&self) -> String {
        if self.bang {
            format!("{}_tab", &self.name)
        } else {
            self.name.clone()
        }
    }
    /// basically return the invocation but allow another name (the shortcut
    /// or a variant)
    pub fn to_string_for_name(&self, name: &str) -> String {
        let mut s = String::new();
        if self.bang {
            s.push('!');
        }
        s.push_str(name);
        if let Some(args) = &self.args {
            s.push(' ');
            s.push_str(&args);
        }
        s
    }
}

impl From<&str> for VerbInvocation {
    /// parse a string being or describing the invocation of a verb with its
    /// arguments and optional bang. The leading space or colon must
    /// have been stripped before.
    fn from(invocation: &str) -> Self {
        let caps = regex!(
            r"(?x)
                ^
                (?P<bang_before>!)?
                (?P<name>[^!\s]*)
                (?P<bang_after>!)?
                (?:[\s:]+(?P<args>.*))?
                \s*
                $
            "
        )
        .captures(invocation)
        .unwrap();
        let bang_before = caps.name("bang_before").is_some();
        let bang_after = caps.name("bang_after").is_some();
        let bang = bang_before || bang_after;
        let name = caps.name("name").unwrap().as_str().to_string();
        let args = caps.name("args").map(|c| c.as_str().to_string());
        VerbInvocation { name, args, bang }
    }
}

#[cfg(test)]
mod verb_invocation_tests {
    use super::*;
    #[test]
    fn check_verb_invocation_parsing() {
        assert_eq!(
            VerbInvocation::from("!mv"),
            VerbInvocation::new("mv", None, true),
        );
        assert_eq!(
            VerbInvocation::from("mva!"),
            VerbInvocation::new("mva", None, true),
        );
        assert_eq!(
            VerbInvocation::from("cp "),
            VerbInvocation::new("cp", Some(""), false),
        );
        assert_eq!(
            VerbInvocation::from("cp ../"),
            VerbInvocation::new("cp", Some("../"), false),
        );
    }
}
