use serde_json::{Value, json};

use crate::Error;
use crate::media::Image;
use crate::model::FeedType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

pub(crate) enum Body {
    Json(Value),
    Multipart(MultipartForm),
    None,
}

pub(crate) struct MultipartForm {
    pub fields: Vec<(&'static str, String)>,
    pub files: Vec<MultipartFile>,
}

pub(crate) struct MultipartFile {
    pub field_name: &'static str,
    pub filename: String,
    pub data: Vec<u8>,
    pub content_type: String,
}

pub(crate) struct ApiRequest {
    pub method: Method,
    pub path: String,
    pub query: Vec<(&'static str, String)>,
    pub body: Body,
    pub scope: Option<&'static str>,
}

fn validate_content_len(content: &str, min: usize, max: usize) -> Result<(), Error> {
    let len = content.chars().count();
    if len < min || len > max {
        return Err(Error::Validation(format!(
            "content must be {min}-{max} characters, got {len}"
        )));
    }
    Ok(())
}

/// Shared by `post_lynt`/`post_comment` and their `_with_images` variants:
/// content can be empty only if at least one image is attached.
fn validate_lynt_content(content: Option<&str>, image_count: usize) -> Result<String, Error> {
    if image_count > 4 {
        return Err(Error::Validation("at most 4 images are allowed".into()));
    }
    let content = content.unwrap_or("").to_string();
    validate_content_len(&content, 0, 280)?;
    if content.is_empty() && image_count == 0 {
        return Err(Error::Validation(
            "content is required unless at least one image is attached".into(),
        ));
    }
    Ok(content)
}

fn images_to_files(images: Vec<Image>) -> Vec<MultipartFile> {
    images
        .into_iter()
        .map(|img| MultipartFile {
            field_name: "images",
            content_type: img.content_type(),
            filename: img.filename.clone(),
            data: img.data.clone(),
        })
        .collect()
}

// --- Profile ---

pub(crate) fn get_me() -> ApiRequest {
    ApiRequest {
        method: Method::Get,
        path: "/me".to_string(),
        query: Vec::new(),
        body: Body::None,
        scope: Some("me:read"),
    }
}

pub(crate) fn patch_me(
    bio: Option<&str>,
    username: Option<&str>,
    name_color: Option<&str>,
) -> Result<ApiRequest, Error> {
    if let Some(bio) = bio {
        validate_content_len(bio, 0, 256)?;
    }
    if let Some(username) = username {
        validate_content_len(username, 1, 60)?;
    }
    let mut body = serde_json::Map::new();
    if let Some(bio) = bio {
        body.insert("bio".to_string(), json!(bio));
    }
    if let Some(username) = username {
        body.insert("username".to_string(), json!(username));
    }
    if let Some(name_color) = name_color {
        body.insert("name_color".to_string(), json!(name_color));
    }
    Ok(ApiRequest {
        method: Method::Patch,
        path: "/me".to_string(),
        query: Vec::new(),
        body: Body::Json(Value::Object(body)),
        scope: Some("me:write"),
    })
}

// --- Feed / Lynts ---

pub(crate) fn get_feed(feed_type: FeedType, before: Option<&str>) -> ApiRequest {
    let mut query = vec![("type", feed_type.as_query_value().to_string())];
    if let Some(before) = before {
        query.push(("before", before.to_string()));
    }
    ApiRequest {
        method: Method::Get,
        path: "/lynts".to_string(),
        query,
        body: Body::None,
        scope: None,
    }
}

pub(crate) fn get_lynt(id: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Get,
        path: format!("/lynts/{id}"),
        query: Vec::new(),
        body: Body::None,
        scope: None,
    }
}

pub(crate) fn post_lynt(content: &str) -> Result<ApiRequest, Error> {
    let content = validate_lynt_content(Some(content), 0)?;
    Ok(ApiRequest {
        method: Method::Post,
        path: "/lynts".to_string(),
        query: Vec::new(),
        body: Body::Json(json!({ "content": content })),
        scope: Some("lynts:write"),
    })
}

pub(crate) fn post_lynt_with_images(
    content: Option<&str>,
    images: Vec<Image>,
) -> Result<ApiRequest, Error> {
    let content = validate_lynt_content(content, images.len())?;
    let mut fields = Vec::new();
    if !content.is_empty() {
        fields.push(("content", content));
    }
    Ok(ApiRequest {
        method: Method::Post,
        path: "/lynts".to_string(),
        query: Vec::new(),
        body: Body::Multipart(MultipartForm {
            fields,
            files: images_to_files(images),
        }),
        scope: Some("lynts:write"),
    })
}

pub(crate) fn put_lynt(id: &str, content: &str) -> Result<ApiRequest, Error> {
    validate_content_len(content, 1, 280)?;
    Ok(ApiRequest {
        method: Method::Put,
        path: format!("/lynts/{id}"),
        query: Vec::new(),
        body: Body::Json(json!({ "content": content })),
        scope: Some("lynts:write"),
    })
}

pub(crate) fn delete_lynt(id: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Delete,
        path: format!("/lynts/{id}"),
        query: Vec::new(),
        body: Body::None,
        scope: Some("lynts:write"),
    }
}

// --- Comments ---

pub(crate) fn get_comments(id: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Get,
        path: format!("/lynts/{id}/comments"),
        query: Vec::new(),
        body: Body::None,
        scope: None,
    }
}

pub(crate) fn get_all_comments(before: Option<&str>) -> ApiRequest {
    let query = before
        .map(|b| vec![("before", b.to_string())])
        .unwrap_or_default();
    ApiRequest {
        method: Method::Get,
        path: "/lynts/all/comments".to_string(),
        query,
        body: Body::None,
        scope: None,
    }
}

pub(crate) fn post_comment(id: &str, content: &str) -> Result<ApiRequest, Error> {
    let content = validate_lynt_content(Some(content), 0)?;
    Ok(ApiRequest {
        method: Method::Post,
        path: format!("/lynts/{id}/comments"),
        query: Vec::new(),
        body: Body::Json(json!({ "content": content })),
        scope: Some("lynts:write"),
    })
}

pub(crate) fn post_comment_with_images(
    id: &str,
    content: Option<&str>,
    images: Vec<Image>,
) -> Result<ApiRequest, Error> {
    let content = validate_lynt_content(content, images.len())?;
    let mut fields = Vec::new();
    if !content.is_empty() {
        fields.push(("content", content));
    }
    Ok(ApiRequest {
        method: Method::Post,
        path: format!("/lynts/{id}/comments"),
        query: Vec::new(),
        body: Body::Multipart(MultipartForm {
            fields,
            files: images_to_files(images),
        }),
        scope: Some("lynts:write"),
    })
}

// --- Likes ---

pub(crate) fn like_lynt(id: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Post,
        path: format!("/lynts/{id}/like"),
        query: Vec::new(),
        body: Body::None,
        scope: Some("lynts:write"),
    }
}

pub(crate) fn unlike_lynt(id: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Delete,
        path: format!("/lynts/{id}/like"),
        query: Vec::new(),
        body: Body::None,
        scope: Some("lynts:write"),
    }
}

// --- Users ---

pub(crate) fn get_user(handle: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Get,
        path: format!("/users/{handle}"),
        query: Vec::new(),
        body: Body::None,
        scope: None,
    }
}

pub(crate) fn follow_user(handle: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Post,
        path: format!("/users/{handle}/follow"),
        query: Vec::new(),
        body: Body::None,
        scope: Some("follows:write"),
    }
}

pub(crate) fn unfollow_user(handle: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Delete,
        path: format!("/users/{handle}/follow"),
        query: Vec::new(),
        body: Body::None,
        scope: Some("follows:write"),
    }
}

// --- Search ---

pub(crate) fn search(q: &str) -> ApiRequest {
    ApiRequest {
        method: Method::Get,
        path: "/search".to_string(),
        query: vec![("q", q.to_string())],
        body: Body::None,
        scope: None,
    }
}

// --- Notifications ---

pub(crate) fn get_notifications() -> ApiRequest {
    ApiRequest {
        method: Method::Get,
        path: "/notifications".to_string(),
        query: Vec::new(),
        body: Body::None,
        scope: Some("notifications:read"),
    }
}
