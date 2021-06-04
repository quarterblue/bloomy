use serde::Deserialize;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct ApiList {
    pub mapper: HashMap<String, String>,
}

impl ApiList {
    pub fn new() -> Self {
        let mut new_mapper: HashMap<String, String> = HashMap::new();
        new_mapper.insert(
            "alpha_vantage".to_string(),
            "https://alphavantage.co/query?function=".to_string(),
        );
        new_mapper.insert(
            "iex_cloud".to_string(),
            "https://cloud.iexapis.com/stable/".to_string(),
        );
        return ApiList { mapper: new_mapper };
    }
}

pub struct Config {
    path: std::path::PathBuf,
    name: String,
    loaded: bool,
}

#[derive(Deserialize, Debug)]
pub struct KeyConfig {
    pub alpha_vantage: String,
    pub iex_cloud: String,
}

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<KeyConfig, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `KeyConfig`.
    let keyconfig = serde_json::from_reader(reader)?;

    // Return the `KeyConfig`.
    Ok(keyconfig)
}
// impl Config {
//     pub fn load_config(config_path: &String) -> Result<Config, std::io::Error> {
//         let path = Path::new(config_path);

//         let config = match File::open(&path) {
//             Err(err) => panic!("Could not open config file, {}", err),
//             Ok(config) => config,
//         };

//         let reader = BufReader::new(config);
//         let mut url = String::from("");
//         let mut key = String::from("");

//         for (index, line) in reader.lines().enumerate() {
//             match line {
//                 Ok(line) => {
//                     if index == 0 {
//                         url = String::from(&line);
//                     } else if index == 1 {
//                         key = String::from(&line);
//                     }
//                     println!("{} {}", index, line)
//                 }
//                 Err(e) => println!("Error: {}", e),
//             }
//         }

//         Ok(Config { url, key })
//     }
// }
