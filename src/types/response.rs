pub enum DiscordEvents {
    TextEvent(String),
    RefreshData,
    StopData,
    Exit,
}

pub struct Response {
    pub discord: Vec<DiscordEvents>,
    pub server: Vec<String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            discord: Vec::new(),
            server: Vec::new(),
        }
    }

    pub fn text<T>(self, text: T) -> Self
    where
        T: AsRef<str>,
    {
        let mut discord = self.discord;
        discord.push(DiscordEvents::TextEvent(text.as_ref().to_owned()));

        Self { discord, ..self }
    }

    pub fn exit(self) -> Self {
        let mut discord = self.discord;
        discord.push(DiscordEvents::Exit);

        Self { discord, ..self }
    }

    pub fn refresh_data(self) -> Self {
        let mut discord = self.discord;
        discord.push(DiscordEvents::RefreshData);

        Self { discord, ..self }
    }

    pub fn stop_data(self) -> Self {
        let mut discord = self.discord;
        discord.push(DiscordEvents::StopData);

        Self { discord, ..self }
    }
}
