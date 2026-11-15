//! SigmaOS-gnome-system-monitor: GNOME System Monitor model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for GNOME System Monitor.
pub struct GnomeProcess {
    pub id: String,
    pub name: String,
    pub pid: String,
    pub enabled: bool,
}

/// Secondary domain entity for GNOME System Monitor.
pub struct GnomeResource {
    pub id: String,
    pub resource_type: String,
    pub description: String,
}

/// Errors for the GNOME System Monitor domain.
#[derive(Debug, PartialEq)]
pub enum GnomeSystemMonitorError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for GnomeSystemMonitorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for GnomeSystemMonitorError {}

/// Registry and manager for GNOME System Monitor.
pub struct GnomeSystemMonitorManager {
    pub name: String,
    items: BTreeMap<String, GnomeProcess>,
    configs: BTreeMap<String, GnomeResource>,
}

impl GnomeSystemMonitorManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: GnomeProcess) -> Result<(), GnomeSystemMonitorError> {
        if item.name.is_empty() {
            return Err(GnomeSystemMonitorError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(GnomeSystemMonitorError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&GnomeProcess> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: GnomeResource) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), GnomeSystemMonitorError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| GnomeSystemMonitorError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_GnomeProcess(id: &str, name: &str) -> GnomeProcess {
        GnomeProcess {
            id: id.to_string(),
            name: name.to_string(),
            pid: "gnome-system-monitor-default".to_string(),
            enabled: true,
        }
    }

    fn make_GnomeResource(id: &str, val: &str) -> GnomeResource {
        GnomeResource {
            id: id.to_string(),
            resource_type: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = GnomeSystemMonitorManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = GnomeSystemMonitorManager::new("test");
        let item = make_GnomeProcess("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = GnomeSystemMonitorManager::new("test");
        let item = make_GnomeProcess("item1", "");
        assert_eq!(mgr.add_item(item), Err(GnomeSystemMonitorError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = GnomeSystemMonitorManager::new("test");
        let item1 = make_GnomeProcess("item1", "First");
        let item2 = make_GnomeProcess("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(GnomeSystemMonitorError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = GnomeSystemMonitorManager::new("test");
        let cfg = make_GnomeResource("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = GnomeSystemMonitorManager::new("test");
        let item = make_GnomeProcess("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = GnomeSystemMonitorManager::new("test");
        let item = make_GnomeProcess("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(GnomeSystemMonitorError::NotFound("none".to_string())));
    }
}
