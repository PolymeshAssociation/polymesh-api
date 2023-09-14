#[cfg(not(feature = "std"))]
use alloc::{
  collections::btree_map::BTreeMap,
  sync::{Arc, RwLock},
};
#[cfg(feature = "std")]
use std::{
  collections::BTreeMap,
  fs::File,
  io::BufReader,
  sync::{Arc, RwLock},
};

#[cfg(not(feature = "std"))]
use alloc::{
  format,
  string::{String, ToString},
};
use sp_std::prelude::*;

use frame_metadata::RuntimeMetadata;

use serde_json::{Map, Value};

use crate::error::*;
use crate::metadata::*;
use crate::type_def::*;
use crate::*;

/// Use large type ids for old schema and metadata (v12-v13) types.
/// The newer v14 metadata uses small ids.
pub const SCHEMA_TYPE_ID_BASE: u32 = 10_000_000;

macro_rules! parse_error {
  ($fmt:expr, $($arg:tt)*) => {
    Error::SchemaParseFailed(format!($fmt, $($arg)*))
  };
}

#[cfg(feature = "v14")]
pub fn is_type_compact(ty: &Type) -> bool {
  match ty.type_def() {
    TypeDef::Compact(_) => true,
    _ => false,
  }
}

#[cfg(feature = "v14")]
pub fn get_type_name(ty: &Type, types: &PortableRegistry, full: bool) -> String {
  let name = match ty.type_def() {
    TypeDef::Sequence(s) => {
      let elm_ty = types
        .resolve(s.type_param())
        .expect("Failed to resolve sequence element type");
      format!("Vec<{}>", get_type_name(elm_ty, types, full))
    }
    TypeDef::Array(a) => {
      let elm_ty = types
        .resolve(a.type_param())
        .expect("Failed to resolve array element type");
      format!("[{}; {}]", get_type_name(elm_ty, types, full), a.len())
    }
    TypeDef::Tuple(t) => {
      let fields = t
        .fields()
        .iter()
        .map(|f| {
          let f_ty = types
            .resolve(*f)
            .expect("Failed to resolve tuple element type");
          get_type_name(f_ty, types, full)
        })
        .collect::<Vec<_>>();
      format!("({})", fields.join(","))
    }
    TypeDef::Primitive(p) => {
      use TypeDefPrimitive::*;
      match p {
        Bool => "bool".into(),
        Char => "char".into(),
        Str => "Text".into(),
        U8 => "u8".into(),
        U16 => "u16".into(),
        U32 => "u32".into(),
        U64 => "u64".into(),
        U128 => "u128".into(),
        U256 => "u256".into(),
        I8 => "i8".into(),
        I16 => "i16".into(),
        I32 => "i32".into(),
        I64 => "i64".into(),
        I128 => "i128".into(),
        I256 => "i256".into(),
      }
    }
    TypeDef::Compact(c) => {
      let elm_ty = types
        .resolve(c.type_param())
        .expect("Failed to resolve Compact type");
      format!("Compact<{}>", get_type_name(elm_ty, types, full))
    }
    _ => {
      if full {
        format!("{}", ty.path())
      } else {
        ty.path().ident().expect("Missing type name").into()
      }
    }
  };
  let ty_params = ty.type_params();
  if ty_params.len() > 0 {
    let params = ty_params
      .iter()
      .map(|p| match &p.ty {
        Some(ty) => {
          let p_ty = types
            .resolve(*ty)
            .expect("Failed to resolve type parameter");
          get_type_name(p_ty, types, full)
        }
        None => p.name.clone(),
      })
      .collect::<Vec<_>>();
    format!("{}<{}>", name, params.join(","))
  } else {
    name
  }
}

#[derive(Clone, Debug)]
pub struct TypeRef {
  pub id: TypeId,
  pub ty: Option<Type>,
}

impl TypeRef {
  pub fn new(id: TypeId, ty: Option<Type>) -> Self {
    Self { id, ty }
  }

  pub fn to_string(&mut self) -> String {
    format!("TypeRef[{:?}]: {:?}", self.id, self.ty)
  }
}

#[derive(Clone)]
pub struct Types {
  next_id: TypeId,
  types: BTreeMap<TypeId, Option<Type>>,
  name_to_id: BTreeMap<String, TypeId>,
  runtime_version: RuntimeVersion,
  metadata: Option<Metadata>,
}

impl Types {
  pub fn new(runtime_version: RuntimeVersion) -> Self {
    Self {
      next_id: TypeId(SCHEMA_TYPE_ID_BASE),
      types: BTreeMap::new(),
      name_to_id: BTreeMap::new(),
      runtime_version,
      metadata: None,
    }
  }

  pub fn get_runtime_version(&self) -> RuntimeVersion {
    self.runtime_version.clone()
  }

  pub fn set_metadata(&mut self, metadata: Metadata) {
    self.metadata = Some(metadata);
  }

  pub fn get_metadata(&self) -> Option<Metadata> {
    self.metadata.as_ref().cloned()
  }

  #[cfg(feature = "std")]
  fn load_schema(&mut self, filename: &str) -> Result<()> {
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

  #[cfg(feature = "std")]
  pub fn try_load_schema(&mut self, filename: &str) -> bool {
    log::info!("Try loading schema: {}", filename);
    match self.load_schema(filename) {
      Ok(_) => true,
      Err(err) => {
        log::debug!("Failed to load schema {}: {err:?}", filename);
        false
      }
    }
  }

  #[cfg(not(feature = "std"))]
  pub fn try_load_schema(&mut self, _filename: &str) -> bool {
    false
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
          log::warn!("UNHANDLED JSON VALUE: {} => {:?}", name, val);
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
    self.insert_type(name, TypeDefVariant::new_variants(variants).into());
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
    let id = self.parse_type(def)?;
    Ok(self.insert_new_type(name, id))
  }

  pub fn parse_type(&mut self, def: &str) -> Result<TypeId> {
    let name = def
      .trim()
      .replace("\r", "")
      .replace("\n", "")
      .replace("T::", "");
    log::trace!("-- parse_type: {def} -> {name}");
    // Try to resolve the type.
    let type_ref = self.resolve(&name);
    // Check if type is unresolved.
    match type_ref.ty {
      None => {
        // Try parsing it.
        log::trace!("Parse Unresolved: name={name}, def={def}");
        if let Some(type_def) = self.parse(def)? {
          // Insert TypeDef for unresolved type.
          self.insert_type(&name, type_def);
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
    log::trace!("-- parse: {def}, last ch={:?}", def.chars().last());
    match def.chars().last() {
      Some('>') => {
        // Handle: Vec<T>, Option<T>, Compact<T>
        let (ty, param) = def
          .strip_suffix('>')
          .and_then(|s| s.split_once('<'))
          .map(|(ty, param)| (ty.trim(), param.trim()))
          .ok_or_else(|| parse_error!("Failed to parse Vec/Option/Compact: {}", def))?;
        log::trace!("-- GENERIC type: {ty}, param: {param}");
        match ty {
          "Vec" => {
            let param_ref = self.parse_type(param)?;
            Ok(Some(TypeDefSequence::new(param_ref).into()))
          }
          "Option" => {
            let param_ref = self.parse_type(param)?;
            Ok(Some(TypeDefVariant::new_option(param_ref).into()))
          }
          "Compact" => {
            let param_ref = self.parse_type(param)?;
            Ok(Some(TypeDefCompact::new(param_ref).into()))
          }
          "Box" => {
            let param_ref = self.parse_type(param)?;
            Ok(Some(TypeDefTuple::new_type(param_ref).into()))
          }
          "Result" => {
            let (ok_ref, err_ref) = match param.split_once(',') {
              Some((ok_ty, err_ty)) => {
                let ok_ref = self.parse_type(ok_ty)?;
                let err_ref = self.parse_type(err_ty)?;
                (ok_ref, err_ref)
              }
              None => {
                let ok_ref = self.parse_type(param)?;
                let err_ref = self.parse_type("Error")?;
                (ok_ref, err_ref)
              }
            };
            Ok(Some(TypeDefVariant::new_result(ok_ref, err_ref).into()))
          }
          "PhantomData" | "sp_std::marker::PhantomData" => Ok(Some(TypeDefTuple::unit().into())),
          ty => Ok(
            self
              .name_to_id
              .get(ty)
              .map(|id| TypeDefTuple::new_type(*id).into()),
          ),
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

  fn new_type(&mut self, ty: Option<Type>) -> TypeId {
    let id = self.next_id;
    self.next_id.inc();
    self.types.insert(id, ty);
    id
  }

  pub fn get_type(&self, id: TypeId) -> Option<&Type> {
    self.types.get(&id).and_then(|t| t.as_ref())
  }

  pub fn resolve(&mut self, name: &str) -> TypeRef {
    let id = if let Some(id) = self.name_to_id.get(name) {
      *id
    } else if let Some(prim) = Self::is_primitive(name) {
      let id = self.new_type(Some(Type::new("", prim.into())));
      self.name_to_id.insert(name.into(), id);
      id
    } else {
      let id = self.new_type(None);
      self.name_to_id.insert(name.into(), id);
      id
    };
    TypeRef::new(id, self.get_type(id).cloned())
  }

  pub fn insert_new_type(&mut self, name: &str, ty_id: TypeId) -> TypeId {
    self.insert_type(name, TypeDef::new_type(ty_id))
  }

  pub fn insert_type(&mut self, name: &str, type_def: TypeDef) -> TypeId {
    let ty = Type::new(name, type_def);
    log::trace!("insert_type: {name} => {ty:?}");
    self.insert(name, ty)
  }

  pub fn import_type(&mut self, name: &str, id: TypeId, ty: Type) -> Result<()> {
    if id.0 >= SCHEMA_TYPE_ID_BASE {
      Err(Error::SchemaParseFailed(format!(
        "Imported type ids must be below schema type base: {:?} >= {}",
        id, SCHEMA_TYPE_ID_BASE
      )))?;
    }
    // insert type.
    if self.types.insert(id, Some(ty)).is_some() {
      Err(Error::SchemaParseFailed(format!(
        "Imported type id {:?} already exists",
        id
      )))?;
    }

    if self.name_to_id.get(name).is_some() {
      self.insert_new_type(name, id);
    } else {
      self.name_to_id.insert(name.into(), id);
    }

    Ok(())
  }

  pub fn insert(&mut self, name: &str, ty: Type) -> TypeId {
    if let Some(id) = self.name_to_id.get(name) {
      if let Some(old_type) = self.types.get_mut(id) {
        // Already exists.  Check if it has a type defined yet.
        if old_type.is_none() {
          *old_type = Some(ty);
        } else {
          log::warn!("REDEFINE TYPE: {}", name);
        }
      } else {
        log::warn!("TYPE_ID MISSING: {} -> {:?}", name, id);
        self.types.insert(*id, Some(ty));
      }
      *id
    } else {
      let id = self.new_type(Some(ty));
      self.name_to_id.insert(name.into(), id);
      id
    }
  }

  /// Dump types.
  pub fn dump_types(&self) {
    for (id, ty) in self.types.iter() {
      log::warn!("Type[{:?}] => {:#?}", id, ty);
    }
  }

  /// Dump unresolved types.
  pub fn dump_unresolved(&self) {
    for (name, id) in self.name_to_id.iter() {
      match self.types.get(id) {
        None => {
          log::warn!("--------- type name maps to invalid type id: {name}");
        }
        Some(None) => {
          log::warn!("--------- Unresolved[{:?}]: {}", id, name);
        }
        Some(Some(_)) => {
          // Defined type.
        }
      }
    }
  }
}

#[cfg(feature = "v14")]
impl Types {
  pub fn import_v14_types(&mut self, types: &PortableRegistry) -> Result<()> {
    for ty in types.types() {
      let name = get_type_name(ty.ty(), &types, true);
      log::debug!("import_v14_type: {:?} => {}", ty.id(), name);
      self.import_type(&name, ty.id(), ty.ty().clone())?;
    }
    Ok(())
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

  pub fn get_type(&self, id: TypeId) -> Option<Type> {
    let t = self.types.read().unwrap();
    t.get_type(id).cloned()
  }

  pub fn resolve(&self, name: &str) -> TypeRef {
    let mut t = self.types.write().unwrap();
    t.resolve(name)
  }

  pub fn insert_type(&self, name: &str, type_meta: TypeDef) -> TypeId {
    let mut t = self.types.write().unwrap();
    t.insert_type(name, type_meta)
  }

  pub fn dump_types(&self) {
    self.types.read().unwrap().dump_types();
  }

  pub fn dump_unresolved(&self) {
    self.types.read().unwrap().dump_unresolved();
  }
}

pub struct InitRegistryFn(Box<dyn Fn(&mut Types) -> Result<()> + Send + Sync + 'static>);

impl InitRegistryFn {
  pub fn init_types(&self, types: &mut Types) -> Result<()> {
    self.0(types)
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SpecVersionKey(String, u32);

impl From<&RuntimeVersion> for SpecVersionKey {
  fn from(version: &RuntimeVersion) -> Self {
    Self(version.spec_name.to_string(), version.spec_version)
  }
}

pub struct InnerTypesRegistry {
  block_types: BTreeMap<Option<SpecVersionKey>, TypeLookup>,
  initializers: Vec<InitRegistryFn>,
}

impl InnerTypesRegistry {
  pub fn new() -> Self {
    Self {
      block_types: BTreeMap::new(),
      initializers: Vec::new(),
    }
  }

  #[cfg(any(feature = "v12", feature = "v32"))]
  fn load_custom_types(&self, prefix: &str, spec: u32, types: &mut Types) -> Result<()> {
    // Load standard substrate types.
    if !types.try_load_schema(&format!("{}/init_{}.json", prefix, spec)) {
      types.try_load_schema("./schemas/init_types.json");
    }
    // Load custom chain types.
    if !types.try_load_schema(&format!("{}/{}.json", prefix, spec)) {
      // fallback.
      types.try_load_schema("schema.json");
    }

    Ok(())
  }

  async fn build_types(
    &self,
    client: &Client,
    version: Option<RuntimeVersion>,
    hash: Option<BlockHash>,
  ) -> Result<TypeLookup> {
    let runtime_version = match version {
      Some(version) => version,
      None => client
        .get_block_runtime_version(hash)
        .await?
        .ok_or_else(|| Error::RpcClient(format!("Failed to get block RuntimeVersion")))?,
    };
    // build schema path.
    let spec_name = runtime_version.spec_name.to_string();
    #[cfg(any(feature = "v12", feature = "v32"))]
    let spec_version = runtime_version.spec_version;
    let name = if let Some((spec_name, _chain_type)) = spec_name.split_once("_") {
      spec_name
    } else {
      &spec_name
    };
    let schema_prefix = format!("./schemas/{}", name);
    log::debug!("schema_prefix = {}", schema_prefix);

    let mut types = Types::new(runtime_version);

    // Load chain metadata.
    let runtime_metadata = client
      .get_block_metadata(hash)
      .await?
      .ok_or_else(|| Error::RpcClient(format!("Failed to get block Metadata")))?;

    // Process chain metadata.
    let metadata = match runtime_metadata.1 {
      #[cfg(feature = "v12")]
      RuntimeMetadata::V12(v12) => {
        if runtime_metadata.0 != frame_metadata::v12::META_RESERVED {
          return Err(Error::MetadataParseFailed(format!(
            "Invalid metadata prefix {}",
            runtime_metadata.0
          )));
        }
        self.load_custom_types(&schema_prefix, spec_version, &mut types)?;

        Metadata::from_v12_metadata(v12, &mut types)?
      }
      #[cfg(feature = "v13")]
      RuntimeMetadata::V13(v13) => {
        if runtime_metadata.0 != frame_metadata::v13::META_RESERVED {
          return Err(Error::MetadataParseFailed(format!(
            "Invalid metadata prefix {}",
            runtime_metadata.0
          )));
        }
        self.load_custom_types(&schema_prefix, spec_version, &mut types)?;

        Metadata::from_v13_metadata(v13, &mut types)?
      }
      #[cfg(feature = "v14")]
      RuntimeMetadata::V14(v14) => {
        if runtime_metadata.0 != frame_metadata::META_RESERVED {
          return Err(Error::MetadataParseFailed(format!(
            "Invalid metadata prefix {}",
            runtime_metadata.0
          )));
        }
        types.try_load_schema("schemas/init_v14_types.json");

        Metadata::from_v14_metadata(v14, &mut types)?
      }
      _ => {
        return Err(Error::MetadataParseFailed(format!(
          "Unsupported metadata version"
        )));
      }
    };
    types.set_metadata(metadata);

    for init in &self.initializers {
      init.init_types(&mut types)?;
    }
    let lookup = TypeLookup::from_types(types);
    Ok(lookup)
  }

  pub async fn get_block_types(
    &mut self,
    client: &Client,
    version: Option<RuntimeVersion>,
    hash: Option<BlockHash>,
  ) -> Result<TypeLookup> {
    let spec_key: Option<SpecVersionKey> = version.as_ref().map(|v| v.into());
    if let Some(types) = self.block_types.get(&spec_key) {
      return Ok(types.clone());
    }

    log::info!(
      "Spec version not found: load schema/metadata.  RuntimeVersion={:?}",
      version
    );
    // Need to build/initialize new Types.
    let lookup = self.build_types(client, version, hash).await?;
    self.block_types.insert(spec_key, lookup.clone());
    Ok(lookup)
  }

  pub fn add_init(&mut self, func: InitRegistryFn) {
    self.initializers.push(func);
  }
}

#[derive(Clone)]
pub struct TypesRegistry(Arc<RwLock<InnerTypesRegistry>>);

impl TypesRegistry {
  pub fn new() -> Self {
    Self(Arc::new(RwLock::new(InnerTypesRegistry::new())))
  }

  pub async fn get_block_types(
    &self,
    client: &Client,
    version: Option<RuntimeVersion>,
    hash: Option<BlockHash>,
  ) -> Result<TypeLookup> {
    let mut inner = self.0.write().unwrap();
    Ok(inner.get_block_types(client, version, hash).await?)
  }

  pub fn add_init<F>(&self, func: F)
  where
    F: 'static + Send + Sync + Fn(&mut Types) -> Result<()>,
  {
    self
      .0
      .write()
      .unwrap()
      .add_init(InitRegistryFn(Box::new(func)))
  }
}
