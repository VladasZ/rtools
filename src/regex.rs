use regex::Regex;

pub fn find_match(str: impl AsRef<str>, query: impl AsRef<str>) -> String {
    let re = Regex::new(query.as_ref()).unwrap();
    let mat = re.find(str.as_ref()).unwrap();
    String::from(mat.as_str())
}

pub fn find_matches(str: impl AsRef<str>, query: impl AsRef<str>) -> Vec<String> {
    Regex::new(query.as_ref())
        .unwrap()
        .find_iter(str.as_ref())
        .map(|x| String::from(x.as_str()))
        .collect()
}
