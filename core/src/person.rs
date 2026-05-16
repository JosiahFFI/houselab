//! Types relating to the people surrounding an inspection.

use serde::{Deserialize, Serialize};

/// A group of pre-saved people.
#[allow(missing_docs)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct People {
    pub inspectors: Vec<Inspector>,
    pub realtors: Vec<Realtor>,
}

/// Generic information about a person.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Person {
    /// The name of the person (first and last, as well as titles).
    pub name: String,
    /// Their phone number, if any.
    pub phone: Option<String>,
    /// Their email, if any.
    pub email: Option<String>,
}

/// An inspector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Inspector {
    /// Their generic information.
    pub info: Person,
    /// Any license numbers the inspector has (e.g. WA state license, SPI license, etc.).
    pub licenses: Vec<(String, String)>,
}

/// The client of an inspection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Client {
    /// Their generic information.
    pub info: Person,
    /// The client's (buyer's) realtor, if applicable.
    pub realtor: Option<Realtor>,
}

/// A realtor in a transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Realtor {
    /// Their generic information.
    pub info: Person,
    /// The firm/company they're with.
    pub firm: String,
}
