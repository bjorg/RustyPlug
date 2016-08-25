pub mod plug {

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
}
