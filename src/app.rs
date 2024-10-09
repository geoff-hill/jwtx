use crate::{cmd::Jwtx,cmd::Jwtx::*,conf::*};

pub fn run(cmd: Jwtx, conf: JwtxConfig) {
    match cmd {
        Token(env, ttype, endpoint) => {
            eprintln!("Token for {} {} {}", env, ttype, endpoint);
        }
        List(opt_env) => {
            match opt_env {
                Some(env) => {
                    // JwtxConfig.list(env)
                }
                None  => {
                    conf.list_all()
                }
                
            }            
        }
    };
}    
