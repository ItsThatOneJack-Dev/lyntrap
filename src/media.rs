/// An image to attach to a lynt or comment, given directly as bytes.
/// `lyntrap` never touches the filesystem — if you have a file, read it
/// yourself first; if you have a pasted clipboard image or a downloaded
/// buffer, hand the bytes straight in.
#[derive(Debug, Clone)]
pub struct Image {
    pub(crate) filename: String,
    pub(crate) data: Vec<u8>,
    pub(crate) content_type: Option<String>,
}

impl Image {
    /// `filename` just needs a plausible extension (e.g. `"image.png"`) —
    /// it's used to guess the content type if you don't set one explicitly,
    /// and is echoed in the multipart part's filename field.
    pub fn new(filename: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Self {
            filename: filename.into(),
            data: data.into(),
            content_type: None,
        }
    }

    /// Override the guessed content type — useful if you already know it
    /// (e.g. from a clipboard paste event's MIME type) and don't want to
    /// rely on the filename's extension.
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    pub(crate) fn content_type(&self) -> String {
        if let Some(ct) = &self.content_type {
            return ct.clone();
        }
        let ext = self
            .filename
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            _ => "application/octet-stream",
        }
        .to_string()
    }
}
