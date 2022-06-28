use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use codec::{Decode, Encode};

use indexmap::map::IndexMap;

use scale_info::TypeDefPrimitive;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Decode, Encode)]
pub struct Path {
  pub segments: Vec<String>,
}

impl Path {
  pub fn is_empty(&self) -> bool {
    self.segments.is_empty()
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct TypeParameter {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: Option<TypeId>,
}

impl TypeParameter {
  pub fn new(name: String, ty: Option<TypeId>) -> Self {
    Self { name, ty }
  }
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

impl Type {}

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
  pub fn new(name: Option<String>, ty: TypeId, type_name: Option<String>) -> Self {
    Self {
      name,
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
  pub fn new(name: String, fields: Vec<Field>, index: u8) -> Self {
    Self {
      name,
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

#[derive(Clone, Debug)]
pub enum TypeMetaDef {
  Unresolved(String),
  NewType(String, TypeId),
  Resolved(TypeDef),
}

#[derive(Clone)]
pub struct TypeRef {
  id: TypeId,
  def: Arc<RwLock<TypeMetaDef>>,
}

impl TypeRef {
  pub fn new(id: TypeId, meta: TypeMetaDef) -> Self {
    Self {
      def: Arc::new(RwLock::new(meta)),
      id,
    }
  }

  pub fn to_string(&mut self) -> String {
    format!("TypeRef[{}]: {:?}", self.id, self.def.read().unwrap())
  }
}

impl PartialEq for TypeRef {
  fn eq(&self, other: &Self) -> bool {
    self.id.eq(&other.id)
  }
}

impl Eq for TypeRef {}

impl PartialOrd for TypeRef {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.id.cmp(&other.id))
  }
}

impl Ord for TypeRef {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.id.cmp(&other.id)
  }
}

impl std::fmt::Debug for TypeRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    let meta = self.def.read().unwrap();
    match &*meta {
      TypeMetaDef::NewType(name, _) => f.write_fmt(format_args!("NewType({})", name)),
      _ => meta.fmt(f),
    }
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
}

#[derive(Clone)]
pub struct Types {
  next_id: u32,
  types: IndexMap<String, TypeRef>,
}

impl Types {
  pub fn new() -> Self {
    Self {
      next_id: 0,
      types: IndexMap::new(),
    }
  }

  pub fn load_schema(&mut self, filename: &str) -> Result<()> {
    log::info!("load_schema: {}", filename);
    let file = File::open(filename)?;

    let schema: serde_json::Value = serde_json::from_reader(BufReader::new(file))?;

    let schema = schema
      .as_object()
      .expect("Invalid schema, expected object.");

    let types = match schema.get("types") {
      Some(val) => val.as_object().unwrap_or(schema),
      _ => schema,
    };
    self.parse_schema_types(types)?;

    Ok(())
  }

  fn parse_schema_types(&mut self, types: &Map<String, Value>) -> Result<()> {
    for (name, val) in types.iter() {
      match val {
        Value::String(val) => {
          self.parse_named_type(name, val)?;
        }
        Value::Object(map) => {
          if let Some(variants) = map.get("_enum") {
            self.parse_enum(name, variants)?;
          } else {
            self.parse_struct(name, map)?;
          }
        }
        _ => {
          eprintln!("UNHANDLED JSON VALUE: {} => {:?}", name, val);
        }
      }
    }
    Ok(())
  }

  fn parse_enum(&mut self, _name: &str, _variants: &Value) -> Result<()> {
    /*
    match variants {
      Value::Array(arr) => {
        let variants = arr
          .iter()
          .try_fold(TypeDefVariant::new(), |mut variants, val| {
            match val.as_str() {
              Some(name) => {
                variants.insert(name, None);
                Ok(variants)
              }
              None => Err(anyhow!(
                "Expected json string for enum {}: got {:?}",
                name,
                val
              )),
            }
          })?;
        self.insert_meta(name, TypeDef::Enum(variants));
      }
      Value::Object(obj) => {
        let variants = obj.iter().try_fold(
          TypeDefVariant::new(),
          |mut variants, (var_name, val)| -> Result<_> {
            match val.as_str() {
              Some("") => {
                variants.insert(var_name, None);
                Ok(variants)
              }
              Some(var_def) => {
                let type_meta = self.parse_type(var_def)?;
                variants.insert(var_name, Some(type_meta));
                Ok(variants)
              }
              None => Err(anyhow!("Expected json string for enum {}: got {:?}", name, val).into()),
            }
          },
        )?;
        self.insert_meta(name, TypeDef::Enum(variants));
      }
      _ => {
        return Err(anyhow!("Invalid json for `_enum`: {:?}", variants));
      }
    }
      */
    Ok(())
  }

  fn parse_struct(&mut self, _name: &str, _def: &Map<String, Value>) -> Result<()> {
    /*
    let fields =
      def
        .iter()
        .try_fold(IndexMap::new(), |mut map, (field_name, val)| -> Result<_> {
          match val.as_str() {
            Some(field_def) => {
              let type_meta = self.parse_type(field_def)?;
              map.insert(field_name.to_string(), type_meta);
              Ok(map)
            }
            None => Err(anyhow!(
              "Expected json string for struct {} field {}: got {:?}",
              name,
              field_name,
              val
            )),
          }
        })?;
    self.insert_meta(name, TypeDef::Struct(fields));
      */
    Ok(())
  }

  pub fn parse_named_type(&mut self, name: &str, def: &str) -> Result<TypeId> {
    let type_ref = self.parse_type(def)?;

    let type_ref = self.new_type(TypeMetaDef::NewType(name.into(), type_ref));
    Ok(self.insert(name, type_ref))
  }

  pub fn parse_type(&mut self, name: &str) -> Result<TypeId> {
    /*
    let name = name
      .trim()
      .replace("\r", "")
      .replace("\n", "")
      .replace("T::", "");
    // Try to resolve the type.
    let type_ref = self.resolve(&name);
    let mut type_meta = type_ref.0.write().unwrap();

    // Check if type is unresolved.
    match &*type_meta {
      TypeDef::Unresolved(def) => {
        // Try parsing it.
        let new_meta = self.parse(def)?;
        *type_meta = new_meta;
      }
      _ => (),
    }
    Ok(type_ref.clone())
      */
    Ok(self.resolve(name))
  }

  /*
  fn parse(&mut self, _def: &str) -> Result<TypeDef> {
    match def.chars().last() {
      Some('>') => {
        // Handle: Vec<T>, Option<T>, Compact<T>
        let (wrap, ty) = def
          .strip_suffix('>')
          .and_then(|s| s.split_once('<'))
          .map(|(wrap, ty)| (wrap.trim(), ty.trim()))
          .ok_or_else(|| anyhow!("Failed to parse Vec/Option/Compact: {}", def))?;
        match wrap {
          "Vec" => {
            let wrap_ref = self.parse_type(ty)?;
            Ok(TypeDef::Vector(wrap_ref))
          }
          "Option" => {
            let wrap_ref = self.parse_type(ty)?;
            Ok(TypeDef::Option(wrap_ref))
          }
          "Compact" => {
            let wrap_ref = self.parse_type(ty)?;
            Ok(TypeDef::Compact(wrap_ref))
          }
          "Box" => {
            let wrap_ref = self.parse_type(ty)?;
            Ok(TypeDef::Box(wrap_ref))
          }
          "Result" => {
            let (ok_ref, err_ref) = match ty.split_once(',') {
              Some((ok_ty, err_ty)) => {
                let ok_ref = self.parse_type(ok_ty)?;
                let err_ref = self.parse_type(err_ty)?;
                (ok_ref, err_ref)
              }
              None => {
                let ok_ref = self.parse_type(ty)?;
                let err_ref = self.parse_type("Error")?;
                (ok_ref, err_ref)
              }
            };
            Ok(TypeDef::Result(ok_ref, err_ref))
          }
          "PhantomData" | "sp_std::marker::PhantomData" => Ok(TypeDef::Unit),
          generic => {
            // Some generic type.
            if self.types.contains_key(generic) {
              Ok(TypeDef::NewType(generic.into(), self.resolve(generic)))
            } else {
              Ok(TypeDef::Unresolved(def.into()))
            }
          }
        }
      }
      Some(')') => {
        let mut broken_type = None;
        let defs = def
          .trim_matches(|c| c == '(' || c == ')')
          .split_terminator(',')
          .filter_map(|s| {
            let s = match broken_type.take() {
              Some(s1) => format!("{}, {}", s1, s),
              None => s.to_string(),
            }
            .trim()
            .to_string();
            // Check for broken type.
            let left = s.chars().filter(|c| *c == '<').count();
            let right = s.chars().filter(|c| *c == '>').count();
            if left != right {
              broken_type = Some(s);
              return None;
            }
            if s != "" {
              Some(s)
            } else {
              None
            }
          })
          .try_fold(Vec::new(), |mut vec, val| -> Result<_> {
            let type_ref = self.parse_type(&val)?;
            vec.push(type_ref);
            Ok(vec)
          })?;
        // Handle tuples.
        Ok(TypeDef::Tuple(defs))
      }
      Some(']') => {
        let (slice_ty, slice_len) = def
          .trim_matches(|c| c == '[' || c == ']')
          .split_once(';')
          .and_then(|(ty, len)| {
            // parse slice length.
            len.trim().parse::<usize>().ok().map(|l| (ty.trim(), l))
          })
          .ok_or_else(|| anyhow!("Failed to parse slice: {}", def))?;
        // Handle slices.
        let slice_ref = self.parse_type(slice_ty)?;
        Ok(TypeDef::Slice(slice_len, slice_ref))
      }
      _ => Ok(TypeDef::Unresolved(def.into())),
    }
  }
    */

  fn new_type(&mut self, meta: TypeMetaDef) -> TypeRef {
    let id = self.next_id;
    self.next_id += 1;
    TypeRef::new(id, meta)
  }

  pub fn resolve(&mut self, name: &str) -> TypeId {
    if let Some(type_ref) = self.types.get(name) {
      type_ref.id
    } else {
      let type_ref = self.new_type(TypeMetaDef::Unresolved(name.into()));
      let id = type_ref.id;
      self.types.insert(name.into(), type_ref);
      id
    }
  }

  pub fn insert_meta(&mut self, name: &str, type_def: TypeDef) -> TypeId {
    let type_ref = self.new_type(TypeMetaDef::Resolved(type_def));
    self.insert(name, type_ref)
  }

  pub fn insert(&mut self, name: &str, type_ref: TypeRef) -> TypeId {
    use indexmap::map::Entry;
    let entry = self.types.entry(name.into());
    match entry {
      Entry::Occupied(entry) => {
        let old_ref = entry.get();
        let mut old_meta = old_ref.def.write().unwrap();
        // Already exists.  Check that it is a `TypeDef::Unresolved`.
        match &*old_meta {
          TypeMetaDef::Unresolved(_) => {
            *old_meta = TypeMetaDef::NewType(name.into(), type_ref.id);
          }
          _ => {
            eprintln!("REDEFINE TYPE: {}", name);
          }
        }
        old_ref.id
      }
      Entry::Vacant(entry) => {
        let id = type_ref.id;
        entry.insert(type_ref);
        id
      }
    }
  }

  /// Dump types.
  pub fn dump_types(&self) {
    for (idx, (key, type_ref)) in self.types.iter().enumerate() {
      eprintln!("Type[{}]: {} => {:#?}", idx, key, type_ref);
    }
  }

  /// Dump unresolved types.
  pub fn dump_unresolved(&self) {
    for (key, type_ref) in self.types.iter() {
      let meta = type_ref.def.read().unwrap();
      match &*meta {
        TypeMetaDef::Unresolved(def) => {
          eprintln!("--------- Unresolved: {} => {}", key, def);
        }
        _ => (),
      }
    }
  }
}

#[derive(Clone)]
pub struct TypeLookup {
  types: Arc<RwLock<Types>>,
}

impl TypeLookup {
  pub fn from_types(types: Types) -> Self {
    Self {
      types: Arc::new(RwLock::new(types)),
    }
  }

  pub fn parse_named_type(&self, name: &str, def: &str) -> Result<TypeId> {
    let mut t = self.types.write().unwrap();
    t.parse_named_type(name, def)
  }

  pub fn parse_type(&self, def: &str) -> Result<TypeId> {
    let mut t = self.types.write().unwrap();
    t.parse_type(def)
  }

  pub fn resolve(&self, name: &str) -> TypeId {
    let mut t = self.types.write().unwrap();
    t.resolve(name)
  }

  pub fn insert_meta(&self, name: &str, type_meta: TypeDef) -> TypeId {
    let mut t = self.types.write().unwrap();
    t.insert_meta(name, type_meta)
  }

  pub fn insert(&self, name: &str, type_def: TypeRef) -> TypeId {
    let mut t = self.types.write().unwrap();
    t.insert(name, type_def)
  }

  pub fn dump_types(&mut self) {
    self.types.read().unwrap().dump_types();
  }

  pub fn dump_unresolved(&mut self) {
    self.types.read().unwrap().dump_unresolved();
  }
}
