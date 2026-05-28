use hbb_common::ResultType;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct CustomServer {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub api: String,
    #[serde(default)]
    pub relay: String,
}

// IDME Remote — hard-coded rendezvous + relay server.
// The .exe ignores its filename and any user input here; both desktop and
// mobile builds end up calling this from their respective bootstrap paths
// (src/platform/windows.rs::bootstrap and the equivalent Android FFI init),
// which write the values into hbb_common::config::EXE_RENDEZVOUS_SERVER.
const IDME_HOST: &str = "remoto.idmesl.es";
const IDME_KEY: &str = "AYBh3beTiHVU+QIf2whUzuOCayNYeaGUDE5+TaUkvjQ=";

pub fn get_custom_server_from_string(_s: &str) -> ResultType<CustomServer> {
    Ok(CustomServer {
        host: IDME_HOST.to_owned(),
        key: IDME_KEY.to_owned(),
        api: String::new(),
        relay: String::new(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_always_returns_idme_server() {
        let expected = CustomServer {
            host: IDME_HOST.to_owned(),
            key: IDME_KEY.to_owned(),
            api: String::new(),
            relay: String::new(),
        };
        assert_eq!(get_custom_server_from_string("anything.exe").unwrap(), expected);
        assert_eq!(get_custom_server_from_string("").unwrap(), expected);
        assert_eq!(get_custom_server_from_string("rustdesk-licensed-XXX.exe").unwrap(), expected);
    }
}
