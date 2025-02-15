use std::collections::HashMap;

use crate::{format_meta_line, indent_lines, MetaValue};

pub struct Prim {
    pub prim_type: PrimType,
    pub name: String,
    pub meta: HashMap<String, MetaValue>,
    pub attributes: HashMap<String, Attribute>,
    pub sub: Vec<Prim>,
}

impl Prim {
    pub fn new(prim_type: PrimType, name: String) -> Prim {
        Prim {
            prim_type,
            name,
            meta: HashMap::new(),
            attributes: HashMap::new(),
            sub: vec![],
        }
    }

    pub fn push_sub(&mut self, sub_prim: Prim) {
        self.sub.push(sub_prim);
    }

    pub fn serialize_to_text(&self) -> String {
        let mut build = format!("def {} \"{}\" ", self.prim_type.text(), self.name);

        // Metadata
        if !self.meta.is_empty() {
            build.push_str("(\n");
            for m in &self.meta {
                build.push_str(&format_meta_line(m.0, m.1));
            }
            build.push_str(")\n");
        }

        build.push_str("{\n");

        // Attributes
        for (k, v) in &self.attributes {
            build.push_str("    ");
            build.push_str(&v.format_line(k));
            build.push('\n');
        }

        // Sub Prims
        for s in &self.sub {
            build.push_str(&indent_lines(&s.serialize_to_text()));
        }

        build.push_str("}\n");

        build
    }
}

pub enum PrimType {
    Xform,
    Mesh,
}

impl PrimType {
    fn text(&self) -> &'static str {
        match self {
            PrimType::Xform => "Xform",
            PrimType::Mesh => "Mesh",
        }
    }
}

pub enum Attribute {
    String(String),
    IntArray(Vec<u32>), // TODO: Correct type ?
    FloatArray(Vec<f32>),
    Float3Array(Vec<[f32; 3]>),
    NormalizedFloat3Array(Vec<[f32; 3]>), // TODO: Custom normalized type ?
    PointFloat3Array(Vec<[f32; 3]>),
    TextureCoordinatesFloat2Array(Vec<[f32; 2]>),
}

impl Attribute {
    fn text_type(&self) -> &'static str {
        match self {
            Attribute::String(_) => "string",
            Attribute::IntArray(_) => "int[]",
            Attribute::FloatArray(_) => "float[]",
            Attribute::Float3Array(_) => "float3[]",
            Attribute::NormalizedFloat3Array(_) => "normal3f[]",
            Attribute::PointFloat3Array(_) => "point3f[]",
            Attribute::TextureCoordinatesFloat2Array(_) => "texCoord2f[]",
        }
    }

    fn value_to_string(&self) -> String {
        match self {
            Attribute::String(s) => format!("\"{}\"", s),
            Attribute::IntArray(items) => {
                let mut build = String::from("[");
                for (i, v) in items.iter().enumerate() {
                    build.push_str(&v.to_string());
                    if i < items.len() - 1 {
                        build.push_str(", ");
                    }
                }
                build.push(']');
                build
            }
            Attribute::FloatArray(_items) => todo!(),
            Attribute::Float3Array(items) => {
                let mut build = String::from("[");
                for (i, fa) in items.iter().enumerate() {
                    build.push_str(&format!("({}, {}, {})", fa[0], fa[1], fa[2]));
                    if i < items.len() - 1 {
                        build.push_str(", ");
                    }
                }
                build.push(']');
                build
            }
            Attribute::NormalizedFloat3Array(_items) => todo!(),
            Attribute::PointFloat3Array(items) => {
                // TODO: Same code as normal Float3Array
                let mut build = String::from("[");
                for (i, fa) in items.iter().enumerate() {
                    build.push_str(&format!("({}, {}, {})", fa[0], fa[1], fa[2]));
                    if i < items.len() - 1 {
                        build.push_str(", ");
                    }
                }
                build.push(']');
                build
            }
            Attribute::TextureCoordinatesFloat2Array(_items) => todo!(),
        }
    }

    fn format_line(&self, name: &str) -> String {
        let mut build = self.text_type().to_string();
        build.push(' ');
        build.push_str(name);
        build.push_str(" = ");
        build.push_str(&self.value_to_string());
        build
    }
}

/// Add vertices positions to Mesh Prim
pub fn set_points(prim: &mut Prim, vertices: Vec<[f32; 3]>) {
    prim.attributes
        .insert("points".to_string(), Attribute::PointFloat3Array(vertices));
}

/// Set indices of vertices forming faces in a flat array. Amount used per face is defined by ```set_face_counts```
pub fn set_face_vertex_indices(prim: &mut Prim, indices: Vec<u32>) {
    prim.attributes.insert(
        "faceVertexIndices".to_string(),
        Attribute::IntArray(indices),
    );
}

/// Set how many vertices each face has
pub fn set_face_vertex_counts(prim: &mut Prim, counts: Vec<u32>) {
    prim.attributes
        .insert("faceVertexCounts".to_string(), Attribute::IntArray(counts));
}
