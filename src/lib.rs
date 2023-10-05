mod errors;
mod validate;

use core::time::{Duration};
use std::collections::HashMap;
use std::error::Error;
use std::{fs, thread};
use std::sync::mpsc;

use http::status::{StatusCode};
use log::{error, info};
use rdev::{grab, Event, EventType, Key};
use reqwest;
use serde::{Deserialize};

use errors::VibinRemoteError;

#[derive(Clone, Debug, Deserialize)]
struct KeyConfig {
    url: String,
}

#[derive(Deserialize)]
struct AppConfig {
    vibin: String,
    #[serde(default = "default_url_timeout")]
    request_timeout: Option<u8>,
    keymap: HashMap<String, KeyConfig>,
}

fn default_url_timeout() -> Option<u8> {
    Some(1)
}

/// Create a HashMap to associate each registered rdev::Key with a KeyConfig.
fn generate_key_to_keyconfig(app_config: &AppConfig) -> Result<HashMap<Key, KeyConfig>, Box<dyn Error>> {
    let mut map: HashMap<Key, KeyConfig> = HashMap::new();

    // Take the keymap data from the app config and process each of the keymap keys. Create a
    // HashMap where the keys are an rdev::Key and the values are a KeyConfig. This allows us to
    // map a keypress to a URL to invoke.
    for key_name in app_config.keymap.keys() {
        map.insert(validate::validate_key_name(key_name)?, app_config.keymap[key_name].clone());
    }

    Ok(map)
}

/// Listen for key presses. For each key press, check if it's a registered rdev::Key and invoke the
/// associated vibin URL.
///
/// This function blocks.
fn process_keypresses(
    app_config: AppConfig, key_to_keyconfig: HashMap<Key, KeyConfig>,
) -> Result<(), Box<dyn Error>> {
    let (url_executor_tx, url_executor_rx) = mpsc::channel();

    // URL executor. Retrieves urls off the message queue and sends them to vibin.
    thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        let timeout = app_config.request_timeout.unwrap() as u64;

        for url in url_executor_rx {
            match client.post(&url).timeout(Duration::new(timeout, 0)).send() {
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::OK => {}
                        code => error!("HTTP error [{code}] {url}"),
                    }
                }
                Err(e) => error!("{e}"),
            }
        }
    });

    // Keypress listener. Listens for registered key presses, takes the associated URL for the
    // registered key, and sends the URLs to the URL executor.
    let keypress_listener = move |event: Event| -> Option<Event> {
        match event.event_type {
            EventType::KeyRelease(key) => {
                if let Some(key_config) = key_to_keyconfig.get(&key) {
                    let url = format!("http://{}{}", app_config.vibin, key_config.url);

                    match url_executor_tx.send(url.clone()) {
                        Ok(_) => info!("{:?} -> {}", key, url),
                        Err(e) => error!("Could not send URL to command executor: {e}"),
                    }
                }
            }
            _ => {}
        }

        Some(event)
    };

    info!("Listening for key presses...");

    // Listen for key presses. This blocks until the main thread is cancelled.
    if let Err(error) = grab(keypress_listener) {
        error!("Keypress listener error: {:?}", error);
    }

    Ok(())
}

/// Run vibinremote using the configuration defined in `config_file`.
pub fn run(config_file: &str) -> Result<(), Box<dyn Error>> {
    let key_mapper_json = fs::read_to_string(config_file)
        .map_err(|e| VibinRemoteError::AppConfigError(format!("Could not load config data: {e}").to_string()))?;

    let app_config: AppConfig = serde_json::from_str(&key_mapper_json)
        .map_err(|e| VibinRemoteError::AppConfigError(format!("Could not parse config data: {e}").to_string()))?;

    let key_to_keyconfig = generate_key_to_keyconfig(&app_config)?;

    info!("Using vibin at: {} (http timeout: {}s)", app_config.vibin, app_config.request_timeout.unwrap());
    info!("Registered {} keys for intercept", key_to_keyconfig.keys().len());

    process_keypresses(app_config, key_to_keyconfig)?;

    Ok(())
}

// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rdev::Key;
    use crate::{AppConfig, generate_key_to_keyconfig, KeyConfig};

    #[test]
    fn it_generates_keyconfig() {
        // Setup.
        let mut test_keymap: HashMap<String, KeyConfig> = HashMap::new();
        let up_url = "/url/path/up";
        let down_url = "/url/path/down";

        let keyconfig_up = KeyConfig { url: up_url.to_string() };
        let keyconfig_down = KeyConfig { url: down_url.to_string() };

        test_keymap.insert("PageUp".to_string(), keyconfig_up);
        test_keymap.insert("PageDown".to_string(), keyconfig_down);

        // Code under test.
        let keyconfig = generate_key_to_keyconfig(&AppConfig {
            vibin: "vibin.local".to_string(),
            request_timeout: Some(1),
            keymap: test_keymap,
        }).unwrap();

        // Expect to get back a HashMap of rdev::Keys to the expected KeyConfig urls
        assert_eq!(keyconfig.len(), 2);
        assert_eq!(keyconfig[&Key::PageUp].url, up_url);
        assert_eq!(keyconfig[&Key::PageDown].url, down_url);
    }
}

// TODO: Add tests for process_keypresses():
//  - registered keypress invokes expected http request
//  - unregistered keypress is ignored