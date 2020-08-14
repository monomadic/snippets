/*
Cargo.toml
toml = "*"
serde = { version = "1.0", features = ["derive"] }
*/

use toml::Value;
let value = "foo = 'bar'".parse::<Value>().unwrap();

// ----------------------------------------------------------------------------
// with serde

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

let config: Config = toml::from_str(r#"
        ip = '127.0.0.1'

        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
    "#).unwrap();
    
// ----------------------------------------------------------------------------
