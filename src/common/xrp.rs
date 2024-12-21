use serde::{Deserialize, Serialize};
use typify_macro::import_types;

import_types!(schema="./schemas/common/xrp.json", derives=[Debug, Deserialize, Serialize]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "address": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
            "tag": 1234
        }"#;
        let xrp: XRP = serde_json::from_str(json).unwrap();
    }
}
