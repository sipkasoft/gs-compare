pub fn parse(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|l| l.split_once("="))
        .map(|(k, v)| (k.trim().into(), v.trim().into()))
        .collect()
}

pub fn get(pairs: &[(String, String)], key: &str) -> Option<String> {
    pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
}
