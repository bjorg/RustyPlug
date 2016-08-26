/*
 * RustyPlug - a rust module with a fluid interface for building requests to sockets
 *
 * Copyright (C) 2016 Steve G. Bjorg
 *
 * For community documentation and downloads visit mindtouch.com;
 * please review the licensing section.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(dead_code)]
#![allow(unused_imports)]

use plug::{Plug, PlugCredentials};
use uri_parser::*;

//--- plub tests ---
fn default_plug() -> Plug {
    return Plug::new(
        "http".into(),
        PlugCredentials::None,
        "example.org".into(),
        None,
        vec![],
        None,
        None,
        false,
    );
}

fn full_plug() -> Plug {
    return Plug::new(
        "http".into(),
        PlugCredentials::UsernamePassword("bob".into(), "pwd".into()),
        "example.org".into(),
        Some(8081),
        vec!["a".into(), "b".into(), "c".into()],
        Some(vec![("key".into(), Some("value".into()))]),
        Some("anchor".into()),
        true,
    );
}

#[test]
fn default_plug_succeeds() {
    let p = default_plug();
    assert_eq!(String::from("http://example.org"), p.to_string());
}

#[test]
fn full_plug_succeeds() {
    let p = full_plug();
    assert_eq!(String::from("http://bob:pwd@example.org:8081/a/b/c/?key=value#anchor"), p.to_string());
}

#[test]
fn get_scheme_succeeds() {
    assert_eq!(String::from("http"), default_plug().get_scheme());
    assert_eq!(String::from("http"), full_plug().get_scheme());
}

#[test]
fn get_credentials_succeeds() {
    assert_eq!(&PlugCredentials::None, default_plug().get_credentials());
    assert_eq!(&PlugCredentials::UsernamePassword(String::from("bob"), String::from("pwd")), full_plug().get_credentials());
}

#[test]
fn get_host_succeeds() {
    assert_eq!(String::from("example.org"), default_plug().get_host());
    assert_eq!(String::from("example.org"), full_plug().get_host());
}

#[test]
fn get_segments_succeeds() {
    assert_eq!(&[] as &[String], default_plug().get_segments());
    assert_eq!(&[String::from("a"), String::from("b"), String::from("c")], full_plug().get_segments());
}

#[test]
fn get_query_succeeds() {
    assert_eq!(&None, default_plug().get_query());
    assert_eq!(&Some(vec![(String::from("key"), Some(String::from("value")))]), full_plug().get_query());
}

#[test]
fn get_fragment_succeeds() {
    assert_eq!(&None, default_plug().get_fragment());
    assert_eq!(&Some(String::from("anchor")), full_plug().get_fragment());
}

#[test]
fn get_trailing_slash_succeeds() {
    assert_eq!(false, default_plug().get_trailing_slash());
    assert_eq!(true, full_plug().get_trailing_slash());
}

#[test]
fn with_scheme_succeeds() {
    let p = default_plug().with_scheme("https".into());
    assert_eq!(String::from("https://example.org"), p.to_string());
}

#[test]
fn with_credentials_username_succeeds() {
    let p = default_plug().with_credentials(PlugCredentials::Username("john".into()));
    assert_eq!(String::from("http://john@example.org"), p.to_string());
}

#[test]
fn with_credentials_username_password_succeeds() {
    let p = default_plug().with_credentials(PlugCredentials::UsernamePassword("john".into(), "pa$$w0rd".into()));
    assert_eq!(String::from("http://john:pa$$w0rd@example.org"), p.to_string());
}

#[test]
fn without_credentials_succeeds() {
    let p = full_plug().without_credentials();
    assert_eq!(String::from("http://example.org:8081/a/b/c/?key=value#anchor"), p.to_string());
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
fn without_query_succeeds() {
    let p = full_plug().without_query();
    assert_eq!(String::from("http://bob:pwd@example.org:8081/a/b/c/#anchor"), p.to_string());
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

#[test]
fn with_trailing_slash_succeeds() {
    let p = default_plug().with_trailing_slash(true);
    assert_eq!(String::from("http://example.org/"), p.to_string());
}

#[test]
fn without_trailing_slash_succeeds() {
    let p = full_plug().without_trailing_slash();
    assert_eq!(String::from("http://bob:pwd@example.org:8081/a/b/c?key=value#anchor"), p.to_string());
}

//--- uri_parser tests ---

#[test]
fn parse_scheme_succeeds() {
    let text = "http";
    let mut chars = text.chars().peekable();
    let scheme = parse_scheme(&mut chars);
    assert_eq!(Ok("http".into()), scheme);
}

#[test]
fn parse_scheme_with_colon_succeeds() {
    let text = "http:";
    let mut chars = text.chars().peekable();
    let scheme = parse_scheme(&mut chars);
    assert_eq!(Ok("http".into()), scheme);
    assert_eq!(Some(':'), chars.next());
}

#[test]
fn parse_scheme_with_empty_scheme_fails() {
    let text = "";
    let mut chars = text.chars().peekable();
    let scheme = parse_scheme(&mut chars);
    assert_eq!(Err(UriParserError::InvalidScheme), scheme);
}

#[test]
fn parse_scheme_with_invalid_terminator_fails() {
    let text = "http*";
    let mut chars = text.chars().peekable();
    let scheme = parse_scheme(&mut chars);
    assert_eq!(Err(UriParserError::InvalidScheme), scheme);
}

#[test]
fn parse_authority_with_hostname_succeeds() {
    let text = "example.org";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::None, "example.org".into(), None)), authority);
}

#[test]
fn parse_authority_with_ipv6_succeeds() {
    let text = "[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::None, "[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]".into(), None)), authority);
}

#[test]
fn parse_authority_with_username_hostname_succeeds() {
    let text = "bob@example.org";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::Username("bob".into()), "example.org".into(), None)), authority);
}

#[test]
fn parse_authority_with_username_password_hostname_succeeds() {
    let text = "bob:pwd@example.org";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::UsernamePassword("bob".into(), "pwd".into()), "example.org".into(), None)), authority);
}

#[test]
fn parse_authority_with_username_ipv6_succeeds() {
    let text = "bob@[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::Username("bob".into()), "[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]".into(), None)), authority);
}

#[test]
fn parse_authority_with_username_password_ipv6_succeeds() {
    let text = "bob:pwd@[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::UsernamePassword("bob".into(), "pwd".into()), "[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]".into(), None)), authority);
}

#[test]
fn parse_authority_with_hostname_portnumber_succeeds() {
    let text = "example.org:8081";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::None, "example.org".into(), Some(8081))), authority);
}

#[test]
fn parse_authority_with_ipv6_portnumber_succeeds() {
    let text = "[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]:8081";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::None, "[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]".into(), Some(8081))), authority);
}

#[test]
fn parse_authority_with_username_hostname_portnumber_succeeds() {
    let text = "bob@example.org:8081";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::Username("bob".into()), "example.org".into(), Some(8081))), authority);
}

#[test]
fn parse_authority_with_username_password_hostname_portnumber_succeeds() {
    let text = "bob:pwd@example.org:8081";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::UsernamePassword("bob".into(), "pwd".into()), "example.org".into(), Some(8081))), authority);
}

#[test]
fn parse_authority_with_username_ipv6_portnumber_succeeds() {
    let text = "bob@[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]:8081";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::Username("bob".into()), "[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]".into(), Some(8081))), authority);
}

#[test]
fn parse_authority_with_username_password_ipv6_portnumber_succeeds() {
    let text = "bob:pwd@[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]:8081";
    let mut chars = text.chars().peekable();
    let authority = parse_authority(&mut chars);
    assert_eq!(Ok((UriCredentials::UsernamePassword("bob".into(), "pwd".into()), "[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]".into(), Some(8081))), authority);
}