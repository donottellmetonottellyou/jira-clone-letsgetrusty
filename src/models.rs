use std::collections::HashMap;

// TODO: derive the appropriate traits
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

// TODO: derive the appropriate traits
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u64>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

// TODO: derive the appropriate traits
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

// TODO: derive the appropriate traits
pub struct DBState {
    pub last_item_id: u64,
    pub epics: HashMap<u64, Epic>,
    pub stories: HashMap<u64, Story>,
}
