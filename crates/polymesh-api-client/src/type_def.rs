use std::fmt;

use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};

pub use scale_info::{form::PortableForm, TypeDefPrimitive};

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

impl fmt::Display for Path {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.segments.join("::"))
  }
}

impl Path {
  pub fn new(ident: &str, module_path: &str) -> Self {
    let mut segments = module_path
      .split("::")
      .filter(|s| !s.is_empty())
      .map(|s| s.into())
      .collect::<Vec<_>>();
    if ident != "" {
      segments.push(ident.into());
    }
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
pub struct TypeParameter {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: Option<TypeId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct Type {
  #[serde(skip_serializing_if = "Path::is_empty", default)]
  pub path: Path,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub type_params: Vec<TypeParameter>,
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

  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn type_params(&self) -> &[TypeParameter] {
    self.type_params.as_slice()
  }

  pub fn type_def(&self) -> &TypeDef {
    &self.type_def
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
  pub fn new() -> Self {
    Self::default()
  }

  pub fn new_variants(variants: Vec<Variant>) -> Self {
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

  pub fn insert(&mut self, index: u8, name: &str, field: Option<TypeId>) {
    self.variants.push(Variant {
      name: name.into(),
      index,
      fields: field.into_iter().map(|id| Field::new(id)).collect(),
      docs: vec![],
    })
  }

  pub fn get_by_idx(&self, index: u8) -> Option<&Variant> {
    // Try quick search.
    let variant = self
      .variants
      .get(index as usize)
      .filter(|v| v.index == index);
    if variant.is_some() {
      return variant;
    }
    // fallback to linear search.
    for variant in &self.variants {
      if variant.index == index {
        return Some(variant);
      }
    }
    // Not found.
    None
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

  pub fn fields(&self) -> &[TypeId] {
    self.fields.as_slice()
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

  pub fn type_param(&self) -> TypeId {
    self.type_param
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

  pub fn type_param(&self) -> TypeId {
    self.type_param
  }

  pub fn len(&self) -> u32 {
    self.len
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

  pub fn type_param(&self) -> TypeId {
    self.type_param
  }
}

#[derive(
  Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Decode, Encode,
)]
pub struct TypeId(#[codec(compact)] pub u32);

impl TypeId {
  pub fn id(&self) -> u32 {
    self.0
  }

  pub fn inc(&mut self) {
    self.0 += 1;
  }
}

impl From<u32> for TypeId {
  fn from(id: u32) -> Self {
    Self(id)
  }
}

impl From<usize> for TypeId {
  fn from(id: usize) -> Self {
    Self(id as u32)
  }
}

impl From<TypeId> for usize {
  fn from(id: TypeId) -> Self {
    id.0 as Self
  }
}

impl std::ops::Deref for TypeId {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

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

  pub fn new_tuple(fields: Vec<TypeId>) -> Self {
    Self::Tuple(TypeDefTuple::new(fields))
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

impl From<TypeDefCompact> for TypeDef {
  fn from(def: TypeDefCompact) -> Self {
    Self::Compact(def)
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct PortableType {
  pub id: TypeId,
  #[serde(rename = "type")]
  pub ty: Type,
}

impl PortableType {
  pub fn id(&self) -> TypeId {
    self.id
  }

  pub fn ty(&self) -> &Type {
    &self.ty
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct PortableRegistry {
  pub types: Vec<PortableType>,
}

impl PortableRegistry {
  pub fn resolve<T: Into<TypeId>>(&self, id: T) -> Option<&Type> {
    let id = id.into();
    self.types.get(id.0 as usize).map(|t| t.ty())
  }

  pub fn types(&self) -> &[PortableType] {
    self.types.as_slice()
  }
}

impl From<&scale_info::PortableRegistry> for PortableRegistry {
  fn from(other: &scale_info::PortableRegistry) -> Self {
    Self {
      types: other
        .types()
        .iter()
        .map(|t| PortableType {
          id: t.id().into(),
          ty: t.ty().into(),
        })
        .collect(),
    }
  }
}

impl From<&scale_info::TypeParameter<PortableForm>> for TypeParameter {
  fn from(other: &scale_info::TypeParameter<PortableForm>) -> Self {
    Self {
      name: other.name().clone(),
      ty: other.ty().map(|t| t.id().into()),
    }
  }
}

impl From<&scale_info::Type<PortableForm>> for Type {
  fn from(other: &scale_info::Type<PortableForm>) -> Self {
    Self {
      path: other.path().into(),
      type_params: other.type_params().iter().map(|p| p.into()).collect(),
      type_def: other.type_def().into(),
      docs: other.docs().into(),
    }
  }
}

impl From<&scale_info::Path<PortableForm>> for Path {
  fn from(other: &scale_info::Path<PortableForm>) -> Self {
    Self {
      segments: other.segments().iter().cloned().collect(),
    }
  }
}

impl From<&scale_info::Field<PortableForm>> for Field {
  fn from(other: &scale_info::Field<PortableForm>) -> Self {
    Self {
      name: other.name().cloned().into(),
      ty: other.ty().id().into(),
      type_name: other.type_name().cloned().into(),
      docs: other.docs().into(),
    }
  }
}

impl From<&scale_info::Variant<PortableForm>> for Variant {
  fn from(other: &scale_info::Variant<PortableForm>) -> Self {
    Self {
      name: other.name().into(),
      fields: other.fields().iter().map(|f| f.into()).collect(),
      index: other.index().into(),
      docs: other.docs().into(),
    }
  }
}

impl From<&scale_info::TypeDefComposite<PortableForm>> for TypeDefComposite {
  fn from(other: &scale_info::TypeDefComposite<PortableForm>) -> Self {
    Self {
      fields: other.fields().iter().map(|v| v.into()).collect(),
    }
  }
}

impl From<&scale_info::TypeDefVariant<PortableForm>> for TypeDefVariant {
  fn from(other: &scale_info::TypeDefVariant<PortableForm>) -> Self {
    Self {
      variants: other.variants().iter().map(|v| v.into()).collect(),
    }
  }
}

impl From<&scale_info::TypeDefSequence<PortableForm>> for TypeDefSequence {
  fn from(other: &scale_info::TypeDefSequence<PortableForm>) -> Self {
    Self {
      type_param: other.type_param().id().into(),
    }
  }
}

impl From<&scale_info::TypeDefArray<PortableForm>> for TypeDefArray {
  fn from(other: &scale_info::TypeDefArray<PortableForm>) -> Self {
    Self {
      len: other.len(),
      type_param: other.type_param().id().into(),
    }
  }
}

impl From<&scale_info::TypeDefTuple<PortableForm>> for TypeDefTuple {
  fn from(other: &scale_info::TypeDefTuple<PortableForm>) -> Self {
    Self {
      fields: other.fields().iter().map(|v| v.id().into()).collect(),
    }
  }
}

impl From<&scale_info::TypeDefCompact<PortableForm>> for TypeDefCompact {
  fn from(other: &scale_info::TypeDefCompact<PortableForm>) -> Self {
    Self {
      type_param: other.type_param().id().into(),
    }
  }
}

impl From<&scale_info::TypeDef<PortableForm>> for TypeDef {
  fn from(other: &scale_info::TypeDef<PortableForm>) -> Self {
    match other {
      scale_info::TypeDef::Composite(c) => TypeDef::Composite(c.into()),
      scale_info::TypeDef::Variant(v) => TypeDef::Variant(v.into()),
      scale_info::TypeDef::Sequence(s) => TypeDef::Sequence(s.into()),
      scale_info::TypeDef::Array(a) => TypeDef::Array(a.into()),
      scale_info::TypeDef::Tuple(t) => TypeDef::Tuple(t.into()),
      scale_info::TypeDef::Primitive(p) => TypeDef::Primitive(p.clone()),
      scale_info::TypeDef::Compact(ty) => TypeDef::Compact(ty.into()),
      _ => {
        todo!();
      }
    }
  }
}
