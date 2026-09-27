# Developer API

> [!IMPORTANT]
> This file is a manually-written copy of the Lyntr API documentation as of the 26th of September, 2026.
> The state of Lyntr's developer API documentation on the 27th of September, 2026, is [available on the Internet Archive's Wayback Machine](https://web.archive.org/web/20260927000839/https://lyntr.gizmowizard.tech/developer).
>
> As this file is manually written, and was partially edited afterwards to ensure easier readability, it is possible that some information is omitted entirely.
> Please keep note of this, and use the Lyntr developer documentation directly if editing this project, or alternatively just update this document to reflect the current details from Lyntr's API documentation!

Build on top of Lyntr with a REST API authenticated by a client ID and secret. Docs are below.

## Documentation

Base URL: `https://lyntr.gizmowizard.tech/api/v2`

### Authentication

Authenticate every request with your client ID and secret, either as HTTP Basic auth (`Authorization: Basic base64(client_id:client_secret)`) or as the `X-Client-Id` / `X-Client-Secret` headers.

```bash
# Basic auth
curl https://lyntr.gizmowizard.tech/api/v2/me \
  -u "$LYNTR_CLIENT_ID:$LYNTR_CLIENT_SECRET"

# or explicit headers
curl https://lyntr.gizmowizard.tech/api/v2/me \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

### Scopes

Scopes limit what a credential can do. Endpoints that write to your account or read private data need a scope; public reads (feed, lynts, comments, profiles, search) only need a valid credential. Pick scopes when you create a credential and change them any time from the credentials panel above.

Credentials created before scopes existed were granted every scope, so existing integrations keep working until you narrow them. Calling a scoped endpoint without the scope returns a 403:

```json
{
  "error": "insufficient_scope",
  "required_scope": "lynts:write",
  "message": "This credential is missing the \"lynts:write\" scope. Add it to the credential on the Developer page."
}
```

#### `me:read`

Your private account details, including LyntCoins and admin flag.

Available endpoints: `GET /me`

#### `me:write` [WRITE]

Change your bio, username, and name color.

Available endpoints: `PATCH /me`

#### `lynts:write` [WRITE]

Create, edit, and delete lynts, and post comments on your behalf.

Available endpoints: `POST /lynts`, `PUT /lynts/:id`, `DELETE /lynts/:id`, `POST /lynts/:id/comments`

#### `likes:write` [WRITE]

Like and unlike lynts as you.

Available endpoints: `POST /lynts/:id/like`, `DELETE /lynts/:id/like`

#### `follows:write` [WRITE]

Follow and unfollow users as you.

Available endpoints: `POST /users/:handle/follow`, `DELETE /users/:handle/follow`

#### `notifications:read` [WRITE]

Your private notification feed.

Available endpoints: `GET /notifications`

### What's new in v2

- Images are now accepted on `POST /lynts` and `POST /lynts/:id/comments` — send `multipart/form-data` with an `images` field (up to 4 files) instead of JSON. Plain JSON still works for text-only posts, unchanged from v1.
- New endpoint: `GET /lynts/all/comments` — the most recent comments across every lynt, not scoped to one parent. Requested by @libhmrc6.

v1 (`/api/v1`) is unchanged and still works for existing integrations — v2 is additive, not a breaking migration. Scopes apply to both versions.

GIF and poll attachments aren't supported over the API. Lynts are up to 280 characters, with up to 4 images on `POST /lynts` and POST `/lynts/:id/comments`.

### API structure

A pathing list of all v2 endpoints (legacy v1 endpoints still work).

```ascii
/api/v2
│
├── lynts
│   ├── GET    /lynts
│   ├── POST   /lynts
│   ├── :id
│   │   ├── GET    /lynts/:id
│   │   ├── PUT    /lynts/:id
│   │   ├── DELETE /lynts/:id
│   │   ├── comments
│   │   │   ├── GET  /lynts/:id/comments
│   │   │   └── POST /lynts/:id/comments
│   │   └── like
│   │       ├── POST   /lynts/:id/like
│   │       └── DELETE /lynts/:id/like
│   │
│   └── all
│       └── comments
│           └── GET /lynts/all/comments
│
├── me
│   ├── GET   /me
│   └── PATCH /me
│
├── notifications
│   └── GET /notifications
│
├── search
│   └── GET /search
│
└── users
    └── :handle
        ├── GET    /users/:handle
        ├── POST   /users/:handle/follow
        └── DELETE /users/:handle/follow
```

## Endpoints

### Profile

#### `GET /me`

<sub>Requires the `me:read` scope.</sub>
Returns the account tied to the credential making the request.

Request:

```bash
curl https://lyntr.gizmowizard.tech/api/v2/me \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{
  "id": "7331021...",
  "username": "dani",
  "handle": "dani",
  "bio": "building lyntr",
  "iq": 142,
  "created_at": "2025-02-11T08:15:00.000Z",
  "is_admin": false,
  "verified": true,
  "lynt_coins": 4820
}
```

#### `PATCH /me`

<sub>Requires the `me:write` scope.</sub>

Text-only profile customization. Supports bio, username, and name_color — the same fields the app itself lets you edit without a file upload. Avatar, banner, and profile-song uploads still aren’t available over the API — v2 added multipart support to lynt/comment creation specifically, not to every endpoint.

> bio: string, max 256 chars · username: string, max 60 chars · name_color: hex string, requires a verified account (403 otherwise).

Request:

```bash
curl -X PATCH https://lyntr.gizmowizard.tech/api/v2/me \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET" \
  -H "Content-Type: application/json" \
  -d '{
    "bio": "building lyntr",
    "name_color": "#5470ff"
  }'
```

Response:

```json
{
  "id": "7331021...",
  "username": "dani",
  "handle": "dani",
  "bio": "building lyntr",
  "name_color": "#5470ff",
  "verified": true
}
```

### Lynts

#### `GET /lynts`

<sub>No scopes required.</sub>

A page of lynts from one of the three feed algorithms.

> type: one of "New" (default), "Following", or "For you". Every lynt object has this full shape — the parent* fields are only populated when the lynt is a reply/comment, and poll is only populated when the lynt has one attached.

Request:

```bash
curl "https://lyntr.gizmowizard.tech/api/v2/lynts?type=New" \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{
  "lynts": [
    {
      "id": "7331099...",
      "content": "just shipped the v2 api",
      "userId": "7331021...",
      "createdAt": "2026-07-19T10:02:11.000Z",
      "editedAt": null,
      "reposted": false,
      "parentId": null,
      "has_image": false,
      "images": [],
      "gif_url": null,
      "gif_preview_url": null,
      "views": 12,
      "likeCount": 3,
      "repostCount": 0,
      "commentCount": 0,
      "likedByUser": false,
      "repostedByUser": false,
      "likedByFollowed": false,
      "handle": "dani",
      "username": "dani",
      "bio": "building lyntr",
      "iq": 142,
      "verified": true,
      "isAdmin": false,
      "contributor": true,
      "loginStreak": 4,
      "followerCount": 918,
      "followsViewer": false,
      "nameColor": "#5470ff",
      "userCreatedAt": "2025-02-11T08:15:00.000Z",
      "parentContent": null,
      "parentHasImage": null,
      "parentImages": null,
      "parentGifUrl": null,
      "parentGifPreviewUrl": null,
      "parentUserHandle": null,
      "parentUserCreatedAt": null,
      "parentUserBio": null,
      "parentUserUsername": null,
      "parentUserVerified": null,
      "parentUserIq": null,
      "parentUserId": null,
      "parentCreatedAt": null,
      "parentUserNameColor": null,
      "poll": null
    }
  ]
}
```

#### `POST /lynts`

<sub>Requires the `lynts:write` scope.</sub>

Posts a lynt, up to 280 characters.
New in v2: images are now accepted, up to 4 per lynt. Content is run through the same moderation pipeline as posts made through the app — image uploads are also run through NSFW screening the same way the app’s own composer does.

> content is required unless at least one image is attached (an image-only lynt is valid) · content: 0–280 characters · images: up to 4 files, field name "images" repeated once per file · GIF and poll attachments still aren’t available over the API. Response is the same full lynt shape returned by every other lynt endpoint (trimmed here for space — see the Feed example above for every field); has_image and images reflect what was actually uploaded.

Request:

```bash
# Text only (application/json) — same as v1
curl -X POST https://lyntr.gizmowizard.tech/api/v2/lynts \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET" \
  -H "Content-Type: application/json" \
  -d '{ "content": "just shipped the v2 api" }'

# With images (multipart/form-data) — new in v2
curl -X POST https://lyntr.gizmowizard.tech/api/v2/lynts \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET" \
  -F "content=screenshots incoming" \
  -F "images=@one.png" \
  -F "images=@two.png"
```

Response:

```json
{
  "id": "7331099...",
  "content": "just shipped the v2 api",
  "userId": "7331021...",
  "createdAt": "2026-07-19T10:02:11.000Z",
  "editedAt": null,
  "reposted": false,
  "parentId": null,
  "has_image": false,
  "images": [],
  "gif_url": null,
  "gif_preview_url": null,
  "views": 0,
  "likeCount": 0,
  "repostCount": 0,
  "commentCount": 0,
  "likedByUser": false,
  "repostedByUser": false,
  "likedByFollowed": false,
  "handle": "dani",
  "username": "dani",
  "bio": "building lyntr",
  "iq": 142,
  "verified": true,
  "isAdmin": false,
  "contributor": true,
  "loginStreak": 4,
  "followerCount": 918,
  "followsViewer": false,
  "nameColor": "#5470ff",
  "userCreatedAt": "2025-02-11T08:15:00.000Z",
  "parentContent": null,
  "poll": null
}
```

#### `GET /lynts/:id`

<sub>No scopes required.</sub>

Fetches a single lynt by id.

> 404 if the lynt doesn’t exist. Full response shape matches the Feed endpoint’s lynt objects, plus two fields the other endpoints don’t return: parent (the raw parent lynt id — parentId is the same value) and referencedLynts (the full chain of ancestor lynts, oldest first, if this lynt is a reply — empty array otherwise).

Request:

```bash
curl https://lyntr.gizmowizard.tech/api/v2/lynts/7331099... \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{
  "id": "7331099...",
  "content": "just shipped the v2 api",
  "userId": "7331021...",
  "createdAt": "2026-07-19T10:02:11.000Z",
  "editedAt": null,
  "reposted": false,
  "parentId": null,
  "parent": null,
  "likeCount": 3,
  "commentCount": 1,
  "likedByUser": false,
  "handle": "dani",
  "username": "dani",
  "poll": null,
  "referencedLynts": [],
  "...": "same full shape as the Feed endpoint's lynt objects"
}
```

#### `PUT /lynts/:id`

<sub>Requires the `lynts:write` scope.</sub>

Edits the text of a lynt you own. Reposts have no original text, so they can’t be edited. Content is re-moderated and re-scanned for @mentions and #hashtags, exactly like an edit made through the app.

> 403 if you don’t own the lynt · 400 if it’s a repost · content: 1–280 characters. Response is the full lynt shape, not just the edited fields.

Request:

```bash
curl -X PUT https://lyntr.gizmowizard.tech/api/v2/lynts/7331099... \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET" \
  -H "Content-Type: application/json" \
  -d '{ "content": "just shipped the v2 api (typo fixed)" }'
```

Response:

```json
{
  "id": "7331099...",
  "content": "just shipped the v2 api (typo fixed)",
  "userId": "7331021...",
  "createdAt": "2026-07-19T10:02:11.000Z",
  "editedAt": "2026-07-19T10:05:44.000Z",
  "likeCount": 3,
  "commentCount": 1,
  "poll": null,
  "...": "same full shape as the Feed endpoint's lynt objects"
}
```

#### `DELETE /lynts/:id`

<sub>Requires the `lynts:write` scope.</sub>

Deletes a lynt you own.

> 404 if the lynt doesn’t exist or isn’t owned by you.

Request:

```bash
curl -X DELETE https://lyntr.gizmowizard.tech/api/v2/lynts/7331099... \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{ "success": true }
```

### Comments

#### `GET /lynts/:id/comments`

<sub>No scopes required.</sub>

Up to 50 most recent top-level comments on a lynt, newest first.

> Each comment is a full lynt object (see the Feed endpoint) with parentId set to the lynt it replies to.

Request:

```bash
curl https://lyntr.gizmowizard.tech/api/v2/lynts/7331099.../comments \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{
  "comments": [
    {
      "id": "7331150...",
      "content": "nice, does it support polls yet?",
      "userId": "7331040...",
      "createdAt": "2026-07-19T10:11:02.000Z",
      "parentId": "7331099...",
      "likeCount": 0,
      "commentCount": 0,
      "likedByUser": false,
      "handle": "someone",
      "username": "Someone",
      "poll": null,
      "...": "same full shape as the Feed endpoint's lynt objects"
    }
  ]
}
```

#### `GET /lynts/all/comments`

<sub>No scopes required.</sub>

New in v2. The most recent comments across every lynt on Lyntr, newest first — not scoped to one parent. Requested by @libhmrc (@nothmrc on Lyntr): before this, getting recent comments meant calling GET /lynts/:id/comments once per lynt you cared about, which doesn’t scale and is an easy way to get rate-limited. This is the same underlying data, queried by time across all parents at once instead of per-parent.

> before: an ISO timestamp cursor — pass the createdAt of the oldest comment you’ve already seen to page further back, same convention as the Feed endpoint’s pagination. Up to 50 per page. Each comment is a full lynt object, same as GET /lynts/:id/comments.

Request:

```bash
curl "https://lyntr.gizmowizard.tech/api/v2/lynts/all/comments" \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{
  "comments": [
    {
      "id": "7331150...",
      "content": "nice, does it support polls yet?",
      "userId": "7331040...",
      "createdAt": "2026-07-19T10:11:02.000Z",
      "parentId": "7331099...",
      "likeCount": 0,
      "commentCount": 0,
      "likedByUser": false,
      "handle": "someone",
      "username": "Someone",
      "poll": null,
      "...": "same full shape as the Feed endpoint's lynt objects"
    }
  ]
}
```

#### `POST /lynts/:id/comments`

<sub>Requires the `lynts:write` scope.</sub>

Replies to a lynt. Images are accepted here too, same as POST /lynts. The parent author gets a notification and lyntcoins award unless you’re replying to yourself.

> content is required unless at least one image is attached · same multipart "images" field as POST /lynts for attaching up to 4 images. Response is a full lynt object (see the Feed endpoint), with parentId set to the lynt you replied to.

Request:

```bash
curl -X POST https://lyntr.gizmowizard.tech/api/v2/lynts/7331099.../comments \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET" \
  -H "Content-Type: application/json" \
  -d '{ "content": "same request shape as v1, images now optional via multipart" }'
```

Response:

```json
{
  "id": "7331150...",
  "content": "same request shape as v1, images now optional via multipart",
  "userId": "7331021...",
  "createdAt": "2026-07-19T10:12:30.000Z",
  "parentId": "7331099...",
  "has_image": false,
  "images": [],
  "likeCount": 0,
  "commentCount": 0,
  "poll": null,
  "...": "same full shape as the Feed endpoint's lynt objects"
}
```

### Likes

#### `POST /lynts/:id/like`

<sub>Requires the `lynts:write` scope.</sub>

Likes a lynt. Liking your own lynt is allowed but won’t generate a notification.

> Returns { "liked": true, "message": "Already liked." } if you’d already liked it.

Request:

```bash
curl -X POST https://lyntr.gizmowizard.tech/api/v2/lynts/7331099.../like \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{ "liked": true }
```

#### `DELETE /lynts/:id/like`

<sub>Requires the `lynts:write` scope.</sub>

Removes your like from a lynt.

Request:

```bash
curl -X DELETE https://lyntr.gizmowizard.tech/api/v2/lynts/7331099.../like \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{ "liked": false }
```

### Users

#### `GET /users/:handle`

<sub>No scopes required.</sub>

A public profile, including follower/following counts and whether you follow them.

Request:

```bash
curl https://lyntr.gizmowizard.tech/api/v2/users/dani \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{
  "id": "7331021...",
  "username": "dani",
  "handle": "dani",
  "bio": "building lyntr",
  "iq": 142,
  "created_at": "2025-02-11T08:15:00.000Z",
  "verified": true,
  "contributor": true,
  "follower_count": 918,
  "following_count": 214,
  "followed_by_viewer": false
}
```

#### `POST /users/:handle/follow`

<sub>Requires the `follows:write` scope.</sub>

Follows a user. Following yourself returns a 400.

> Returns { "following": true, "message": "Already following." } if you already followed them. 400 if you try to follow yourself.

Request:

```bash
curl -X POST https://lyntr.gizmowizard.tech/api/v2/users/dani/follow \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{ "following": true }
```

#### `DELETE /users/:handle/follow`

<sub>Requires the `follows:write` scope.</sub>

Unfollows a user.

Request:

```bash
curl -X DELETE https://lyntr.gizmowizard.tech/api/v2/users/dani/follow \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{ "following": false }
```

### Search

#### `GET /search`

<sub>No scopes required.</sub>

Simple plain-substring content search. For the full operator syntax (from:, #tag, etc.) use the app itself for now.

> q is required — a 400 is returned if it’s missing. Results are full lynt objects (see the Feed endpoint), reposts excluded.

Request:

```bash
curl "https://lyntr.gizmowizard.tech/api/v2/search?q=lyntr%20api" \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{
  "lynts": [
    {
      "id": "7331099...",
      "content": "just shipped the v2 api",
      "userId": "7331021...",
      "createdAt": "2026-07-19T10:02:11.000Z",
      "likeCount": 3,
      "commentCount": 0,
      "likedByUser": false,
      "handle": "dani",
      "username": "dani",
      "poll": null,
      "...": "same full shape as the Feed endpoint's lynt objects"
    }
  ]
}
```

### Notifications

#### `GET /notifications`

<sub>Requires the `notifications:read` scope.</sub>

Up to 50 most recent notifications, newest first.

Request:

```bash
curl https://lyntr.gizmowizard.tech/api/v2/notifications \
  -H "X-Client-Id: $LYNTR_CLIENT_ID" \
  -H "X-Client-Secret: $LYNTR_CLIENT_SECRET"
```

Response:

```json
{
  "notifications": [
    {
      "id": "7331201...",
      "type": "like",
      "sourceUserId": "7331040...",
      "sourceUserHandle": "someone",
      "sourceUsername": "Someone",
      "lyntId": "7331099...",
      "lyntContent": "just shipped the v2 api",
      "read": false,
      "createdAt": "2026-07-19T10:11:40.000Z",
      "mentionCount": null
    }
  ]
}
```
