use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Copy)]
enum ResourceType {
    Pod,
    Namespace,
    Deployment,
}

trait Resources {
    fn namespace(&self) -> &String;
    fn name(&self) -> &String;
    fn kind(&self) -> ResourceType;
}

macro_rules! impl_resources {
    ($i:ident) => {
        #[derive(Deserialize)]
        struct $i {
            namespace: String,
            name: String,
            kind: ResourceType,
        }

        impl Resources for $i {
            fn namespace(&self) -> &String {
                &self.namespace
            }
            fn name(&self) -> &String {
                &self.name
            }
            fn kind(&self) -> ResourceType {
                self.kind
            }
        }
    };
}

impl_resources!(Pod);
impl_resources!(Deployment);
impl_resources!(Namespace);

impl TryFrom<&str> for ResourceType {
    type Error = &'static str;
    fn try_from(val: &str) -> core::result::Result<Self, Self::Error> {
        match val {
            "Pod" => Ok(Self::Pod),
            "Namespace" => Ok(Self::Namespace),
            "Deployment" => Ok(Self::Deployment),
            _ => Err("unknown resource type"),
        }
    }
}

pub struct Cache {
    resources: HashMap<String, Box<dyn Resources>>,
}

fn deserialize_cache(reader: BufReader<File>, cache: &mut Cache) -> Result<(), Box<dyn Error>> {
    let mut content: Value = serde_json::from_reader(reader)?;
    let resources = content.as_array_mut().ok_or("deserialize cache file")?;
    for resource in resources {
        let kind: ResourceType = resource["kind"]
            .as_str()
            .ok_or("unable to find kind")?
            .try_into()?;
        match kind {
            ResourceType::Pod => {
                let result: Pod = serde_json::from_value(resource.take())?;
                cache
                    .resources
                    .insert(result.name.clone(), Box::new(result))
            }
            _ => {
                panic!("this shoud never happen cause we already panic in the enum conversion")
            }
        };
    }
    Ok(())
}

pub fn cache_init() -> Cache {
    let mut cache = Cache {
        resources: HashMap::new(),
    };
    if let Ok(file) = File::open("cache.json") {
        let reader = BufReader::new(file);
        deserialize_cache(reader, &mut cache).unwrap();
    }

    cache
}
