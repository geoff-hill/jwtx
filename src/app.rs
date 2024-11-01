use crate::{cmd::Jwtx,cmd::Jwtx::*,conf::*,};

pub fn run(cmd: Jwtx, conf: JwtxConfig) {
    match cmd {
        Token(env, token_type) => {
            match conf.environment_with_name(&env) {
                Some(environment) => {
                    let token = environment.tokens.get(&token_type);
                    match token {
                        Some(t) => {
                            match t {
                                TokenSpec::ClientCredentials { client_id, client_secret } => {
                                    println!("ClientCredentials: client_id: {}, client_secret: {}", client_id, client_secret);
                                }
                                TokenSpec::UserPassword { user_name, password } => {
                                    println!("UserPassword: user_name: {}, password: {}", user_name, password);
                                }
                            }
                        }
                        None => {
                            eprintln!("Token type {} not found", token_type);
                        }
                    }
                }
                None => {
                    eprintln!("Environment {} not found", env);
                }
            };
        }
        List(opt_env) => {
            match opt_env {
                Some(env) => {
                    conf.list(&env);
                }
                None  => {
                    conf.list_all()
                }
                
            }            
        }
    };
}    
