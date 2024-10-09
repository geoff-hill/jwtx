use jwtx::{app,cmd,conf};
use type_cli::*;

// jwtx token dev client_flow
fn main() {
    let conf = conf::read_jwtx_from_file("test_yaml.yml");
    let cmd = cmd::Jwtx::process();
    app::run(cmd, conf);
}
 