// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-omamac: omamac component model.
//!
//! Real implementation: omamac_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmamacItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmamacError {
    EmptyOmamacItemName,
    DuplicateOmamacItem(String),
    EmptyEventSubject,
}

impl fmt::Display for OmamacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOmamacItemName => write!(f, "omamac_item name must not be empty"),
            Self::DuplicateOmamacItem(id) => write!(f, "duplicate omamac_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for OmamacError {}

pub struct Omamac {
    pub name: String,
    omamac_items: BTreeMap<String, OmamacItem>,
    events: BTreeMap<String, Event>,
}

impl Omamac {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            omamac_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_omamac_item(&mut self, item: OmamacItem) -> Result<(), OmamacError> {
        if item.name.trim().is_empty() {
            return Err(OmamacError::EmptyOmamacItemName);
        }
        let id = item.id.clone();
        if self.omamac_items.contains_key(&id) {
            return Err(OmamacError::DuplicateOmamacItem(id.clone()));
        }
        self.omamac_items.insert(id, item);
        Ok(())
    }

    pub fn omamac_item_count(&self) -> usize {
        self.omamac_items.len()
    }

    pub fn get_omamac_item(&self, id: &str) -> Option<&OmamacItem> {
        self.omamac_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), OmamacError> {
        if event.subject.trim().is_empty() {
            return Err(OmamacError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_omamac_item(id: &str, name: &str) -> OmamacItem {
        OmamacItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Omamac::new("Omamac");
        assert_eq!(m.name, "Omamac");
        assert_eq!(m.omamac_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_omamac_item() {
        let mut m = Omamac::new("Omamac");
        m.add_omamac_item(sample_omamac_item("a1", "Alpha")).unwrap();
        assert_eq!(m.omamac_item_count(), 1);
        assert!(m.get_omamac_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Omamac::new("Omamac");
        assert!(m.add_omamac_item(sample_omamac_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Omamac::new("Omamac");
        m.add_omamac_item(sample_omamac_item("x", "One")).unwrap();
        assert!(m.add_omamac_item(sample_omamac_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Omamac::new("Omamac");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
