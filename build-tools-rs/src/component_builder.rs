use edit_xml::{Document, EditXMLError, ReadOptions, WriteOptions};
use thiserror::Error;

static COMPONENT_SCRIPT: &str = include_str!("../template.vue");
pub static COMPONENT_TYPE_DECLARATION: &str = include_str!("../template.vue.d.ts");
#[derive(Debug, Error)]
pub enum ComponentBuilderError {
    #[error(transparent)]
    XMLError(#[from] EditXMLError),
    #[error("No root element found in SVG content")]
    NoRootElement,
}
pub fn generate_component(svg_content: &str) -> Result<String, ComponentBuilderError> {
    let reader = std::io::Cursor::new(svg_content);
    let mut svg_content = Document::parse_reader_with_opts(
        reader,
        ReadOptions {
            require_decl: false,
            ..Default::default()
        },
    )?;
    let root = svg_content
        .root_element()
        .ok_or(ComponentBuilderError::NoRootElement)?;

    root.set_attribute(&mut svg_content, ":width", "finalSize");
    root.set_attribute(&mut svg_content, ":height", "finalSize");
    root.set_attribute(&mut svg_content, "role", "img");
    root.set_attribute(&mut svg_content, "viewBox", "0 0 24 24");
    root.set_attribute(&mut svg_content, "v-bind", "$attrs");
    root.set_attribute(&mut svg_content, "fill", "currentColor");

    let svg_content = svg_content.write_str_with_opts(WriteOptions {
        write_decl: false,
        ..Default::default()
    })?;

    return Ok(format!(
        "<template>
        {svg_content}
        </template>
        \n\n{COMPONENT_SCRIPT}",
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_component() {
        let svg_content =
            r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="12" cy="12" r="10"/></svg>"#;
        let component = generate_component(svg_content).unwrap();
        assert!(component.contains(":width=\"finalSize\""));
        assert!(component.contains(":height=\"finalSize\""));
        assert!(component.contains("role=\"img\""));
        assert!(component.contains("viewBox=\"0 0 24 24\""));
        assert!(component.contains("v-bind=\"$attrs\""));
        assert!(component.contains("fill=\"currentColor\""));
        println!("{}", component);
    }
}
