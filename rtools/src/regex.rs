use anyhow::{anyhow, Result};
use regex::Regex;

/// Find first regex match
pub fn find_match(str: &str, query: &str) -> Result<String> {
    let re = Regex::new(query)?;
    let mat = re.find(str).ok_or(anyhow!("Failed to find regex"))?;
    Ok(String::from(mat.as_str()))
}

/// Find all regex matches
pub fn find_matches(str: &str, query: &str) -> Result<Vec<String>> {
    Ok(Regex::new(query)?.find_iter(str).map(|x| x.as_str().to_string()).collect())
}

#[cfg(test)]
mod test {
    use crate::regex::{find_match, find_matches};

    const INCLUDE_QUERY: &str = r#"#include (("[^ "]+"))"#;

    #[test]
    fn test() {
        let input = r#"
            #include "sokol";
            #include "falkon";
            #include "birdkon";
        "#;

        let find = find_match(input, INCLUDE_QUERY).unwrap();

        assert_eq!(find, r#"#include "sokol""#);

        let find_all = find_matches(input, INCLUDE_QUERY).unwrap();

        assert_eq!(
            find_all,
            [
                r#"#include "sokol""#,
                r#"#include "falkon""#,
                r#"#include "birdkon""#
            ]
        );
    }
}
