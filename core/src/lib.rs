//! Common types for Houselab.
#![deny(missing_docs)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod inspection;
pub mod person;
pub mod template;
pub mod test;

/// A comment about a condition of a component or system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    /// The base of the comment.
    ///
    /// May be templated using `{{field}}` syntax, with the fields being
    /// interpolated based on the later fields.
    pub base: String,

    /// Whether or not the comment is applied.
    ///
    /// Outside of an active inspection, this can be used to have
    /// automatically-selected comments.
    pub applied: bool,

    /// A mapping of variable names to lists of prefabricated entries.
    ///
    /// For a basic example, a list of electrical service drop kinds:
    /// `{ "kind": ["overhead", "underground"] }`
    pub lists: HashMap<String, List>,

    /// A list of text entry fields to be filled out in the comment.
    pub entries: HashMap<String, String>,

    /// The severity of the condition, as noted in the report summary.
    pub severity: Severity,

    /// Whether or not the comment will show up in the report summary.
    pub summary: bool,
}

/// A dropdown list in a [`Comment`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct List {
    /// The options to choose from.
    pub items: Vec<String>,

    /// The item that's currently selected.
    pub selected: usize,
}

/// The severity of a condition, as noted in the report summary.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// A condition not relating to immediate functionality or safety concerns.
    General,
    /// A condition immediately affecting proper operation of a system.
    Functionality,
    /// A condition immediately affecting the safety of a system.
    Safety,
}

/// An RGBA color.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
