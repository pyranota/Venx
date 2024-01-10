use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct SmallBlockCollection {
    pub string_to_id: HashMap<String, usize>,
}
