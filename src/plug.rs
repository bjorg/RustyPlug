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

    pub fn parse(uri: &str) -> Result<Plug, UriParserError> {
        let mut parser = uri.chars().peekable();
        let scheme = try!(parse_scheme(&mut parser));

        return Ok(Plug {
            scheme: scheme,

            // TODO
            credentials: PlugCredentials::None,
            host: String::from(""),
            port: None,
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
