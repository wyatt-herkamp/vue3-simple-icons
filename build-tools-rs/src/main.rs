use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::bail;
use chrono::Local;
use clap::{Parser, Subcommand};

use crate::types::VueSimpleIcons;
pub mod component_builder;
pub mod downloader;
pub mod renamer;
pub mod types;
#[derive(Parser, Debug, Clone)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Download {
        version: String,
        #[clap(long = "target", default_value = "simple-icons")]
        target_location: PathBuf,
    },

    Build {
        #[clap(long = "target", default_value = "src/")]
        target_location: PathBuf,
        #[clap(long = "simple-icons", default_value = "simple-icons")]
        simple_icons_path: PathBuf,
        #[clap(long = "info-json", default_value = "simple-icons.json")]
        info_json_path: PathBuf,
    },
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Download {
            version,
            target_location,
        } => {
            downloader::download_icons(&version, &target_location).await?;
        }
        Commands::Build {
            target_location,
            simple_icons_path,
            info_json_path,
        } => {
            build_components(target_location, simple_icons_path, info_json_path)?;
        }
    }

    Ok(())
}

pub fn build_components(
    target_location: PathBuf,
    simple_icons_path: PathBuf,
    info_json_path: PathBuf,
) -> anyhow::Result<()> {
    let mut icons_reader =
        std::fs::File::open(simple_icons_path.join("data").join("simple-icons.json"))?;
    let icons: Vec<types::Brand> = serde_json::from_reader(&mut icons_reader)?;
    let mut components = VueSimpleIcons {
        built_on: Local::now(),
        simple_icons_version: get_version_from_package_json(&simple_icons_path)?,
        components: Vec::with_capacity(icons.len()),
    };
    let components_folder = target_location.join("components");
    if !components_folder.exists() {
        std::fs::create_dir_all(&components_folder)?;
    }
    for icon in icons {
        let file = icon.get_file();
        let full_path = simple_icons_path.join(&file);
        if !full_path.exists() {
            bail!("Icon file does not exist: {}", full_path.display());
        }
        let svg_content = std::fs::read_to_string(full_path)?;
        let component = component_builder::generate_component(&svg_content)?;
        let component_name = icon.get_component_name()?;
        let outpath = components_folder.join(format!("{}.vue", component_name));
        std::fs::write(outpath, component)?;
        components.components.push(types::Component {
            original_title: icon.title.clone(),
            slug: icon.get_slug().into_owned(),
            component_name,
        });
    }
    {
        let mut index_file = File::create(target_location.join("index.ts"))?;
        for component in &components.components {
            writeln!(
                index_file,
                r#"import {component_name} from "./components/{component_name}.vue"; // Source From icons/{source}.svg"#,
                component_name = component.component_name,
                source = component.slug
            )?;
        }
        let exports = components
            .components
            .iter()
            .map(|c| c.component_name.clone())
            .collect::<Vec<_>>()
            .join(", \n");
        writeln!(index_file, "export {{{}}};", exports)?;
    }
    let mut info_json_file = File::create(info_json_path)?;
    serde_json::to_writer_pretty(&mut info_json_file, &components)?;

    println!(
        "Built {} components in {:?}",
        components.components.len(),
        target_location
    );

    Ok(())
}

pub fn get_version_from_package_json(download_path: impl AsRef<Path>) -> anyhow::Result<String> {
    let mut json_file = File::open(download_path.as_ref().join("package.json"))?;
    let package_json: serde_json::Value = serde_json::from_reader(&mut json_file)?;
    let version = package_json
        .get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Version not found in package.json"))?;
    Ok(version.to_string())
}
