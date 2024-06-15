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
    username: String,
    password: String,
    essentials: bool,
    code: Option<String>,
    backend: RequestBuilder,
}

impl ShotBuilder {
    const fn new(username: String, password: String, backend: RequestBuilder) -> Self {
        Self {
            username,
            password,
            essentials: false,
            code: None,
            backend,
        }
    }

    #[must_use]
    pub const fn essentials(mut self) -> Self {
        self.essentials = true;
        self
    }

    #[must_use]
    pub fn shared(mut self, code: &str) -> Self {
        self.code = Some(String::from(code));
        self
    }
}

impl Builder for ShotBuilder {
    async fn build<Shot: for<'de> serde::Deserialize<'de>>(self) -> Result<Shot, Error> {
        let mut builder = self.backend;
        match self.code {
            Some(code) => {
                let params = [("code", code)];
                // NOTE: I really don't like this here and points to shortcomings in the upstream
                // API, or maybe my abstraction.
                builder = reqwest::Client::new()
                    .get("https://visualizer.coffee/api/shots/shared")
                    .form(&params);
            }
            None => builder = builder.basic_auth(self.username, Some(self.password)),
        }
        if self.essentials {
            let params = [("essentials", self.essentials)];
            builder = builder.form(&params);
        }
        let response = builder.send().await?;
        response.json::<Shot>().await
    }
}

pub struct ShotListBuilder {
    username: String,
    password: String,
    public: bool,
    page: u32,
    item: u32,
    backend: RequestBuilder,
}

impl ShotListBuilder {
    const fn new(username: String, password: String, backend: RequestBuilder) -> Self {
        Self {
            username,
            password,
            public: false,
            page: 1,
            item: 10,
            backend,
        }
    }

    #[must_use]
    pub const fn public(mut self) -> Self {
        self.public = true;
        self
    }
}

impl Builder for ShotListBuilder {
    async fn build<ShotList: for<'de> serde::Deserialize<'de>>(self) -> Result<ShotList, Error> {
        let mut builder = self.backend;
        if !self.public {
            builder = builder.basic_auth(self.username, Some(self.password));
        }
        let params = [("page", self.page), ("items", self.item)];
        builder = builder.form(&params);
        let response = builder.send().await?;
        response.json::<ShotList>().await
    }
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
            self.username,
            self.password,
            self.backend.get(format!(
                "https://visualizer.coffee/api/shots/{shot_id}/download"
            )),
        )
    }

    #[must_use]
    pub fn shot_list(self) -> ShotListBuilder {
        ShotListBuilder::new(
            self.username,
            self.password,
            self.backend.get("https://visualizer.coffee/api/shots"),
        )
    }
}
