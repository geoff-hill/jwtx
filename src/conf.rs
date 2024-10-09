use std::fs::{self, read_to_string};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub iss: String,
    pub tokens: Vec<Token>,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token_type: String,
    pub oauth_flow: OAuthFlow,
    pub client_credentials: Option<ClientCredentials>,
    pub user_password: Option<UserPassword>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserPassword {
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum OAuthFlow {
    ClientCredentialsFlow,
    UserPassword,
}

#[derive(Serialize, Deserialize)]
pub struct JwtxConfig {
    pub environments: Vec<Environment>,
}

impl JwtxConfig {
    pub fn list_all(&self) {
       let yaml = to_yaml_string(self);
        println!("{}", yaml);
    }

    pub fn list(&self, env_name: &str) {
        for env in &self.environments {
            if env.name == env_name {
                for token in &env.tokens {
                    println!("{} {} {}", token.token_type, env.name, env.iss);
                }
            }
        }
    }
}




fn create_test_jwtx() -> JwtxConfig {
    let token = Token {
        token_type: "access".to_string(),
        oauth_flow: OAuthFlow::ClientCredentialsFlow,
        client_credentials: Some(ClientCredentials {
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
        }),
        user_password: None,
    };
    
    let environment = Environment {
        name: "dev".to_string(),
        iss: "https://dev.com".to_string(),
        tokens: vec![token],
    };
    
    let environment2 = Environment {
        name: "syst".to_string(),
        iss: "https://dev.com".to_string(),
        tokens: Vec::new(),
    };
    
    let jwtx = JwtxConfig {
        environments: vec![environment, environment2],
    };
    
    jwtx
}

fn to_yaml_string(jwtx: &JwtxConfig) -> String {
    let yaml = serde_yaml::to_string(jwtx).unwrap();
    yaml
}

fn write_jwtx_yaml_to_file(jwtx: JwtxConfig, relative_path: &str) {
    fs::write(relative_path, to_yaml_string(&jwtx)).expect("Unable to write yaml");
}

pub fn read_jwtx_from_file(relative_path: &str) -> JwtxConfig {
    let file_contents = read_to_string(relative_path).expect("Cannot read config file {relative_path}");
    let jwtx: JwtxConfig = serde_yaml::from_str::<JwtxConfig>(&file_contents).expect("Cannot parse file {relative_path}");
    jwtx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_writes_yaml_to_file() {
        let jwtx = create_test_jwtx();
        write_jwtx_yaml_to_file(jwtx, "test_yaml.yml");
    }

    #[test]
    fn it_reads_yaml_from_file() {
        let jwtx = read_jwtx_from_file("test_yaml.yml");
        to_yaml_string(&jwtx);
    }
}
