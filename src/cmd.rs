use type_cli::CLI;

#[derive(CLI)]
#[help = "Build manager tool for rust"]
pub enum Jwtx {
    #[help = "Get a token"]
    Token(#[named = "environment"] String, String, String),
    List(#[optional]Option<String>),
}

