use spacepls::FileIO;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Clone)]
pub struct NativeFileIO {}

impl NativeFileIO {
    pub fn init() -> Self {
        NativeFileIO {}
    }
}

#[async_trait::async_trait]
impl FileIO for NativeFileIO {
    async fn write<'a>(
        &'a self,
        path: &'a str,
        content: &'a [u8],
        _: String,
    ) -> anyhow::Result<()> {
        // TODO add AES support
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(content).await?;
        log::info!("File write: {} ... ok", path);
        Ok(())
    }

    async fn read<'a>(&'a self, path: &'a str, _: String) -> anyhow::Result<String> {
        // TODO add AES support
        let mut file = tokio::fs::File::open(path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        log::info!("File read: {} ... ok", path);
        Ok(String::from_utf8(buffer)?)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[tokio::test]
    async fn test_write_and_read_file() {
        // Setup - Create a temporary file
        let tmp_file = NamedTempFile::new().expect("Failed to create temp file");
        let tmp_path = tmp_file
            .path()
            .to_str()
            .expect("Failed to get temp file path");
        let file_io = NativeFileIO::init();

        // Test writing to the file
        let content = b"Hello, world!";
        file_io
            .write(tmp_path, content, String::new())
            .await
            .expect("Failed to write to temp file");

        // Test reading from the file
        let read_content = file_io
            .read(tmp_path, String::new())
            .await
            .expect("Failed to read from temp file");

        // Verify the content is as expected
        assert_eq!(read_content, String::from_utf8_lossy(content));
    }

    #[tokio::test]
    async fn test_write_error() {
        // Attempt to write to an invalid path
        let file_io = NativeFileIO::init();
        let result = file_io
            .write("/invalid/path/to/file.txt", b"content", String::new())
            .await;

        // Verify that an error is returned
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read_error() {
        // Attempt to read from a non-existent file
        let file_io = NativeFileIO::init();
        let result = file_io.read("/non/existent/file.txt", String::new()).await;

        // Verify that an error is returned
        assert!(result.is_err());
    }
}
