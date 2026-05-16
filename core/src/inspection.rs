//! Types relating to an inspection as a whole.

use std::collections::HashMap;

use jiff::civil::{Date, Time};
use serde::{Deserialize, Serialize};

use crate::Comment;

/// A whole inspection, as recorded onsite and edited afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inspection {
    /// The name of the inspection.
    pub name: String,
    /// The address of the inspected property.
    pub address: String,
    /// The date of the on-site inspection. Not the date of the last modification.
    pub date: Date,
    /// The time of the on-site inspection.
    pub time: Time,
    /// The template used for the inspection.
    ///
    /// Note that this is carried alongside the inspection wholesale, not referenced. This is to
    /// ensure that any future changes can be made in accordance with the template, and to prevent
    /// breakage if the original template was modified/deleted.
    pub template: crate::template::Template,

    /// The inspectors for a given inspection.
    ///
    /// If there is a primary inspector, list them first.
    pub inspectors: Vec<crate::person::Inspector>,
    /// The client of the inspection.
    pub client: crate::person::Client,
    /// The seller's realtor for the inspection.
    pub seller: Option<crate::person::Realtor>,

    /// The images bundled with the inspection.
    ///
    /// Each path specifies a section in the template, e.g. `Interior -> Kitchen -> Appliances`, and
    /// all the images in that section.
    pub images: HashMap<crate::template::Id, Vec<Image>>,
}

/// An image embedded in an inspection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    /// The comment associated with the image, if any.
    pub comment: Option<Comment>,
    /// The actual image data.
    ///
    /// Note that this is full-resolution; the image is not downscaled or modified in any way until
    /// the report is generated.
    pub data: (), // TODO: how to store images?
    /// Any overlays made to the image. Note that these are not fully applied until the report is
    /// generated.
    pub overlays: Vec<Overlay>,
}

/// An overlay applied to an image.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Overlay {
    pub shape: Shape,
    pub color: crate::Color,
}

/// A kind of overlay.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Line { start: Pos, end: Pos },
    Arrow { start: Pos, end: Pos },
    Rect { upper_left: Pos, bottom_right: Pos },
    Circle { center: Pos, radius: f64 },
    Text { start: Pos, body: String },
}

/// A 2D position within an image.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}
