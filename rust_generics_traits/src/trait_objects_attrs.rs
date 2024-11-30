// -----------------------------------------------------------
// This example uses the "Trait Objects" feature to accomplish
// the need to have a collection of different types.
// As of Rust 2021, the `dyn` keyword must used.
// -----------------------------------------------------------

use std::fmt::Display;

#[derive(Debug)]
enum AttributeType {
    Text,
    Numeric,
}

impl Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeType::Text => write!(f, "text"),
            AttributeType::Numeric => write!(f, "numeric"),
        }
    }
}

trait Attribute {
    fn get_name(&self) -> String;
    fn get_type(&self) -> AttributeType;
    fn get_text_value(&self) -> Option<String>;
    fn get_numeric_value(&self) -> Option<i32>;
}

struct TextAttribute {
    name: String,
    value: String,
}

impl Attribute for TextAttribute {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> AttributeType {
        AttributeType::Text
    }

    fn get_text_value(&self) -> Option<String> {
        Some(self.value.clone())
    }

    fn get_numeric_value(&self) -> Option<i32> {
        None
    }
}

struct NumericAttribute {
    name: String,
    value: i32,
}

impl Attribute for NumericAttribute {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> AttributeType {
        AttributeType::Numeric
    }

    fn get_text_value(&self) -> Option<String> {
        None
    }

    fn get_numeric_value(&self) -> Option<i32> {
        Some(self.value)
    }
}

fn main() {
    //
    // Inits
    //

    let text_attr = TextAttribute {
        name: "greeting".to_string(),
        value: "Hello".to_string(),
    };

    let numeric_attr = NumericAttribute {
        name: "number".to_string(),
        value: 42,
    };

    //
    // Usage
    //

    let mut attrs: Vec<Box<dyn Attribute>> = Vec::new();
    attrs.push(Box::new(text_attr));
    attrs.push(Box::new(numeric_attr));

    println!("\n+------------+------------+------------+");
    println!(
        "| {0: <10} | {1: <10} | {2: <10} |",
        "type", "name", "value"
    );
    println!("+------------+------------+------------+");
    for attr in attrs {
        let value = match attr.get_type() {
            AttributeType::Text => format!("{}", attr.get_text_value().unwrap()),
            AttributeType::Numeric => format!("{:}", attr.get_numeric_value().unwrap()),
        };
        println!(
            "| {0:<10} | {1: <10} | {2: <10} |",
            attr.get_type().to_string(),
            attr.get_name(),
            value
        );
    }
    println!("+------------+------------+------------+\n");
}
