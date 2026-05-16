//! Types relating to inspection templates.

use core::fmt;
use std::collections::HashMap;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{Color, Comment};

/// A template for an inspection, including prefab comments and styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    /// The name of the template.
    pub name: String,
    /// An optional description of the template.
    pub description: Option<String>,
    /// The last time the template was modified.
    pub last_modified: Timestamp,

    /// The (default) sections in the template.
    pub sections: Vec<Section>,

    /// The monotonically-increasing ID generator for sections.
    pub id_gen: u16,

    // TODO: provide more styling control
    /// The colors to use with styling.
    pub colors: HashMap<String, Color>,
}

impl Template {
    /// Resolve a path to a section or sub-section.
    pub fn get(&self, id: Id) -> Option<&Section> {
        for section in &self.sections {
            if let s @ Some(_) = section.get(id) {
                return s;
            }
        }

        None
    }

    /// Resolve a path to a section or sub-section.
    pub fn get_mut(&mut self, id: Id) -> Option<&mut Section> {
        for section in &mut self.sections {
            if let s @ Some(_) = section.get_mut(id) {
                return s;
            }
        }

        None
    }

    /// Resolve a path to a section or sub-section.
    pub fn path(&self, id: Id) -> Option<impl Iterator<Item = &str>> {
        let _ = self.get(id)?;

        let mut i = 0;
        let mut set = &self.sections[..];
        Some(std::iter::from_fn(move || {
            if i > id.level() {
                None
            } else {
                let bit = id.levels[i];

                for section in set {
                    if section.id.levels[i] == bit {
                        set = &section.children[..];
                        i += 1;
                        return Some(section.name.as_str());
                    }
                }

                unreachable!("ID was valid but path was not found; this should never happen");
            }
        }))
    }
}

/// A section in a template.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    /// The stable ID of this section.
    pub id: Id,

    /// The name of the section (e.g. "Kitchen", "Exterior", "Condition").
    pub name: String,
    /// An optional, short description of the section.
    ///
    /// Can be used to e.g. provide tips for inspecting a certain system/component.
    pub description: Option<String>,
    /// Whether or not to display this section inline with its parent, or to display it on its own
    /// page.
    ///
    /// Note that inline sections automatically inline all their children. Usually, adding children
    /// to an inline section is a mistake.
    pub inline: bool,

    // TODO: should these be one thing?
    /// Premade comments to be quickly applied to this section.
    pub comments: Vec<Comment>,
    /// Premade comments to be quickly applied to images in this section.
    pub observations: Vec<Comment>,

    /// Sections nested within this one.
    pub children: Vec<Section>,
}

impl Section {
    /// Resolve an ID to a section.
    pub fn get(&self, id: Id) -> Option<&Self> {
        let level = self.id.level() + 1;

        if id == self.id {
            return Some(self);
        } else if level == 8 || self.id.levels[..level] != id.levels[..level] {
            return None;
        }

        for child in &self.children {
            if child.id.levels[level] == id.levels[level] {
                return child.get(id);
            }
        }

        None
    }

    /// Resolve an ID to a section.
    pub fn get_mut(&mut self, id: Id) -> Option<&mut Self> {
        let level = self.id.level() + 1;

        if id == self.id {
            return Some(self);
        } else if level == 8 || self.id.levels[..level] != id.levels[..level] {
            return None;
        }

        for child in &mut self.children {
            if child.id.levels[level] == id.levels[level] {
                return child.get_mut(id);
            }
        }

        None
    }

    /// Update this section's ID, recursing into children as well.
    pub fn rebase(&mut self, id: Id) {
        self.id = id;

        for child in &mut self.children {
            let id = self
                .id
                .next()
                .expect("this already has children, therefore can't be level 7");
            child.rebase(id);
        }
    }
}

/// A unique ID assigned to a [`Section`].
///
/// IDs can be nested up to 8 levels deep, and each level can have up to 65535
/// siblings. The `last` field is used to generate new IDs for children of
/// this section, and is not considered part of the ID itself.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Id {
    levels: [u16; 8],
    last: u16,
}

impl core::cmp::PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.levels == other.levels
    }
}

impl core::cmp::Eq for Id {}

impl core::hash::Hash for Id {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.levels.hash(state);
    }
}

impl Id {
    /// Returns the zero ID.
    pub const ZERO: Self = Self::root(0);

    /// Returns the zero ID with the first level set to the given value.
    pub const fn root(level: u16) -> Self {
        Self {
            levels: [level, 0, 0, 0, 0, 0, 0, 0],
            last: 0,
        }
    }

    /// Returns the level (0-7) of this ID.
    pub fn level(self) -> usize {
        7 - self.levels.iter().rev().position(|&x| x != 0).unwrap_or(7)
    }

    /// Generate a new child ID from this ID.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Self> {
        self.last += 1;
        let level = self.level();

        if level == 7 {
            return None;
        }

        let mut id = *self;
        id.levels[level + 1] = self.last;
        id.last = 0;
        Some(id)
    }

    /// Get the parent ID of this ID, or `None` if this is a root ID.
    pub fn parent(&self) -> Option<Self> {
        let level = self.level();
        if level == 0 {
            None
        } else {
            let mut id = *self;
            id.levels[level] = 0;
            Some(id)
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.level())?;
        for level in &self.levels[..=self.level()] {
            write!(f, ":{:04x}", level)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Id;

    #[test]
    fn test_id_level() {
        let mut id = Id {
            levels: [1, 2, 0xFFFF, 3, 0xFFFF, 4, 0, 0],
            last: 0x0FF0,
        };
        assert_eq!(id.level(), 5);

        id.levels[7] = 1;
        assert_eq!(id.level(), 7);

        id.last = 0;
        assert_eq!(id.level(), 7);

        id.levels[7] = 0;
        id.levels[5] = 0;
        assert_eq!(id.level(), 4);

        id = Id::ZERO;
        id.last = 0xFFFF;
        assert_eq!(id.level(), 0)
    }
}
