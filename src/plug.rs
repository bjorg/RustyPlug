#[derive(Debug, Clone)]
pub struct Plug {
    scheme: String,
    user: Option<String>,
    password: Option<String>,
    host: String,
    port: Option<u16>,
    segments: Vec<String>,
    query: Option<Vec<(String, Option<String>)>>,
    fragment: Option<String>,
    trailing_slash: bool
}

impl Plug {
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

    pub fn at(&self, segments: &[&str]) -> Plug {
        let mut new_segments = self.segments.to_vec();
        for &segment in segments.iter() {
            new_segments.push(segment.into());
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
