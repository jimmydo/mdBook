extern crate toml;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TomlConfig {
    pub source: Option<PathBuf>,

    pub title: Option<String>,
    pub author: Option<String>,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,

    pub output: Option<TomlOutputConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TomlOutputConfig {
    pub html: Option<TomlHtmlConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TomlHtmlConfig {
    pub destination: Option<PathBuf>,
    pub theme: Option<PathBuf>,
    pub google_analytics: Option<String>,
}

/// Returns a TomlConfig from a TOML string
///
/// ```
/// # use mdbook::config::tomlconfig::TomlConfig;
/// # use std::path::PathBuf;
/// let toml = r#"title="Some title"
/// [output.html]
/// destination = "htmlbook" "#;
/// 
/// let config = TomlConfig::from_toml(&toml).expect("Should parse correctly");
/// assert_eq!(config.title, Some(String::from("Some title")));
/// assert_eq!(config.output.unwrap().html.unwrap().destination, Some(PathBuf::from("htmlbook")));
/// ```
impl TomlConfig {
    pub fn from_toml(input: &str) -> Result<Self, String> {
        let config: TomlConfig = toml::from_str(input)
                                        .map_err(|e| format!("Could not parse TOML: {}", e))?;
        
        return Ok(config);
    }
}


