use url::Url;

pub fn validate_url(a_url: String) -> Option<bool> {
    let issue_list_url = Url::parse(&a_url).ok()?;
    Some(issue_list_url.scheme() == "https" && !issue_list_url.cannot_be_a_base())
}
