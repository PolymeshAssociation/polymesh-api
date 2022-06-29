use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

use serde_json::{Map, Value};

use crate::error::*;
use crate::type_def::*;

macro_rules! parse_error {
  ($fmt:expr, $($arg:tt)*) => {
    Error::SchemaParseFailed(format!($fmt, $($arg)*))
  };
}

#[derive(Clone, Debug)]
pub enum TypeMetaDef {
  Unresolved(String),
  Resolved(Type),
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
    meta.fmt(f)
  }
}

#[derive(Clone)]
pub struct Types {
  next_id: u32,
  types: HashMap<String, TypeRef>,
}

impl From<TypeDefCompact> for TypeDef {
  fn from(def: TypeDefCompact) -> Self {
    Self::Compact(def)
  }
}

impl Types {
  pub fn new() -> Self {
    Self {
      next_id: 0,
      types: HashMap::new(),
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
          log::trace!("Named type: name={name}, val={val}");
          self.parse_named_type(name, val)?;
        }
        Value::Object(map) => {
          if let Some(variants) = map.get("_enum") {
            log::trace!("ENUM: name={name}, variants={variants}");
            self.parse_enum(name, variants)?;
          } else {
            log::trace!("STRUCT: name={name}, fields={map:?}");
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

  fn parse_variant(&mut self, def: &str) -> Result<Vec<Field>> {
    match self.parse(def)? {
      Some(TypeDef::Tuple(tuple)) => Ok(tuple.fields.into_iter().map(|t| Field::new(t)).collect()),
      Some(_) => {
        let type_id = self.parse_type(def)?;
        Ok(vec![Field::new(type_id)])
      }
      None => {
        let type_ref = self.resolve(def);
        Ok(vec![Field::new(type_ref.id)])
      }
    }
  }

  fn parse_enum(&mut self, name: &str, variants: &Value) -> Result<()> {
    let mut index = 0;
    let variants = match variants {
      Value::Array(arr) => arr
        .iter()
        .map(|val| match val.as_str() {
          Some(var_name) => {
            let idx = index;
            index += 1;
            Ok(Variant::new(var_name, vec![], idx))
          }
          None => Err(parse_error!(
            "Expected json string for enum {}: got {:?}",
            name,
            val
          )),
        })
        .collect::<Result<Vec<Variant>>>()?,
      Value::Object(obj) => obj
        .iter()
        .map(|(var_name, val)| -> Result<_> {
          let idx = index;
          index += 1;
          match val.as_str() {
            Some("") => Ok(Variant::new(var_name, vec![], idx)),
            Some(var_def) => {
              let fields = self.parse_variant(var_def)?;
              Ok(Variant::new(var_name, fields, idx))
            }
            None => {
              Err(parse_error!("Expected json string for enum {}: got {:?}", name, val).into())
            }
          }
        })
        .collect::<Result<Vec<Variant>>>()?,
      _ => {
        return Err(parse_error!("Invalid json for `_enum`: {:?}", variants));
      }
    };
    self.insert_type(name, TypeDefVariant::new(variants).into());
    Ok(())
  }

  fn parse_struct(&mut self, name: &str, def: &Map<String, Value>) -> Result<()> {
    let fields = def
      .iter()
      .map(|(field_name, val)| -> Result<_> {
        match val.as_str() {
          Some(field_def) => {
            let type_id = self.parse_type(field_def)?;
            Ok(Field::new_named(
              field_name,
              type_id,
              Some(field_def.to_string()),
            ))
          }
          None => Err(parse_error!(
            "Expected json string for struct {} field {}: got {:?}",
            name,
            field_name,
            val
          )),
        }
      })
      .collect::<Result<Vec<Field>>>()?;
    self.insert_type(name, TypeDefComposite::new(fields).into());
    Ok(())
  }

  pub fn parse_named_type(&mut self, name: &str, def: &str) -> Result<TypeId> {
    let type_id = self.parse_type(def)?;

    Ok(self.insert_type(name, TypeDef::new_type(type_id)))
  }

  pub fn parse_type(&mut self, name: &str) -> Result<TypeId> {
    let name = name
      .trim()
      .replace("\r", "")
      .replace("\n", "")
      .replace("T::", "");
    // Try to resolve the type.
    let type_ref = self.resolve(&name);
    let mut type_meta = type_ref.def.write().unwrap();

    // Check if type is unresolved.
    match &*type_meta {
      TypeMetaDef::Unresolved(def) => {
        // Try parsing it.
        log::trace!("Parse Unresolved: name={name}, def={def}");
        if let Some(type_def) = self.parse(def)? {
          let ty = Type::new(&name, type_def);
          let new_meta = TypeMetaDef::Resolved(ty);
          *type_meta = new_meta;
        }
      }
      _ => (),
    }
    Ok(type_ref.id)
  }

  fn is_primitive(def: &str) -> Option<TypeDefPrimitive> {
    // Check for primitives.
    match def {
      "u8" => Some(TypeDefPrimitive::U8),
      "u16" => Some(TypeDefPrimitive::U16),
      "u32" => Some(TypeDefPrimitive::U32),
      "u64" => Some(TypeDefPrimitive::U64),
      "u128" => Some(TypeDefPrimitive::U128),
      "u256" => Some(TypeDefPrimitive::U256),
      "i8" => Some(TypeDefPrimitive::I8),
      "i16" => Some(TypeDefPrimitive::I16),
      "i32" => Some(TypeDefPrimitive::I32),
      "i64" => Some(TypeDefPrimitive::I64),
      "i128" => Some(TypeDefPrimitive::I128),
      "i256" => Some(TypeDefPrimitive::I256),
      "bool" => Some(TypeDefPrimitive::Bool),
      "char" => Some(TypeDefPrimitive::Char),
      "String" => Some(TypeDefPrimitive::Str),
      "Text" => Some(TypeDefPrimitive::Str),
      _ => None,
    }
  }

  fn parse(&mut self, def: &str) -> Result<Option<TypeDef>> {
    match def.chars().last() {
      Some('>') => {
        // Handle: Vec<T>, Option<T>, Compact<T>
        let (wrap, ty) = def
          .strip_suffix('>')
          .and_then(|s| s.split_once('<'))
          .map(|(wrap, ty)| (wrap.trim(), ty.trim()))
          .ok_or_else(|| parse_error!("Failed to parse Vec/Option/Compact: {}", def))?;
        match wrap {
          "Vec" => {
            let wrap_ref = self.parse_type(ty)?;
            Ok(Some(TypeDefSequence::new(wrap_ref).into()))
          }
          "Option" => {
            let wrap_ref = self.parse_type(ty)?;
            Ok(Some(TypeDefVariant::new_option(wrap_ref).into()))
          }
          "Compact" => {
            let wrap_ref = self.parse_type(ty)?;
            Ok(Some(TypeDefCompact::new(wrap_ref).into()))
          }
          "Box" => {
            let wrap_ref = self.parse_type(ty)?;
            Ok(Some(TypeDefTuple::new_type(wrap_ref).into()))
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
            Ok(Some(TypeDefVariant::new_result(ok_ref, err_ref).into()))
          }
          "PhantomData" | "sp_std::marker::PhantomData" => Ok(Some(TypeDefTuple::unit().into())),
          _ => {
            // Unresolved type.
            Ok(None)
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
            let type_id = self.parse_type(&val)?;
            vec.push(type_id);
            Ok(vec)
          })?;
        // Handle tuples.
        Ok(Some(TypeDefTuple::new(defs).into()))
      }
      Some(']') => {
        let (slice_ty, slice_len) = def
          .trim_matches(|c| c == '[' || c == ']')
          .split_once(';')
          .and_then(|(ty, len)| {
            // parse slice length.
            len.trim().parse::<usize>().ok().map(|l| (ty.trim(), l))
          })
          .ok_or_else(|| parse_error!("Failed to parse slice: {}", def))?;
        // Handle slices.
        let slice_ref = self.parse_type(slice_ty)?;
        Ok(Some(TypeDefArray::new(slice_len as u32, slice_ref).into()))
      }
      _ => Ok(None),
    }
  }

  fn new_type(&mut self, meta: TypeMetaDef) -> TypeRef {
    let id = self.next_id;
    self.next_id += 1;
    TypeRef::new(id, meta)
  }

  pub fn resolve(&mut self, name: &str) -> TypeRef {
    if let Some(type_ref) = self.types.get(name) {
      type_ref.clone()
    } else if let Some(prim) = Self::is_primitive(name) {
      let type_ref = self.new_type(TypeMetaDef::Resolved(Type::new("", prim.into())));
      self.types.insert(name.into(), type_ref.clone());
      type_ref
    } else {
      let type_ref = self.new_type(TypeMetaDef::Unresolved(name.into()));
      self.types.insert(name.into(), type_ref.clone());
      type_ref
    }
  }

  pub fn insert_type(&mut self, name: &str, type_def: TypeDef) -> TypeId {
    let ty = Type::new(name, type_def);
    let type_ref = self.new_type(TypeMetaDef::Resolved(ty));
    self.insert(name, type_ref)
  }

  pub fn insert(&mut self, name: &str, type_ref: TypeRef) -> TypeId {
    use std::collections::hash_map::Entry;
    let entry = self.types.entry(name.into());
    match entry {
      Entry::Occupied(entry) => {
        let old_ref = entry.get();
        let mut old_meta = old_ref.def.write().unwrap();
        // Already exists.  Check that it is a `TypeDef::Unresolved`.
        match &*old_meta {
          TypeMetaDef::Unresolved(_) => {
            let ty = Type::new(name, TypeDef::new_type(type_ref.id));
            *old_meta = TypeMetaDef::Resolved(ty);
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

  pub fn resolve(&self, name: &str) -> TypeRef {
    let mut t = self.types.write().unwrap();
    t.resolve(name)
  }

  pub fn insert_type(&self, name: &str, type_meta: TypeDef) -> TypeId {
    let mut t = self.types.write().unwrap();
    t.insert_type(name, type_meta)
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
