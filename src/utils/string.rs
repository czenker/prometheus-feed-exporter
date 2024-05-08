use regex::Regex;

pub fn strip_html(html: &String) -> String {
    let re_strip_tags = Regex::new(r"<[^>]*>").unwrap();
    let re_collapse_whitespace = Regex::new(r"\s+").unwrap();
    let foo = re_strip_tags.replace_all(html, "");
    let bar = re_collapse_whitespace.replace_all(&foo, " ");
    let baz = html_escape::decode_html_entities(&bar);

    String::from(baz.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_html() {
        assert_eq!("Hello World!", strip_html(&String::from("<h1>Hello World!</h1>")));
        assert_eq!("Lorem ipsum", strip_html(&String::from("<p>Lorem <i>ipsum</i></p>")));
    }

    #[test]
    fn test_strip_html_removes_whitespace() {
        assert_eq!("Hello World! Lorem ipsum.", strip_html(&String::from(" \n <h1>Hello World! </h1>\n\n<p> Lorem   ipsum.</p>\n\n   ")));
    }

    #[test]
    fn test_strip_html_unescapes_html() {
        assert_eq!("Rust & Ferris 🦀", strip_html(&String::from("Rust &amp; Ferris &#x1f980;")));
    }

}