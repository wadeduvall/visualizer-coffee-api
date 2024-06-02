use reqwest::{Client, Error, RequestBuilder};

pub trait Builder {
    fn build<T: for<'a> serde::Deserialize<'a>>(
        self,
    ) -> impl std::future::Future<Output = Result<T, Error>> + Send;
}

pub struct ClientBuilder {
    username: String,
    password: String,
    backend: Client,
}

pub struct ShotBuilder {
    shot_id: String,
    essentials: bool,
    public: bool,
    backend: RequestBuilder,
}

impl ShotBuilder {
    const fn new(shot_id: String, backend: RequestBuilder) -> Self {
        Self {
            shot_id,
            essentials: false,
            public: false,
            backend,
        }
    }

    #[must_use]
    pub const fn essentials(mut self) -> Self {
        self.essentials = true;
        self
    }

    #[must_use]
    pub const fn public(mut self) -> Self {
        self.public = true;
        self
    }
}

impl Builder for ShotBuilder {
    async fn build<Shot: for<'de> serde::Deserialize<'de>>(self) -> Result<Shot, Error> {
        let mut builder = self.backend;
        if self.essentials {
            let params = [("essentials", self.essentials)];
            builder = builder.form(&params);
        }
        let response = builder.send().await?;
        let shot: Shot = response.json::<Shot>().await?;
        Ok(shot)
    }
}

pub struct ShotListBuilder {
    essencials: bool,
    public: bool,
}

impl ClientBuilder {
    /// Constructs a new Client.
    /// # Panics
    /// Panics if `request::Client::new()` panics.
    #[must_use]
    pub fn connect(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
            backend: reqwest::Client::new(),
        }
    }

    #[must_use]
    pub fn shot(self, shot_id: &str) -> ShotBuilder {
        ShotBuilder::new(
            String::from(shot_id),
            self.backend
                .get(format!(
                    "https://visualizer.coffee/api/shots/{shot_id}/download"
                ))
                .basic_auth(self.username, Some(self.password)),
        )
    }

    /// Get users shots
    /// # Errors
    /// This method will return the `reqwest::Error` if it fails.
    pub async fn get_shots(self, params: Option<Vec<(&str, &str)>>) -> Result<String, Error> {
        let mut builder = self
            .backend
            .get("https://visualizer.coffee/api/shots/")
            .basic_auth(self.username, Some(self.password));
        if let Some(p) = params {
            builder = builder.form(&p);
        }
        let result = builder.send().await?.text().await?;
        Ok(result)
    }
}
