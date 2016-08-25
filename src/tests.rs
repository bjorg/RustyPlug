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

fn full_plug() -> Plug {
    return Plug {
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
}

#[test]
fn full_plug_succeeds() {
    let p = full_plug();
    assert_eq!(String::from("http://bob:pwd@example.org:8081/a/b/c/?key=value#anchor"), p.to_string());
}

#[test]
fn default_plug_succeeds() {
    let p = default_plug();
    assert_eq!(String::from("http://example.org"), p.to_string());
}

#[test]
fn with_scheme_succeeds() {
    let p = default_plug().with_scheme("https".into());
    assert_eq!(String::from("https://example.org"), p.to_string());
}

#[test]
fn with_host_succeeds() {
    let p = default_plug().with_host("example.com".into());
    assert_eq!(String::from("http://example.com"), p.to_string());
}

#[test]
fn with_port_succeeds() {
    let p = default_plug().with_port(8081);
    assert_eq!(String::from("http://example.org:8081"), p.to_string());
}

#[test]
fn without_port_succeeds() {
    let p = full_plug().without_port();
    assert_eq!(String::from("http://bob:pwd@example.org/a/b/c/?key=value#anchor"), p.to_string());
}

#[test]
fn at_succeeds() {
    let p = default_plug().at(vec!["a".into(), "b".into(), "c".into()]);
    assert_eq!(String::from("http://example.org/a/b/c"), p.to_string());
}

#[test]
fn without_path_succeeds() {
    let p = full_plug().without_path();
    assert_eq!(String::from("http://bob:pwd@example.org:8081/?key=value#anchor"), p.to_string());
}

#[test]
fn with_succeeds() {
    let p = default_plug().with("key".into(), "value".into());
    assert_eq!(String::from("http://example.org?key=value"), p.to_string());
}

#[test]
fn with_fragment_succeeds() {
    let p = default_plug().with_fragment("anchor".into());
    assert_eq!(String::from("http://example.org#anchor"), p.to_string());
}

#[test]
fn without_fragment_succeeds() {
    let p = full_plug().without_fragment();
    assert_eq!(String::from("http://bob:pwd@example.org:8081/a/b/c/?key=value"), p.to_string());
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