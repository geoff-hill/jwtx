use std::fs::{self, read_to_string};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Environment {
    pub iss: String,
    pub tokens: HashMap<String,TokenSpec>,
}

#[derive(Serialize, Deserialize)]
pub enum TokenSpec {
    ClientCredentials { client_id: String, client_secret: String },
    UserPassword { user_name: String, password: String },
}


#[derive(Serialize, Deserialize)]
pub struct JwtxConfig {
    pub environments: HashMap<String, Environment>,
}

impl JwtxConfig {
    pub fn list_all(&self) {
       let yaml = to_yaml_string(self);
        println!("{}", yaml);
    }

    pub fn list(&self, env_name: &str) {
        match self.environment_with_name(env_name) {
            Some(env) => {
                let yaml = serde_yaml::to_string(env).unwrap();
                println!("{}", yaml);
            }
            None => {
                eprintln!("Environment {} not found", env_name);
            }
        }
    }

    pub fn environment_with_name(&self, name: &str) -> Option<&Environment> {
        self.environments.get(name)
    }
}




fn create_test_jwtx() -> JwtxConfig {
    let token = new_client_credentials_token(
        "client_id",
        "client_secret",
    );

    let token2 = new_user_password_token(
        "user_name",
        "password",
    );

    let mut tokens1 = HashMap::new();
    tokens1.insert("developer_client".to_string(), token);
    tokens1.insert("standard_user".to_string(), token2);
    let tokens1 = tokens1;
    
    let environment = Environment {
        iss: "https://dev.com".to_string(),
        tokens: tokens1,
    };
    
    let environment2 = Environment {
        iss: "https://dev.com".to_string(),
        tokens: HashMap::new(),
    };

    let mut env_map = HashMap::new();
    env_map.insert("dev".to_string(), environment);
    env_map.insert("syst".to_string(), environment2);
    
    let jwtx = JwtxConfig {
        // environments: vec![environment, environment2],
        environments: env_map,
    };
    
    jwtx
}

pub fn new_user_password_token(user_name: &str, password: &str) -> TokenSpec {
    let user_password = TokenSpec::UserPassword {
        user_name: user_name.to_string(),
        password: password.to_string(),
    };
    user_password
}

pub fn new_client_credentials_token(client_id: &str, client_secret: &str) -> TokenSpec {
    let client_credentials = TokenSpec::ClientCredentials {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
    }; 
    client_credentials
}


fn to_yaml_string(jwtx: &JwtxConfig) -> String {
    let yaml = serde_yaml::to_string(jwtx).unwrap();
    yaml
}

fn write_jwtx_yaml_to_file(jwtx: JwtxConfig, relative_path: &str) {
    fs::write(relative_path, to_yaml_string(&jwtx)).expect("Unable to write yaml");
}

fn write_jwtx_as_toml_to_file(jwtx: JwtxConfig, relative_path: &str) {
    let toml = toml::to_string(&jwtx).unwrap();
    fs::write(relative_path, toml).expect("Unable to write toml");
}

fn write_jwtx_as_json_to_file(jwtx: JwtxConfig, relative_path: &str) {
    let json = serde_json::to_string(&jwtx).unwrap();
    fs::write(relative_path, json).expect("Unable to write json");
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
    fn it_writes_toml_to_file() {
        let jwtx = create_test_jwtx();
        write_jwtx_as_toml_to_file(jwtx, "test_toml.toml");
    }

    #[test]
    fn it_writes_json_to_file() {
        let jwtx = create_test_jwtx();
        write_jwtx_as_json_to_file(jwtx, "test_json.json");
    }

    #[test]
    fn it_reads_yaml_from_file() {
        let jwtx = read_jwtx_from_file("test_yaml.yml");
        to_yaml_string(&jwtx);
    }
}
