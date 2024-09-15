use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommanderData {
    pub command_name: String,
    pub ip: String,
}

impl CommanderData {
    pub fn deserialize(data: &str) -> Result<CommanderData, String> {
        toml::from_str::<CommanderData>(data)
            .map_err(|e| format!("Could not create CommanderData from {data}: {e}"))
    }

    pub fn serialize(&self) -> Result<String, String> {
        toml::to_string(&self)
            .map_err(|e| format!("Could not serialize CommanderData {:?}: {e}", &self))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
// use one char for each field, to minify serialized size
pub struct ServerData {
    pub c: String, // command name
    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    pub d: u128, // deadline in ns
    pub s: u8,     // strict - 0 == false, 1 == true
    pub i: Option<String>, // ip address
}

impl ServerData {
    pub fn create(
        command: &str,
        deadline: u16,
        strict: bool,
        ip: Option<String>,
        now_ns: u128,
    ) -> ServerData {
        ServerData {
            c: command.to_string(),
            d: now_ns + (u128::from(deadline) * 1_000_000_000),
            s: if strict { 1 } else { 0 },
            i: ip,
        }
    }

    pub fn deserialize(data: &[u8]) -> Result<ServerData, String> {
        let data_str = String::from_utf8_lossy(data).to_string();
        toml::from_str::<ServerData>(&data_str)
            .map_err(|e| format!("Could not deserialize ServerData {data_str}: {e}"))
    }

    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        toml::to_string(&self)
            .map(|s| s.trim().replace(" = ", "=").as_bytes().to_vec())
            .map_err(|e| format!("Could not serialize data for server {:?}: {e}", &self))
    }

    pub fn is_strict(&self) -> bool {
        self.s == 1
    }

    pub fn ip(&self) -> Option<String> {
        self.i.clone()
    }

    pub fn deadline(&self) -> u128 {
        self.d
    }
}

// Custom serialize function for timestamp
fn serialize<S>(timestamp: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Serialize the timestamp as a string
    serializer.serialize_str(&timestamp.to_string())
}

// Custom deserialize function for timestamp
fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the timestamp from a string
    let s = String::deserialize(deserializer)?;
    s.parse::<u128>().map_err(D::Error::custom)
}