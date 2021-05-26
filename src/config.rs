pub struct config {
    path: PathBuf,
    name: String,
    loaded: bool,
}

// pub struct Config {
//     pub url: String,
//     pub key: String,
// }

/**
 * Config file loader, config.txt must be in the following format:
 *
 * Line 0: URL
 * Line 1: KEY
 *
 * Example:
 * https://www.stockapisite.com/api/v2/
 * OAdawjiofWA20489ajiofwajoi
 *
 */
pub fn load_config(config_path: &String) -> Result<Config, std::io::Error> {
    let path = Path::new(config_path);

    let config = match fs::File::open(&path) {
        Err(err) => panic!("Could not open config file, {}", err),
        Ok(config) => config,
    };

    let reader = BufReader::new(config);
    let mut url = String::from("");
    let mut key = String::from("");

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if index == 0 {
                    url = String::from(&line);
                } else if index == 1 {
                    key = String::from(&line);
                }
                println!("{} {}", index, line)
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(Config { url, key })
}
