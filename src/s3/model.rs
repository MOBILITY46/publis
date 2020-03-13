use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Statement {
    sid: String,
    effect: String,
    principal: std::collections::HashMap<String, String>,
    action: String,
    resource: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BucketPolicy {
    version: String,
    id: String,
    statement: Vec<Statement>,
}

impl BucketPolicy {
    pub fn website(bucket: &str) -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert("AWS".into(), "*".into());

        Self {
            version: "2008-10-17".into(),
            id: "PolicyForPublicWebsiteContent".into(),
            statement: vec![Statement {
                sid: "PublicReadGetObject".into(),
                effect: "Allow".into(),
                resource: format!("arn:aws:s3:::{}/*", bucket),
                action: "s3:GetObject".into(),
                principal: map,
            }],
        }
    }
}
