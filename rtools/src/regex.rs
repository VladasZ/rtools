use anyhow::{anyhow, Result};
use regex::Regex;

pub fn find_match(str: &str, query: &str) -> Result<String> {
    let re = Regex::new(query)?;
    let mat = re.find(str).ok_or(anyhow!("Failed to find regex"))?;
    Ok(String::from(mat.as_str()))
}

pub fn find_matches(str: &str, query: &str) -> Result<Vec<String>> {
    Ok(Regex::new(query)?.find_iter(str).map(|x| x.as_str().to_string()).collect())
}
