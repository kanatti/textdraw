use anyhow::{Result, bail};

/// Trait for elements that expose editable properties
pub trait HasProperties {
    /// Get the specification for this element's properties
    fn properties_spec(&self) -> PropertiesSpec;

    /// Get the current value of a property by name
    fn get_property(&self, name: &str) -> Option<PropertyValue>;

    /// Set a property by name
    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<()>;
}

/// Specification for an element's properties
#[derive(Debug, Clone)]
pub struct PropertiesSpec {
    pub sections: Vec<PropertySection>,
}

impl PropertiesSpec {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: PropertySection) {
        self.sections.push(section);
    }

    /// Builder method to add a section with a closure
    pub fn section(mut self, title: impl Into<String>, builder: impl FnOnce(PropertySection) -> PropertySection) -> Self {
        let section = builder(PropertySection::new(title));
        self.sections.push(section);
        self
    }

    /// Get all fields flattened across sections
    pub fn all_fields(&self) -> Vec<&PropertyField> {
        self.sections
            .iter()
            .flat_map(|section| section.fields.iter())
            .collect()
    }

    /// Get a field by name
    pub fn get_field(&self, name: &str) -> Option<&PropertyField> {
        self.all_fields().into_iter().find(|f| f.name == name)
    }
}

impl Default for PropertiesSpec {
    fn default() -> Self {
        Self::new()
    }
}

/// A section grouping related properties
#[derive(Debug, Clone)]
pub struct PropertySection {
    pub title: String,
    pub fields: Vec<PropertyField>,
}

impl PropertySection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: PropertyField) {
        self.fields.push(field);
    }

    /// Builder method to add a numeric field
    pub fn numeric(mut self, name: impl Into<String>, label: impl Into<String>, min: u16, max: u16) -> Self {
        self.fields.push(PropertyField::new(name, label, FieldType::Numeric { min, max }));
        self
    }

    /// Builder method to add a text field
    pub fn text(mut self, name: impl Into<String>, label: impl Into<String>, max_length: usize) -> Self {
        self.fields.push(PropertyField::new(name, label, FieldType::Text { max_length }));
        self
    }

    /// Builder method to add a choice field
    pub fn choice(mut self, name: impl Into<String>, label: impl Into<String>, options: Vec<String>) -> Self {
        self.fields.push(PropertyField::new(name, label, FieldType::Choice { options }));
        self
    }

    /// Builder method to add a boolean field
    pub fn boolean(mut self, name: impl Into<String>, label: impl Into<String>) -> Self {
        self.fields.push(PropertyField::new(name, label, FieldType::Boolean));
        self
    }
}

/// A single property field
#[derive(Debug, Clone)]
pub struct PropertyField {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
}

impl PropertyField {
    pub fn new(name: impl Into<String>, label: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            field_type,
        }
    }
}

/// Type of a property field
#[derive(Debug, Clone)]
pub enum FieldType {
    Numeric { min: u16, max: u16 },
    Text { max_length: usize },
    Choice { options: Vec<String> },
    Boolean,
}

/// Value of a property
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Numeric(u16),
    Text(String),
    Choice(String),
    Boolean(bool),
}

impl PropertyValue {
    /// Try to extract a numeric value
    pub fn as_numeric(&self) -> Result<u16> {
        match self {
            PropertyValue::Numeric(n) => Ok(*n),
            _ => bail!("Property value is not numeric"),
        }
    }

    /// Try to extract a text value
    pub fn as_text(&self) -> Result<&str> {
        match self {
            PropertyValue::Text(s) => Ok(s),
            _ => bail!("Property value is not text"),
        }
    }

    /// Try to extract a choice value
    pub fn as_choice(&self) -> Result<&str> {
        match self {
            PropertyValue::Choice(s) => Ok(s),
            _ => bail!("Property value is not a choice"),
        }
    }

    /// Try to extract a boolean value
    pub fn as_boolean(&self) -> Result<bool> {
        match self {
            PropertyValue::Boolean(b) => Ok(*b),
            _ => bail!("Property value is not boolean"),
        }
    }
}
