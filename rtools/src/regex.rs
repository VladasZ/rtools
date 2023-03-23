use regex::Regex;

pub fn find_match(str: &str, query: &str) -> String {
    let re = Regex::new(query).unwrap();
    let mat = re.find(str).unwrap();
    String::from(mat.as_str())
}

pub fn find_matches(str: &str, query: &str) -> Vec<String> {
    Regex::new(query)
        .unwrap()
        .find_iter(str)
        .map(|x| x.as_str().to_string())
        .collect()
}
