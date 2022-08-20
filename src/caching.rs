use std::collections::HashMap;

use serde::Deserialize;

enum ResourceType {
    Pod,
    Namespace,
    Deployment,
}

trait Resources {
    fn namespace(&self) -> String;
    // fn name(&self) -> String;
    // fn r#type(&self) -> ResourceType;
    fn serialize(&self) -> String;
}

pub struct Cache {
    resources: HashMap<String, Box<dyn Resources>>,
}

impl<'de> Deserialize<'de> for Cache {
    fn deserialize<D>(_: D) -> std::result::Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

pub fn cache_init() -> Cache {
    let result = Cache {
        resources: HashMap::new(),
    };

    result
}
