use reqwest::Method as ReqwestMethod;

use crate::model::{
    CommentsEnvelope, FeedType, FollowingEnvelope, LikedEnvelope, Lynt, LyntsEnvelope, Me,
    Notification, NotificationsEnvelope, PublicUser,
};
use crate::request::{self, ApiRequest, Body, Method, MultipartForm};
use crate::{Config, Error, Image};

pub struct APIClient {
    config: Config,
    http: reqwest::Client,
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

    async fn send(&self, req: ApiRequest) -> Result<reqwest::Response, Error> {
        let url = self.config.url(&req.path);
        let method = match req.method {
            Method::Get => ReqwestMethod::GET,
            Method::Post => ReqwestMethod::POST,
            Method::Put => ReqwestMethod::PUT,
            Method::Delete => ReqwestMethod::DELETE,
            Method::Patch => ReqwestMethod::PATCH,
        };
        let mut builder = self
            .http
            .request(method, url)
            .query(&req.query)
            .header("X-Client-Id", &self.config.client_id)
            .header("X-Client-Secret", &self.config.client_secret);

        builder = match req.body {
            Body::Json(value) => builder.json(&value),
            Body::None => builder,
            Body::Multipart(form) => builder.multipart(build_multipart_form(form)?),
        };

        let resp = builder
            .send()
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::error_from_response(status, &body));
        }
        Ok(resp)
    }

    async fn send_json<T: serde::de::DeserializeOwned>(&self, req: ApiRequest) -> Result<T, Error> {
        self.send(req)
            .await?
            .json::<T>()
            .await
            .map_err(|e| Error::Transport(e.to_string()))
    }

    pub async fn get_me(&self) -> Result<Me, Error> {
        self.send_json(request::get_me()).await
    }

    pub async fn patch_me(
        &self,
        bio: Option<&str>,
        username: Option<&str>,
        name_color: Option<&str>,
    ) -> Result<Me, Error> {
        self.send_json(request::patch_me(bio, username, name_color)?)
            .await
    }

    pub async fn get_feed(
        &self,
        feed_type: FeedType,
        before: Option<&str>,
    ) -> Result<Vec<Lynt>, Error> {
        self.send_json::<LyntsEnvelope>(request::get_feed(feed_type, before))
            .await
            .map(|e| e.lynts)
    }

    pub async fn get_lynt(&self, id: &str) -> Result<Lynt, Error> {
        self.send_json(request::get_lynt(id)).await
    }

    pub async fn post_lynt(&self, content: &str) -> Result<Lynt, Error> {
        self.send_json(request::post_lynt(content)?).await
    }

    pub async fn post_lynt_with_images(
        &self,
        content: Option<&str>,
        images: Vec<Image>,
    ) -> Result<Lynt, Error> {
        self.send_json(request::post_lynt_with_images(content, images)?)
            .await
    }

    pub async fn edit_lynt(&self, id: &str, content: &str) -> Result<Lynt, Error> {
        self.send_json(request::put_lynt(id, content)?).await
    }

    pub async fn delete_lynt(&self, id: &str) -> Result<(), Error> {
        self.send(request::delete_lynt(id)).await?;
        Ok(())
    }

    pub async fn get_comments(&self, id: &str) -> Result<Vec<Lynt>, Error> {
        self.send_json::<CommentsEnvelope>(request::get_comments(id))
            .await
            .map(|e| e.comments)
    }

    pub async fn get_all_comments(&self, before: Option<&str>) -> Result<Vec<Lynt>, Error> {
        self.send_json::<CommentsEnvelope>(request::get_all_comments(before))
            .await
            .map(|e| e.comments)
    }

    pub async fn post_comment(&self, id: &str, content: &str) -> Result<Lynt, Error> {
        self.send_json(request::post_comment(id, content)?).await
    }

    pub async fn post_comment_with_images(
        &self,
        id: &str,
        content: Option<&str>,
        images: Vec<Image>,
    ) -> Result<Lynt, Error> {
        self.send_json(request::post_comment_with_images(id, content, images)?)
            .await
    }

    pub async fn like(&self, id: &str) -> Result<bool, Error> {
        self.send_json::<LikedEnvelope>(request::like_lynt(id))
            .await
            .map(|e| e.liked)
    }

    pub async fn unlike(&self, id: &str) -> Result<bool, Error> {
        self.send_json::<LikedEnvelope>(request::unlike_lynt(id))
            .await
            .map(|e| e.liked)
    }

    pub async fn get_user(&self, handle: &str) -> Result<PublicUser, Error> {
        self.send_json(request::get_user(handle)).await
    }

    pub async fn follow(&self, handle: &str) -> Result<bool, Error> {
        self.send_json::<FollowingEnvelope>(request::follow_user(handle))
            .await
            .map(|e| e.following)
    }

    pub async fn unfollow(&self, handle: &str) -> Result<bool, Error> {
        self.send_json::<FollowingEnvelope>(request::unfollow_user(handle))
            .await
            .map(|e| e.following)
    }

    pub async fn search(&self, q: &str) -> Result<Vec<Lynt>, Error> {
        self.send_json::<LyntsEnvelope>(request::search(q))
            .await
            .map(|e| e.lynts)
    }

    pub async fn get_notifications(&self) -> Result<Vec<Notification>, Error> {
        self.send_json::<NotificationsEnvelope>(request::get_notifications())
            .await
            .map(|e| e.notifications)
    }
}

fn build_multipart_form(form: MultipartForm) -> Result<reqwest::multipart::Form, Error> {
    let mut multipart = reqwest::multipart::Form::new();
    for (name, value) in form.fields {
        multipart = multipart.text(name, value);
    }
    for file in form.files {
        let part = reqwest::multipart::Part::bytes(file.data)
            .file_name(file.filename)
            .mime_str(&file.content_type)
            .map_err(|e| Error::Validation(format!("invalid image content type: {e}")))?;
        multipart = multipart.part(file.field_name, part);
    }
    Ok(multipart)
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
        let mut http_builder = reqwest::Client::builder();
        if let Some(timeout) = self.timeout {
            http_builder = http_builder.timeout(timeout);
        }

        let mut config = Config::new(self.client_id, self.client_secret);
        if let Some(base_url) = self.base_url {
            config.base_url = base_url;
        }

        APIClient {
            config,
            // the only fallible step in reqwest::Client::builder().build() is
            // TLS backend init, which our fixed feature set guarantees works —
            // safe to unwrap rather than push a Result through every builder.
            http: http_builder
                .build()
                .expect("failed to build reqwest client"),
        }
    }
}
