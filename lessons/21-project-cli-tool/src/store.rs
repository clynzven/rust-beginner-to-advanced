use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub priority: u8,
    pub done: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    next_id: u32,
    pub tasks: Vec<Task>,
}

impl Store {
    pub fn load(path: &str) -> Result<Store> {
        if !Path::new(path).exists() {
            return Ok(Store { next_id: 1, tasks: vec![] });
        }
        let text = std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
        serde_json::from_str(&text).with_context(|| format!("{path} is not a valid task file"))
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json).with_context(|| format!("writing {path}"))
    }

    pub fn add(&mut self, title: &str, priority: u8) -> &Task {
        let task = Task { id: self.next_id, title: title.to_string(), priority, done: false };
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    fn find(&mut self, id: u32) -> Result<&mut Task> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(t) => Ok(t),
            None => bail!("no task with id {id}"),
        }
    }

    pub fn complete(&mut self, id: u32) -> Result<&Task> {
        let task = self.find(id)?;
        task.done = true;
        Ok(task)
    }

    pub fn remove(&mut self, id: u32) -> Result<Task> {
        let pos = self.tasks.iter().position(|t| t.id == id);
        match pos {
            Some(i) => Ok(self.tasks.remove(i)),
            None => bail!("no task with id {id}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_complete_remove() {
        let mut s = Store { next_id: 1, tasks: vec![] };
        s.add("write tests", 1);
        s.add("ship it", 2);
        assert!(s.complete(1).unwrap().done);
        assert_eq!(s.remove(2).unwrap().title, "ship it");
        assert!(s.remove(99).is_err());
        assert_eq!(s.tasks.len(), 1);
    }
}
