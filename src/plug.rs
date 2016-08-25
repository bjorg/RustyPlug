use uri_parser;

#[derive(Debug, Clone)]
pub struct Plug {
    pub scheme: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub segments: Vec<String>,
    pub query: Option<Vec<(String, Option<String>)>>,
    pub fragment: Option<String>,
    pub trailing_slash: bool
}

impl Plug {

    // TOOD: missing methods
    // * with_credentials(username, password)
    // * without_credentials()
    // * without_query()
    // * with_trailing_slash(bool)

    pub fn new(uri: &str) -> Option<Plug> {
        let mut parser = uri.chars();
        let scheme = uri_parser::try_parse_scheme(&mut parser);
        if scheme == None {
            return None;
        }

        return Some(Plug {
            scheme: scheme.unwrap(),

            // TODO
            user: None,
            password: None,
            host: String::from(""),
            port: None,
            segments: vec![],
            query: None,
            fragment: None,
            trailing_slash: false,
        });
    }

    pub fn with_scheme(&self, scheme: String) -> Plug {
        return Plug { scheme: scheme, ..self.clone() };
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

    pub fn with_fragment(&self, fragment: &str) -> Plug {
        return Plug { fragment: Some(fragment.into()), ..self.clone() };
    }

    pub fn without_fragment(&self) -> Plug {
        return Plug { fragment: None, ..self.clone() };
    }
}

impl ToString for Plug {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.scheme);
        buffer.push_str("://");
        if let Some(ref user) = self.user {
            buffer.push_str(&user);
            if let Some(ref password) = self.password {
                buffer.push_str(":");
                buffer.push_str(&password);
            }
            buffer.push_str("@");
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
