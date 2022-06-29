use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};

pub use scale_info::{TypeDefPrimitive, TypeParameter};

#[derive(Clone, Debug, Default)]
pub struct TypeForm;

impl scale_info::form::Form for TypeForm {
  type Type = TypeId;
  type String = String;
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Decode, Encode)]
pub struct Path {
  pub segments: Vec<String>,
}

impl Path {
  pub fn new(ident: &str, module_path: &str) -> Self {
    let mut segments = module_path
      .split("::")
      .map(|s| s.into())
      .collect::<Vec<_>>();
    segments.push(ident.into());
    Self { segments }
  }

  pub fn is_empty(&self) -> bool {
    self.segments.is_empty()
  }

  pub fn ident(&self) -> Option<&str> {
    self.segments.last().map(|s| s.as_str())
  }

  pub fn namespace(&self) -> &[String] {
    self.segments.split_last().map(|(_, ns)| ns).unwrap_or(&[])
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct Type {
  #[serde(skip_serializing_if = "Path::is_empty", default)]
  pub path: Path,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub type_params: Vec<TypeParameter<TypeForm>>,
  #[serde(rename = "def")]
  pub type_def: TypeDef,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub docs: Vec<String>,
}

impl Type {
  pub fn new(name: &str, type_def: TypeDef) -> Self {
    Self {
      path: Path::new(name, ""),
      type_def,
      type_params: Default::default(),
      docs: Default::default(),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct Field {
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub name: Option<String>,
  #[serde(rename = "type")]
  pub ty: TypeId,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub type_name: Option<String>,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub docs: Vec<String>,
}

impl Field {
  pub fn new(ty: TypeId) -> Self {
    Self {
      name: None,
      ty,
      type_name: None,
      docs: Vec::new(),
    }
  }

  pub fn new_named(name: &str, ty: TypeId, type_name: Option<String>) -> Self {
    Self {
      name: Some(name.into()),
      ty,
      type_name,
      docs: Vec::new(),
    }
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Decode, Encode)]
pub struct Variant {
  pub name: String,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub fields: Vec<Field>,
  pub index: u8,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub docs: Vec<String>,
}

impl Variant {
  pub fn new(name: &str, fields: Vec<Field>, index: u8) -> Self {
    Self {
      name: name.into(),
      fields,
      index,
      docs: Vec::new(),
    }
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Decode, Encode)]
pub struct TypeDefVariant {
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub variants: Vec<Variant>,
}

impl TypeDefVariant {
  pub fn new(variants: Vec<Variant>) -> Self {
    Self { variants }
  }

  pub fn new_option(ty: TypeId) -> Self {
    Self {
      variants: vec![
        Variant::new("None", vec![], 0),
        Variant::new("Some", vec![Field::new(ty)], 1),
      ],
    }
  }

  pub fn new_result(ok_ty: TypeId, err_ty: TypeId) -> Self {
    Self {
      variants: vec![
        Variant::new("Ok", vec![Field::new(ok_ty)], 0),
        Variant::new("Err", vec![Field::new(err_ty)], 1),
      ],
    }
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Decode, Encode)]
pub struct TypeDefComposite {
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub fields: Vec<Field>,
}

impl TypeDefComposite {
  pub fn new(fields: Vec<Field>) -> Self {
    Self { fields }
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Decode, Encode)]
#[serde(transparent)]
pub struct TypeDefTuple {
  pub fields: Vec<TypeId>,
}

impl TypeDefTuple {
  pub fn new(fields: Vec<TypeId>) -> Self {
    Self { fields }
  }

  pub fn new_type(field: TypeId) -> Self {
    Self {
      fields: vec![field],
    }
  }

  pub fn unit() -> Self {
    Self::new(vec![])
  }

  pub fn is_unit(&self) -> bool {
    self.fields.is_empty()
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct TypeDefSequence {
  #[serde(rename = "type")]
  pub type_param: TypeId,
}

impl TypeDefSequence {
  pub fn new(type_param: TypeId) -> Self {
    Self { type_param }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct TypeDefArray {
  pub len: u32,
  #[serde(rename = "type")]
  pub type_param: TypeId,
}

impl TypeDefArray {
  pub fn new(len: u32, type_param: TypeId) -> Self {
    Self { len, type_param }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct TypeDefCompact {
  #[serde(rename = "type")]
  pub type_param: TypeId,
}

impl TypeDefCompact {
  pub fn new(type_param: TypeId) -> Self {
    Self { type_param }
  }
}

pub type TypeId = u32;

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
#[serde(rename_all = "lowercase")]
pub enum TypeDef {
  #[codec(index = 0)]
  Composite(TypeDefComposite),
  #[codec(index = 1)]
  Variant(TypeDefVariant),
  #[codec(index = 2)]
  Sequence(TypeDefSequence),
  #[codec(index = 3)]
  Array(TypeDefArray),
  #[codec(index = 4)]
  Tuple(TypeDefTuple),
  #[codec(index = 5)]
  Primitive(TypeDefPrimitive),
  #[codec(index = 6)]
  Compact(TypeDefCompact),
  // TODO: BitSequence
}

impl TypeDef {
  pub fn to_string(&mut self) -> String {
    format!("TypeDef: {:?}", self)
  }

  pub fn new_type(ty: TypeId) -> Self {
    Self::Tuple(TypeDefTuple::new_type(ty))
  }
}

impl From<TypeDefComposite> for TypeDef {
  fn from(def: TypeDefComposite) -> Self {
    Self::Composite(def)
  }
}

impl From<TypeDefVariant> for TypeDef {
  fn from(def: TypeDefVariant) -> Self {
    Self::Variant(def)
  }
}

impl From<TypeDefSequence> for TypeDef {
  fn from(def: TypeDefSequence) -> Self {
    Self::Sequence(def)
  }
}

impl From<TypeDefArray> for TypeDef {
  fn from(def: TypeDefArray) -> Self {
    Self::Array(def)
  }
}

impl From<TypeDefTuple> for TypeDef {
  fn from(def: TypeDefTuple) -> Self {
    Self::Tuple(def)
  }
}

impl From<TypeDefPrimitive> for TypeDef {
  fn from(def: TypeDefPrimitive) -> Self {
    Self::Primitive(def)
  }
}
