use serde::Deserialize;

/// The one full lynt shape returned by nearly every endpoint (feed, single lynt,
/// comments, search, etc). Endpoints that only need a subset still deserialize
/// into this — the `#[serde(default)]` fields are the ones that are only
/// actually populated on a subset of endpoints (e.g. `parent` / `referenced_lynts`
/// only appear on `GET /lynts/:id`).
#[derive(Debug, Clone, Deserialize)]
pub struct Lynt {
    pub id: String,
    pub content: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "editedAt", default)]
    pub edited_at: Option<String>,
    #[serde(default)]
    pub reposted: bool,
    #[serde(rename = "parentId", default)]
    pub parent_id: Option<String>,
    /// Raw parent lynt id; only present on `GET /lynts/:id` (same value as `parent_id`).
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub has_image: bool,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default)]
    pub gif_url: Option<String>,
    #[serde(default)]
    pub gif_preview_url: Option<String>,
    #[serde(default)]
    pub views: i64,
    #[serde(rename = "likeCount", default)]
    pub like_count: i64,
    #[serde(rename = "repostCount", default)]
    pub repost_count: i64,
    #[serde(rename = "commentCount", default)]
    pub comment_count: i64,
    #[serde(rename = "likedByUser", default)]
    pub liked_by_user: bool,
    #[serde(rename = "repostedByUser", default)]
    pub reposted_by_user: bool,
    #[serde(rename = "likedByFollowed", default)]
    pub liked_by_followed: bool,
    pub handle: String,
    pub username: String,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub iq: Option<i64>,
    #[serde(default)]
    pub verified: bool,
    #[serde(rename = "isAdmin", default)]
    pub is_admin: bool,
    #[serde(default)]
    pub contributor: bool,
    #[serde(rename = "loginStreak", default)]
    pub login_streak: i64,
    #[serde(rename = "followerCount", default)]
    pub follower_count: i64,
    #[serde(rename = "followsViewer", default)]
    pub follows_viewer: bool,
    #[serde(rename = "nameColor", default)]
    pub name_color: Option<String>,
    #[serde(rename = "userCreatedAt", default)]
    pub user_created_at: Option<String>,

    // Populated only when this lynt is itself a reply.
    #[serde(rename = "parentContent", default)]
    pub parent_content: Option<String>,
    #[serde(rename = "parentHasImage", default)]
    pub parent_has_image: Option<bool>,
    #[serde(rename = "parentImages", default)]
    pub parent_images: Option<Vec<String>>,
    #[serde(rename = "parentGifUrl", default)]
    pub parent_gif_url: Option<String>,
    #[serde(rename = "parentGifPreviewUrl", default)]
    pub parent_gif_preview_url: Option<String>,
    #[serde(rename = "parentUserHandle", default)]
    pub parent_user_handle: Option<String>,
    #[serde(rename = "parentUserCreatedAt", default)]
    pub parent_user_created_at: Option<String>,
    #[serde(rename = "parentUserBio", default)]
    pub parent_user_bio: Option<String>,
    #[serde(rename = "parentUserUsername", default)]
    pub parent_user_username: Option<String>,
    #[serde(rename = "parentUserVerified", default)]
    pub parent_user_verified: Option<bool>,
    #[serde(rename = "parentUserIq", default)]
    pub parent_user_iq: Option<i64>,
    #[serde(rename = "parentUserId", default)]
    pub parent_user_id: Option<String>,
    #[serde(rename = "parentCreatedAt", default)]
    pub parent_created_at: Option<String>,
    #[serde(rename = "parentUserNameColor", default)]
    pub parent_user_name_color: Option<String>,

    /// Full ancestor chain, oldest first. Only present on `GET /lynts/:id`.
    #[serde(rename = "referencedLynts", default)]
    pub referenced_lynts: Vec<Lynt>,

    /// Poll attachments aren't creatable over the API, but lynts made via the
    /// app can carry one — left as raw JSON until we've seen a populated example.
    #[serde(default)]
    pub poll: Option<serde_json::Value>,
}

/// `GET /me` — the account tied to the credential, private fields included.
#[derive(Debug, Clone, Deserialize)]
pub struct Me {
    pub id: String,
    pub username: String,
    pub handle: String,
    pub bio: Option<String>,
    pub iq: i64,
    pub created_at: String,
    pub is_admin: bool,
    pub verified: bool,
    pub lynt_coins: i64,
}

/// `GET /users/:handle` — a public profile.
#[derive(Debug, Clone, Deserialize)]
pub struct PublicUser {
    pub id: String,
    pub username: String,
    pub handle: String,
    pub bio: Option<String>,
    pub iq: i64,
    pub created_at: String,
    pub verified: bool,
    pub contributor: bool,
    pub follower_count: i64,
    pub following_count: i64,
    pub followed_by_viewer: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String, // tighten to an enum once we've seen the full set of types
    #[serde(rename = "sourceUserId")]
    pub source_user_id: String,
    #[serde(rename = "sourceUserHandle")]
    pub source_user_handle: String,
    #[serde(rename = "sourceUsername")]
    pub source_username: String,
    #[serde(rename = "lyntId", default)]
    pub lynt_id: Option<String>,
    #[serde(rename = "lyntContent", default)]
    pub lynt_content: Option<String>,
    pub read: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "mentionCount", default)]
    pub mention_count: Option<i64>,
}

/// One of the three feed algorithms for `GET /lynts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedType {
    New,
    Following,
    ForYou,
}

impl FeedType {
    pub(crate) fn as_query_value(&self) -> &'static str {
        match self {
            FeedType::New => "New",
            FeedType::Following => "Following",
            FeedType::ForYou => "For you",
        }
    }
}

/// A scope grantable to a credential — used client-side to pre-check a call
/// before it hits the network, and echoed back in `insufficient_scope` errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    MeRead,
    MeWrite,
    LyntsWrite,
    LikesWrite,
    FollowsWrite,
    NotificationsRead,
}

impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scope::MeRead => "me:read",
            Scope::MeWrite => "me:write",
            Scope::LyntsWrite => "lynts:write",
            Scope::LikesWrite => "likes:write",
            Scope::FollowsWrite => "follows:write",
            Scope::NotificationsRead => "notifications:read",
        }
    }
}

/// Response envelope for endpoints that return `{ "lynts": [...] }`.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LyntsEnvelope {
    pub lynts: Vec<Lynt>,
}

/// Response envelope for endpoints that return `{ "comments": [...] }`.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CommentsEnvelope {
    pub comments: Vec<Lynt>,
}

/// Response envelope for `GET /notifications`.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NotificationsEnvelope {
    pub notifications: Vec<Notification>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LikedEnvelope {
    pub liked: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FollowingEnvelope {
    pub following: bool,
}
