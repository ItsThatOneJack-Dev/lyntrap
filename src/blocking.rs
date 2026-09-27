use crate::model::{
    CommentsEnvelope, FeedType, FollowingEnvelope, LikedEnvelope, Lynt, LyntsEnvelope, Me,
    Notification, NotificationsEnvelope, PublicUser,
};
use crate::request::{self, ApiRequest, Body, Method, MultipartForm};
use crate::{Config, Error, Image};

pub struct APIClient {
    config: Config,
    agent: ureq::Agent,
}

impl APIClient {
    /// Shorthand for `APIClientBuilder::new(client_id, client_secret).build()`.
    /// Use [`APIClient::builder`] instead when you need a custom base URL
    /// (e.g. pointing at a mock server in tests) or a request timeout.
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        APIClientBuilder::new(client_id, client_secret).build()
    }

    pub fn builder(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> APIClientBuilder {
        APIClientBuilder::new(client_id, client_secret)
    }

    fn send(&self, req: ApiRequest) -> Result<ureq::Response, Error> {
        let url = self.config.url(&req.path);
        let scope = req.scope; // capture before req.body gets moved out below
        let mut call = match req.method {
            Method::Get => self.agent.get(&url),
            Method::Post => self.agent.post(&url),
            Method::Put => self.agent.put(&url),
            Method::Delete => self.agent.delete(&url),
            Method::Patch => self.agent.request("PATCH", &url),
        };
        for (k, v) in &req.query {
            call = call.query(k, v);
        }
        call = call
            .set("X-Client-Id", &self.config.client_id)
            .set("X-Client-Secret", &self.config.client_secret);

        let result = match req.body {
            Body::Json(value) => call.send_json(value),
            Body::None => call.call(),
            Body::Multipart(form) => {
                let (body, content_type) = build_multipart_body(&form);
                call.set("Content-Type", &content_type).send_bytes(&body)
            }
        };

        result.map_err(|e| match e {
            ureq::Error::Status(status, resp) => {
                let body = resp.into_string().unwrap_or_default();
                crate::error::error_from_response(status, &body, scope)
            }
            other => Error::Transport(other.to_string()),
        })
    }

    fn send_json<T: serde::de::DeserializeOwned>(&self, req: ApiRequest) -> Result<T, Error> {
        self.send(req)?
            .into_json::<T>()
            .map_err(|e| Error::Transport(e.to_string()))
    }

    pub fn get_me(&self) -> Result<Me, Error> {
        self.send_json(request::get_me())
    }

    pub fn patch_me(
        &self,
        bio: Option<&str>,
        username: Option<&str>,
        name_color: Option<&str>,
    ) -> Result<Me, Error> {
        self.send_json(request::patch_me(bio, username, name_color)?)
    }

    pub fn get_feed(&self, feed_type: FeedType, before: Option<&str>) -> Result<Vec<Lynt>, Error> {
        self.send_json::<LyntsEnvelope>(request::get_feed(feed_type, before))
            .map(|e| e.lynts)
    }

    pub fn get_lynt(&self, id: &str) -> Result<Lynt, Error> {
        self.send_json(request::get_lynt(id))
    }

    pub fn post_lynt(&self, content: &str) -> Result<Lynt, Error> {
        self.send_json(request::post_lynt(content)?)
    }

    pub fn post_lynt_with_images(
        &self,
        content: Option<&str>,
        images: Vec<Image>,
    ) -> Result<Lynt, Error> {
        self.send_json(request::post_lynt_with_images(content, images)?)
    }

    pub fn edit_lynt(&self, id: &str, content: &str) -> Result<Lynt, Error> {
        self.send_json(request::put_lynt(id, content)?)
    }

    pub fn delete_lynt(&self, id: &str) -> Result<(), Error> {
        self.send(request::delete_lynt(id))?;
        Ok(())
    }

    pub fn get_comments(&self, id: &str) -> Result<Vec<Lynt>, Error> {
        self.send_json::<CommentsEnvelope>(request::get_comments(id))
            .map(|e| e.comments)
    }

    pub fn get_all_comments(&self, before: Option<&str>) -> Result<Vec<Lynt>, Error> {
        self.send_json::<CommentsEnvelope>(request::get_all_comments(before))
            .map(|e| e.comments)
    }

    pub fn post_comment(&self, id: &str, content: &str) -> Result<Lynt, Error> {
        self.send_json(request::post_comment(id, content)?)
    }

    pub fn post_comment_with_images(
        &self,
        id: &str,
        content: Option<&str>,
        images: Vec<Image>,
    ) -> Result<Lynt, Error> {
        self.send_json(request::post_comment_with_images(id, content, images)?)
    }

    pub fn like(&self, id: &str) -> Result<bool, Error> {
        self.send_json::<LikedEnvelope>(request::like_lynt(id))
            .map(|e| e.liked)
    }

    pub fn unlike(&self, id: &str) -> Result<bool, Error> {
        self.send_json::<LikedEnvelope>(request::unlike_lynt(id))
            .map(|e| e.liked)
    }

    pub fn get_user(&self, handle: &str) -> Result<PublicUser, Error> {
        self.send_json(request::get_user(handle))
    }

    pub fn follow(&self, handle: &str) -> Result<bool, Error> {
        self.send_json::<FollowingEnvelope>(request::follow_user(handle))
            .map(|e| e.following)
    }

    pub fn unfollow(&self, handle: &str) -> Result<bool, Error> {
        self.send_json::<FollowingEnvelope>(request::unfollow_user(handle))
            .map(|e| e.following)
    }

    pub fn search(&self, q: &str) -> Result<Vec<Lynt>, Error> {
        self.send_json::<LyntsEnvelope>(request::search(q))
            .map(|e| e.lynts)
    }

    pub fn get_notifications(&self) -> Result<Vec<Notification>, Error> {
        self.send_json::<NotificationsEnvelope>(request::get_notifications())
            .map(|e| e.notifications)
    }
}

fn boundary_token() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("lyntrap-{nanos:x}-{n:x}")
}

fn build_multipart_body(form: &MultipartForm) -> (Vec<u8>, String) {
    let boundary = boundary_token();
    let mut body = Vec::new();

    for (name, value) in &form.fields {
        body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        body.extend_from_slice(
            format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
        );
        body.extend_from_slice(value.as_bytes());
        body.extend_from_slice(b"\r\n");
    }

    for file in &form.files {
        body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        body.extend_from_slice(
            format!(
                "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                file.field_name, file.filename
            )
            .as_bytes(),
        );
        body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", file.content_type).as_bytes());
        body.extend_from_slice(&file.data);
        body.extend_from_slice(b"\r\n");
    }

    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    (body, format!("multipart/form-data; boundary={boundary}"))
}

pub struct APIClientBuilder {
    client_id: String,
    client_secret: String,
    base_url: Option<String>,
    timeout: Option<std::time::Duration>,
}

impl APIClientBuilder {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            base_url: None,
            timeout: None,
        }
    }

    /// Override the API base URL — mainly for pointing at a mock server in tests.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> APIClient {
        let mut agent_builder = ureq::AgentBuilder::new();
        if let Some(timeout) = self.timeout {
            agent_builder = agent_builder.timeout(timeout);
        }

        let mut config = Config::new(self.client_id, self.client_secret);
        if let Some(base_url) = self.base_url {
            config.base_url = base_url;
        }

        APIClient {
            config,
            agent: agent_builder.build(),
        }
    }
}
