use std::{borrow::Cow, path::PathBuf};

use crate::renamer::{self, IconNameError};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VueSimpleIcons {
    pub built_on: DateTime<Local>,
    pub simple_icons_version: String,
    pub components: Vec<Component>,
}
impl VueSimpleIcons {
    pub fn generate_index_ts(&self) -> String {
        let mut index_content = String::new();
        for component in &self.components {
            index_content.push_str(&format!(
                r#"export {} from './components/{}.vue';"#,
                component.component_name, component.component_name
            ));
            index_content.push('\n');
        }
        index_content
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub original_title: String,
    pub slug: String,
    pub component_name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Brand {
    pub aliases: Option<Aliases>,
    pub guidelines: Option<String>,
    pub hex: Option<String>,
    pub license: Option<License>,
    pub slug: Option<String>,
    pub source: String,
    pub title: String,
}
impl Brand {
    pub fn get_slug(&self) -> Cow<'_, str> {
        if let Some(slug) = &self.slug {
            Cow::Borrowed(slug)
        } else {
            Cow::Owned(renamer::title_to_slug(&self.title))
        }
    }
    pub fn get_file_name(&self) -> String {
        if let Some(slug) = &self.slug {
            format!("{}.svg", slug)
        } else {
            let fixed_title = renamer::title_to_slug(&self.title);
            format!("{}.svg", fixed_title)
        }
    }
    pub fn get_file(&self) -> PathBuf {
        PathBuf::from("icons").join(self.get_file_name())
    }
    pub fn get_component_name(&self) -> Result<String, IconNameError> {
        renamer::component_name(self)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Aliases {
    pub aka: Option<Vec<String>>,
    pub dup: Option<Vec<Duplicate>>,
    pub old: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duplicate {
    pub guidelines: Option<String>,
    pub hex: Option<String>,
    pub slug: Option<String>,
    pub source: Option<String>,
    pub title: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub struct License {
    #[serde(rename = "type")]
    pub license_type: String,
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::types::Brand;

    #[test]
    pub fn test_icon_names() -> anyhow::Result<()> {
        let icons_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("downloaded_icons");
        let path = icons_path.join("data").join("simple-icons.json");
        let mut icons_reader = std::fs::File::open(path)?;
        let icons: Vec<Brand> = serde_json::from_reader(&mut icons_reader)?;
        for icon in icons {
            let path = icon.get_file();
            let full_path = icons_path.join(&path);
            if !full_path.exists() {
                println!("Icon file does not exist: {}", full_path.display());
                continue;
            }
        }
        Ok(())
    }
}
