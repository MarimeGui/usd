pub mod prim;
pub mod stage;

fn indent_lines(in_text: &str) -> String {
    let mut build = String::new();
    for l in in_text.lines() {
        build.push_str("    ");
        build.push_str(l);
        build.push('\n');
    }
    build
}

fn format_meta_line(name: &str, value: &MetaValue) -> String {
    format!(
        "    {} = {}\n",
        name,
        match value {
            MetaValue::String(s) => {
                format!("\"{}\"", s)
            }
            MetaValue::Float(v) => v.to_string(),
        }
    )
}

pub enum MetaValue {
    //Dict(HashMap<String, MetaType>),
    String(String),
    Float(f32),
}

pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn text(&self) -> &'static str {
        match self {
            Axis::X => "X",
            Axis::Y => "Y",
            Axis::Z => "Z",
        }
    }
}
