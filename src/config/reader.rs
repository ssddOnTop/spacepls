use crate::config::Config;
use crate::TargetRuntime;
use reqwest::Url;
use std::path::Path;

/// Reads the configuration from a file or from an HTTP URL and resolves all linked extensions to create a ConfigModule.
pub struct ConfigReader {
    runtime: TargetRuntime,
}

/// Response of a file read operation
#[derive(Debug)]
struct FileRead {
    content: String,
    path: String,
}

impl ConfigReader {
    pub fn init(runtime: TargetRuntime) -> Self {
        Self { runtime }
    }

    /// Reads the config file and returns serialized config
    pub async fn read<T: ToString>(&self, files: &T) -> anyhow::Result<Config> {
        let file = self.read_file(files).await?;

        let config = Self::resolve(
            Config::from_json(&file.content)?,
            Path::new(&file.path).parent(),
        )
        .await;

        Ok(config)
    }
    /// Reads a file from the filesystem or from an HTTP URL
    async fn read_file<T: ToString>(&self, file: &T) -> anyhow::Result<FileRead> {
        // Is an HTTP URL
        let content = if let Ok(url) = Url::parse(&file.to_string()) {
            let response = self
                .runtime
                .http
                .execute(
                    reqwest::Request::new(reqwest::Method::GET, url),
                    String::new(),
                ) // reading config should not require key
                .await?;

            String::from_utf8(response.body.to_vec())?
        } else {
            // Is a file path

            self.runtime
                .file
                .read(&file.to_string(), String::new())
                .await? // reading config should not require key
        };

        Ok(FileRead {
            content,
            path: file.to_string(),
        })
    }
    async fn resolve(mut config: Config, parent_dir: Option<&Path>) -> Config {
        let dir = config.extensions.dir_path.unwrap_or("spacepls".to_string());
        let dir = Self::resolve_path(&dir, parent_dir);
        config.extensions.dir_path = Some(dir);
        config.extensions.password = Some("$Xme-Ef9r[@EsqF".to_string()); // randomly generated password

        config
    }
    /// Checks if path is absolute else it joins file path with relative dir path
    fn resolve_path(src: &str, root_dir: Option<&Path>) -> String {
        if Path::new(&src).is_absolute() {
            src.to_string()
        } else {
            let path = root_dir.unwrap_or(Path::new(""));
            path.join(src).to_string_lossy().to_string()
        }
    }
}
