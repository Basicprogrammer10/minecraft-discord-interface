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

    pub fn discord_text<T>(self, text: T) -> Self
    where
        T: AsRef<str>,
    {
        let mut discord = self.discord;
        discord.push(DiscordEvents::TextEvent(text.as_ref().to_owned()));

        Self { discord, ..self }
    }

    pub fn async_exit(self) -> Self {
        let mut discord = self.discord;
        discord.push(DiscordEvents::Exit);

        Self { discord, ..self }
    }

    pub fn discord_refresh_data(self) -> Self {
        let mut discord = self.discord;
        discord.push(DiscordEvents::RefreshData);

        Self { discord, ..self }
    }

    pub fn discord_stop_data(self) -> Self {
        let mut discord = self.discord;
        discord.push(DiscordEvents::StopData);

        Self { discord, ..self }
    }

    pub fn server_command<T>(self, command: T) -> Self
    where
        T: AsRef<str>,
    {
        let mut server = self.server;
        server.push(command.as_ref().to_owned());

        Self { server, ..self }
    }
}
