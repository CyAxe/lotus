use std::collections::HashMap;
use url::Url;

pub fn change_urlquery(
    url: String,
    payload: String,
    remove_content: bool,
) -> HashMap<String, String> {
    let url = Url::parse(&url).unwrap();
    let mut scan_params = HashMap::new();
    let mut result: HashMap<String, String> = HashMap::new();
    let mut param_list = Vec::new();

    url.query_pairs()
        .collect::<HashMap<_, _>>()
        .iter()
        .for_each(|(key, value)| {
            scan_params.insert(key.to_string(), value.to_string());
            param_list.push(key.to_string());
        });

    scan_params.iter().for_each(|(key, value)| {
        payload.split('\n').into_iter().for_each(|payload| {
            let mut new_params = scan_params.clone();
            if remove_content {
                new_params.insert(key.to_string(), payload.to_string());
            } else {
                new_params.insert(key.to_string(), value.as_str().to_owned() + payload);
            }
            let mut new_url = url.clone();
            new_url.query_pairs_mut().clear();

            new_url.query_pairs_mut().extend_pairs(&new_params);

            result.insert(key.to_string(), new_url.as_str().to_string());
        });
    });
    result
}

pub fn set_urlvalue(url: &str, param: &str, payload: &str) -> String {
    let mut url = Url::parse(url).unwrap();
    let mut final_params = HashMap::new();
    url.query_pairs()
        .into_iter()
        .collect::<HashMap<_, _>>()
        .iter()
        .for_each(|(k, v)| {
            if k == param {
                final_params.insert(k.to_string(), format!("{}{}", v, payload));
            } else {
                final_params.insert(k.to_string(), v.to_string());
            }
        });
    url.query_pairs_mut().clear();
    url.query_pairs_mut().extend_pairs(final_params);
    url.as_str().to_string()
}

pub fn urljoin(url: String, path: String) -> String {
    Url::parse(&url)
        .unwrap()
        .join(&path)
        .unwrap()
        .as_str()
        .to_string()
}
