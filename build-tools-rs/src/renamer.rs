use std::{borrow::Cow, sync::LazyLock};

use heck::ToPascalCase;
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;
#[derive(Debug, Error)]
pub enum IconNameError {
    #[error("Invalid icon name: {0}")]
    NumTwoWordsError(num2words::Num2Err),
    #[error("Number Was Found but could not be converted: {0}")]
    NumberConversionError(#[from] std::num::ParseIntError),
}
impl From<num2words::Num2Err> for IconNameError {
    fn from(err: num2words::Num2Err) -> Self {
        IconNameError::NumTwoWordsError(err)
    }
}

use crate::types::Brand;
static VALID_ICON_NAME_REGEX_STR: &str = r"^[a-zA-Z ]+$";
static VALID_ICON_NAME_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(VALID_ICON_NAME_REGEX_STR).expect("Failed to compile regex")
});
static STARTING_NUMBER_REGEX_STR: &str = r#"^(?P<number>[\d]+)"#;
static STARTING_NUMBER_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(STARTING_NUMBER_REGEX_STR).expect("Failed to compile regex")
});
static TITLE_TO_SLUG_REGEX_STR: &str = r#"[^a-z\d]"#;
static TITLE_TO_SLUG_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(TITLE_TO_SLUG_REGEX_STR).expect("Failed to compile regex"));

#[derive(Debug, Clone)]
pub struct TitleFixes {
    pub char: char,
    pub replacement: &'static str,
}

pub const TITLE_FIXES: &[TitleFixes] = &[
    TitleFixes {
        char: '&',
        replacement: "And",
    },
    TitleFixes {
        char: '+',
        replacement: "Plus",
    },
    TitleFixes {
        char: '.',
        replacement: "Dot",
    },
    TitleFixes {
        char: 'đ',
        replacement: "d",
    },
    TitleFixes {
        char: 'ħ',
        replacement: "h",
    },
    TitleFixes {
        char: 'ĸ',
        replacement: "k",
    },
    TitleFixes {
        char: 'ŀ',
        replacement: "l",
    },
    TitleFixes {
        char: 'ł',
        replacement: "l",
    },
    TitleFixes {
        char: 'ß',
        replacement: "ss",
    },
    TitleFixes {
        char: 'ŧ',
        replacement: "o",
    },
];
fn capitalize_first_letter(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    s.chars()
        .next()
        .map(|c| c.to_uppercase().to_string() + &s[1..])
        .unwrap()
}
/// Remove a number if it is the first part of the string and convert it to words.
pub fn remove_starting_number<'input>(
    input: &'input str,
) -> Result<Cow<'input, str>, IconNameError> {
    for cap in STARTING_NUMBER_REGEX.captures_iter(input) {
        // Extract the number from the capture group
        if let Some(number) = cap.name("number") {
            println!("Found number: {} in {}", number.as_str(), input);
            // Convert the number to a string and replace it in the original string
            let number_str: i64 = number.as_str().parse()?;
            let number_two_str = num2words::Num2Words::new(number_str)
                .cardinal()
                .to_words()?
                .split(" ")
                .map(|s| capitalize_first_letter(s))
                .collect::<Vec<String>>()
                .join(" ");
            return Ok(Cow::Owned(format!(
                "{}{}",
                number_two_str,
                capitalize_first_letter(&input[number.end()..])
            )));
        }
    }
    Ok(Cow::Borrowed(input))
}

pub fn fix_string<'input>(title_fixes: &[TitleFixes], input: &'input str) -> Cow<'input, str> {
    let mut fixed_string = input.to_string();
    for fix in title_fixes {
        fixed_string = fixed_string.replace(fix.char, fix.replacement);
    }
    Cow::Owned(fixed_string)
}

pub fn component_name(brand: &Brand) -> Result<String, IconNameError> {
    let icon_name = brand.slug.as_deref().unwrap_or(brand.title.as_str());
    if VALID_ICON_NAME_REGEX.is_match(icon_name) {
        let icon_as_pascal_case = icon_name.to_pascal_case();
        return Ok(format!("{}Icon", icon_as_pascal_case));
    }
    let fixed_title = fix_string(TITLE_FIXES, icon_name);
    let fix_numbers = remove_starting_number(&fixed_title)?;

    Ok(format!("{}Icon", fix_numbers.to_pascal_case()))
}
pub fn title_to_slug(title: &str) -> String {
    let current_str = fix_string(TITLE_FIXES, &title.to_lowercase())
        .to_lowercase()
        .nfd()
        .to_string();

    TITLE_TO_SLUG_REGEX
        .replace_all(&current_str, "")
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_title_regex() {
        let input = "Hello & World + Rust.";
        let expected = "Hello And World Plus RustDot";
        let result = fix_string(TITLE_FIXES, input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_correct_numbers() -> anyhow::Result<()> {
        let cases = vec![("100 g 100", "One Hundred g 100"), ("4Chan", "FourChan")];
        for (input, expected) in cases {
            let result = remove_starting_number(input)?;
            assert_eq!(result, expected);
        }
        Ok(())
    }

    #[test]
    fn full_tests() {
        let test_cases = vec![
            (
                Brand {
                    slug: Some("hello-world".to_string()),
                    title: "Hello World".to_string(),
                    ..Default::default()
                },
                "HelloWorldIcon",
            ),
            (
                Brand {
                    title: "/e/".to_owned(),
                    ..Default::default()
                },
                "EIcon",
            ),
            (
                Brand {
                    title: "4chan".to_owned(),
                    ..Default::default()
                },
                "FourChanIcon",
            ),
        ];
        for (brand, expected) in test_cases {
            let result = component_name(&brand).unwrap();
            println!("Brand: {:?}, Result: {}", brand, result);
            assert_eq!(result, expected);
        }
    }
}
