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

use uri_parser::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PlugCredentials {
    None,
    Username(String),
    UsernamePassword(String, String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plug {
    scheme: String,
    credentials: PlugCredentials,
    host: String,
    port: Option<u16>,
    segments: Vec<String>,
    query: Option<Vec<(String, Option<String>)>>,
    fragment: Option<String>,
    trailing_slash: bool
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlugParserError {
    InternalError,
    InvalidScheme,
    InvalidHostname,
    InvalidIPv6,
    InvalidPortNumber,
    MissingColonSlashSlash
}

impl Plug {
    pub fn new(
        scheme: String,
        credentials: PlugCredentials,
        host: String,
        port: Option<u16>,
        segments: Vec<String>,
        query: Option<Vec<(String, Option<String>)>>,
        fragment: Option<String>,
        trailing_slash: bool
    ) -> Plug {
        return Plug {
            scheme: scheme,
            credentials: credentials,
            host: host,
            port: port,
            segments: segments,
            query: query,
            fragment: fragment,
            trailing_slash: trailing_slash
        };
    }

    pub fn parse(uri: &str) -> Result<Plug, PlugParserError> {
        let mut parser = uri.chars().peekable();
        let scheme = match parse_scheme(&mut parser) {
            Ok(value) => value,
            Err(UriParserError::InternalError) => return Err(PlugParserError::InternalError),
            Err(UriParserError::InvalidScheme) => return Err(PlugParserError::InvalidScheme),
            Err(UriParserError::InvalidHostname) => return Err(PlugParserError::InvalidHostname),
            Err(UriParserError::InvalidIPv6) => return Err(PlugParserError::InvalidIPv6),
            Err(UriParserError::InvalidPortNumber) => return Err(PlugParserError::InvalidPortNumber)
        };
        if (Some(':') != parser.next()) || (Some(':') != parser.next()) || (Some(':') != parser.next()) {
            return Err(PlugParserError::MissingColonSlashSlash);
        }
        let (credentials, host, port) = match parse_authority(&mut parser) {
            Ok(value) => value,
            Err(UriParserError::InternalError) => return Err(PlugParserError::InternalError),
            Err(UriParserError::InvalidScheme) => return Err(PlugParserError::InvalidScheme),
            Err(UriParserError::InvalidHostname) => return Err(PlugParserError::InvalidHostname),
            Err(UriParserError::InvalidIPv6) => return Err(PlugParserError::InvalidIPv6),
            Err(UriParserError::InvalidPortNumber) => return Err(PlugParserError::InvalidPortNumber)
        };
        return Ok(Plug {
            scheme: scheme,
            credentials: match credentials {
                UriCredentials::None => PlugCredentials::None,
                UriCredentials::Username(username) => PlugCredentials::Username(username),
                UriCredentials::UsernamePassword(username, password) => PlugCredentials::UsernamePassword(username, password)
            },
            host: host,
            port: port,

            // TODO
            segments: vec![],
            query: None,
            fragment: None,
            trailing_slash: false,
        });
    }

    pub fn get_scheme(&self) -> &str {
        return &self.scheme;
    }

    pub fn get_credentials(&self) -> &PlugCredentials {
        return &self.credentials;
    }

    pub fn get_host(&self) -> &str {
        return &self.host;
    }

    pub fn get_segments(&self) -> &[String] {
        return &self.segments;
    }

    pub fn get_query(&self) -> &Option<Vec<(String, Option<String>)>> {
        return &self.query;
    }

    pub fn get_fragment(&self) -> &Option<String> {
        return &self.fragment;
    }

    pub fn get_trailing_slash(&self) -> bool {
        return self.trailing_slash;
    }

    pub fn with_scheme(&self, scheme: String) -> Plug {
        return Plug { scheme: scheme, ..self.clone() };
    }

    pub fn with_credentials(&self, credentials: PlugCredentials) -> Plug {
        return Plug { credentials: credentials, ..self.clone() };
    }

    pub fn without_credentials(&self) -> Plug {
        return Plug { credentials: PlugCredentials::None, ..self.clone() };
    }

    pub fn with_host(&self, host: String) -> Plug {
        return Plug { host: host, ..self.clone() };
    }

    pub fn with_port(&self, port: u16) -> Plug {
        return Plug { port: Some(port), ..self.clone() };
    }

    pub fn without_port(&self) -> Plug {
        return Plug { port: None, ..self.clone() };
    }

    pub fn at(&self, segments: Vec<String>) -> Plug {
        let mut new_segments = self.segments.to_vec();
        for segment in segments.into_iter() {
            new_segments.push(segment);
        }
        return Plug { segments: new_segments, ..self.clone() };
    }

    pub fn without_path(&self) -> Plug {
        return Plug { segments: Vec::new(), ..self.clone() };
    }

    pub fn with(&self, key: String, value: String) -> Plug {
        let mut new_query = match self.query {
            Some(ref params) => params.to_vec(),
            None => Vec::new(),
        };
        new_query.push((key.into(), Some(value.into())));
        return Plug { query: Some(new_query), ..self.clone() };
    }

    pub fn without_query(&self) -> Plug {
        return Plug { query: None, ..self.clone() };
    }

    pub fn with_fragment(&self, fragment: &str) -> Plug {
        return Plug { fragment: Some(fragment.into()), ..self.clone() };
    }

    pub fn without_fragment(&self) -> Plug {
        return Plug { fragment: None, ..self.clone() };
    }

    pub fn with_trailing_slash(&self, trailing_slash: bool) -> Plug {
        return Plug { trailing_slash: trailing_slash, ..self.clone() };
    }

    pub fn without_trailing_slash(&self) -> Plug {
        return Plug { trailing_slash: false, ..self.clone() };
    }
}

impl ToString for Plug {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.scheme);
        buffer.push_str("://");
        match self.credentials  {
            PlugCredentials::None => (),
            PlugCredentials::Username(ref username) => {
                buffer.push_str(&username);
                buffer.push_str("@");
            },
            PlugCredentials::UsernamePassword(ref username, ref password) => {
                buffer.push_str(&username);
                buffer.push_str(":");
                buffer.push_str(&password);
                buffer.push_str("@");
            }
        }
        buffer.push_str(&self.host);
        if let Some(port) = self.port {
            buffer.push_str(&format!(":{}", port));
        }
        for segment in self.segments.iter() {
            buffer.push_str("/");
            buffer.push_str(&segment);
        }
        if self.trailing_slash {
            buffer.push_str("/");
        }
        if let Some(ref query) = self.query {
            buffer.push_str("?");
            for &(ref key, ref optional_value) in query.iter() {
                buffer.push_str(&key);
                if let &Some(ref value) = optional_value {
                    buffer.push_str("=");
                    buffer.push_str(&value);
                }
            }
        }
        if let Some(ref fragment) = self.fragment {
            buffer.push_str("#");
            buffer.push_str(fragment);
        }
        return buffer;
    }
}
