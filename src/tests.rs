#![allow(dead_code)]
#![allow(unused_imports)]

use plug::Plug;
use uri_parser;

//--- plub tests ---
fn default_plug() -> Plug {
    return Plug {
        scheme: "http".into(),
        user: None,
        password: None,
        host: "example.org".into(),
        port: None,
        segments: vec![],
        query: None,
        fragment: None,
        trailing_slash: false,
    };
}

#[test]
fn to_string_with_all_fields() {
    let p = Plug {
        scheme: "http".into(),
        user: Some("bob".into()),
        password: Some("pwd".into()),
        host: "example.org".into(),
        port: Some(8081),
        segments: vec!["a".into(), "b".into(), "c".into()],
        query: Some(vec![("key".into(), Some("value".into()))]),
        fragment: Some("anchor".into()),
        trailing_slash: true,
    };
    assert_eq!(String::from("http://bob:pwd@example.org:8081/a/b/c/?key=value#anchor"), p.to_string());
}

#[test]
fn default_plug_succeeds() {
    let p = default_plug();
    assert_eq!(String::from("http://example.org"), p.to_string());
}

#[test]
fn with_scheme_succeeds() {
    let p = default_plug();
    assert_eq!(String::from("https://example.org"), p.with_scheme("https".into()).to_string());
}

//--- uri_parser tests ---

#[test]
fn try_parse_scheme_succeeds() {
    let text = "http://*";
    let mut chars = text.chars();
    let scheme = uri_parser::try_parse_scheme(&mut chars);
    assert_eq!(Some("http".into()), scheme);
    assert_eq!(Some('*'), chars.next());
}

#[test]
fn try_parse_scheme_with_empty_scheme_fails() {
    let text = "";
    let scheme = uri_parser::try_parse_scheme(&mut text.chars());
    assert_eq!(None, scheme);
}

#[test]
fn try_parse_scheme_with_scheme_with_missing_colon_slash_slash_fails() {
    let text = "http";
    let scheme = uri_parser::try_parse_scheme(&mut text.chars());
    assert_eq!(None, scheme);
}