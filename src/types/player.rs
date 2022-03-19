use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
    pub online: bool,
    pub bot: bool,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            online: true,
            bot: false,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", if self.bot { "🤖 " } else { "" }, self.name)
    }
}
