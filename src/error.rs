#![cfg_attr(feature = "cargo-clippy", allow(redundant_closure))]

use std::io;

use cobalt;
use liquid;
use serde_json;
use serde_yaml;
use toml;

error_chain! {

    links {
    }

    foreign_links {
        Io(io::Error);
        Cobalt(cobalt::Error);
        Liquid(liquid::Error);
        SerdeYaml(serde_yaml::Error);
        SerdeJson(serde_json::Error);
        Toml(toml::de::Error);
    }

    errors {
        ConfigFileMissingFields {
            description("missing fields in config file")
            display("name, description and link need to be defined in the config file to \
                    generate RSS")
        }

        UnsupportedPlatform(functionality: &'static str, platform: &'static str) {
            description("functionality is not implemented for this platform")
            display("{} is not implemented for the {} platform", functionality, platform)
        }
    }
}
