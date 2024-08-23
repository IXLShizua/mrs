use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug)]
pub struct Identifier {
    pub namespace: String,
    pub value: String,
}

impl Identifier {
    pub fn new(namespace: Option<&str>, value: &str) -> Identifier {
        Identifier {
            namespace: namespace.unwrap_or("minecraft").to_string(),
            value: value.to_string(),
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized_string = format!("{}:{}", self.namespace, self.value);

        serializer.serialize_str(&serialized_string)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdentifierVisitor;

        impl<'de> Visitor<'de> for IdentifierVisitor {
            type Value = Identifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string in the format 'namespace:value'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Identifier, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = value.splitn(2, ':').collect();
                if parts.len() == 2 {
                    Ok(Identifier {
                        namespace: parts[0].to_string(),
                        value: parts[1].to_string(),
                    })
                } else {
                    Err(E::custom(format!(
                        "invalid format for Identifier: '{}'",
                        value
                    )))
                }
            }
        }

        deserializer.deserialize_str(IdentifierVisitor)
    }
}
