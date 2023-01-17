use std::collections::HashMap;
use phf::phf_map;

#[derive(Copy)]
pub struct TCAPIVersion(&'static str);

impl From<&'static str> for TCAPIVersion {
    fn from(version: &'static str) -> Self {
        Self(version)
    }
}

impl ToString for TCAPIVersion {
    fn to_string(&self) -> String {
        String::from(self.0.clone())
    }
}

impl Clone for TCAPIVersion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub static V103: TCAPIVersion = TCAPIVersion("1.0.3");
pub static V102: TCAPIVersion = TCAPIVersion("1.0.2");
pub static V101: TCAPIVersion = TCAPIVersion("1.0.1");
pub static V100: TCAPIVersion = TCAPIVersion("1.0.0");
pub static V095: TCAPIVersion = TCAPIVersion("0.95");
pub static V090: TCAPIVersion = TCAPIVersion("0.9");

static KNOWN: phf::Map<&'static str, TCAPIVersion> = phf_map! {
    "1.0.3" => V103,
    "1.0.2" => V102,
    "1.0.1" => V101,
    "1.0.0" => V100,
    "0.95" => V095,
    "0.9" => V090
};
static SUPPORTED: phf::Map<&'static str, TCAPIVersion> = phf_map! {
    "1.0.3" => V103,
    "1.0.2" => V102,
    "1.0.1" => V101,
    "1.0.0" => V100
};

pub fn get_supported() -> &'static phf::Map<&'static str, TCAPIVersion> {
    &SUPPORTED
}

pub fn latest() -> TCAPIVersion {
    V103.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let version = TCAPIVersion::from("1.0.0");
        assert_eq!(version.0, "1.0.0");
    }

    #[test]
    fn test_to_string() {
        let version = TCAPIVersion::from("1.0.0");
        assert_eq!(version.to_string(), String::from("1.0.0"));
    }

    #[test]
    fn test_get_supported() {
        let supported = get_supported();
        assert_eq!(supported.len(), 4);
    }

    #[test]
    fn test_latest() {
        let version = latest();
        assert_eq!(version.0, "1.0.3");
    }
}