use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use tiny_fail::{ErrorMessageExt, Fail};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Criteria {
    enum_split_filelds: HashMap<String, String>,
}

impl Criteria {
    pub fn new() -> Criteria {
        Criteria::default()
    }

    pub fn add(&mut self, field_name: &str, type_field: &str) {
        self.enum_split_filelds
            .insert(field_name.to_owned(), type_field.to_owned());
    }

    pub fn is_split_enum(&self, field_path: &str) -> Option<&str> {
        self.enum_split_filelds.get(field_path).map(String::as_str)
    }
}

#[derive(Debug, Clone)]
pub struct Criterias(HashMap<String, Criteria>);

impl Criterias {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Criterias, Fail> {
        let path = path.as_ref();
        let f = File::open(path).err_msg(format!("failed load config file '{:?}'", path))?;
        let r = BufReader::new(f);

        let criterias: HashMap<String, Criteria> =
            from_reader(r).err_msg("failed parse config file")?;
        Ok(Criterias(criterias))
    }

    pub fn get(&self, target_name: &str) -> Criteria {
        self.0.get(target_name).cloned().unwrap_or_default()
    }
}
