use std::collections::HashMap;

use crate::{format_meta_line, prim::Prim, Axis, MetaValue};

/// This is the root of the file
#[derive(Default)]
pub struct Stage {
    pub meta: HashMap<String, MetaValue>,
    pub sub: Vec<Prim>,
}

impl Stage {
    /// Specify which axis is pointing up in space. Required for compliance
    pub fn set_up_axis(&mut self, axis: Axis) {
        self.meta.insert(
            "upAxis".to_string(),
            MetaValue::String(axis.text().to_string()),
        );
    }

    /// Set how long a space unit is in meters. Required for compliance
    pub fn set_scale(&mut self, meters_per_unit: f32) {
        self.meta.insert(
            "metersPerUnit".to_string(),
            MetaValue::Float(meters_per_unit),
        );
    }

    /// Set the name of the primitive used by default. Required for compliance
    pub fn set_default_prim(&mut self, default: String) {
        self.meta
            .insert("defaultPrim".to_string(), MetaValue::String(default));
    }

    pub fn push_sub(&mut self, sub_prim: Prim) {
        self.sub.push(sub_prim);
    }

    pub fn serialize_to_text(&self) -> String {
        // Magic number
        let mut build = String::from("#usda 1.0\n");

        // Metadata
        if !self.meta.is_empty() {
            build.push_str("(\n");
            for m in &self.meta {
                build.push_str(&format_meta_line(m.0, m.1));
            }
            build.push_str(")\n");
        }

        // Sub Prims
        for prim in &self.sub {
            build.push_str(&prim.serialize_to_text());
        }

        build
    }
}
