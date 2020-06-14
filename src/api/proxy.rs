static CORS_PROXY: &str = "https://cors-anywhere.herokuapp.com/";

pub fn proxy<S: AsRef<str>>(url: S) -> String {
    let mut proxied = CORS_PROXY.to_owned();
    proxied.push_str(url.as_ref());
    proxied
}
