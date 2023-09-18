use codec::{Compact, Decode, Input};
use serde_json::{json, Map, Value};

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
use sp_std::prelude::*;

use crate::error::*;
use crate::schema::*;
use crate::type_def::*;

pub mod de;

#[derive(Clone)]
pub struct TypeCodec {
  type_lookup: TypeLookup,
  ty: Type,
  id: TypeId,
}

impl TypeCodec {
  pub fn new(type_lookup: &TypeLookup, type_ref: TypeRef) -> Option<Self> {
    type_ref.ty.map(|ty| Self {
      type_lookup: type_lookup.clone(),
      ty,
      id: type_ref.id,
    })
  }

  pub fn decode_value<I: Input>(&self, input: &mut I, is_compact: bool) -> Result<Value> {
    self.ty.decode_value(&self.type_lookup, input, is_compact)
  }

  pub fn decode(&self, mut data: &[u8]) -> Result<Value> {
    self.decode_value(&mut data, false)
  }

  pub fn from_slice<'a, T>(&'a self, data: &'a [u8]) -> Result<T>
  where
    T: serde::de::Deserialize<'a>,
  {
    let mut deserializer = de::TypeDeserializer::from_slice(self, data);
    Ok(T::deserialize(&mut deserializer)?)
  }
}

impl TypeLookup {
  pub fn type_codec(&self, name: &str) -> Option<TypeCodec> {
    let type_ref = self.resolve(name);
    TypeCodec::new(self, type_ref)
  }

  pub fn decode_value<I: Input>(
    &self,
    type_id: TypeId,
    input: &mut I,
    is_compact: bool,
  ) -> Result<Value> {
    let ty = self
      .get_type(type_id)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {type_id:?}")))?;
    if ty.path().is_empty() {
      log::trace!("decode type[{type_id:?}]");
    } else {
      log::trace!("decode type[{type_id:?}]: {}", ty.path());
    }
    ty.decode_value(self, input, is_compact)
  }
}

impl Type {
  pub fn decode_value<I: Input>(
    &self,
    type_lookup: &TypeLookup,
    input: &mut I,
    is_compact: bool,
  ) -> Result<Value> {
    if !self.path().is_empty() {
      log::trace!("decode type: {}", self.path());
    }
    self
      .type_def
      .decode_value(self, type_lookup, input, is_compact)
  }
}

impl TypeDef {
  pub fn decode_value<I: Input>(
    &self,
    ty: &Type,
    type_lookup: &TypeLookup,
    input: &mut I,
    is_compact: bool,
  ) -> Result<Value> {
    match self {
      TypeDef::Composite(def) => def.decode_value(type_lookup, input, is_compact),
      TypeDef::Variant(def) => {
        if ty.path().segments == &["Option"] {
          def.decode_value(type_lookup, input, is_compact, true)
        } else {
          def.decode_value(type_lookup, input, is_compact, false)
        }
      }
      TypeDef::Sequence(def) => def.decode_value(type_lookup, input, is_compact),
      TypeDef::Array(def) => def.decode_value(type_lookup, input, is_compact),
      TypeDef::Tuple(def) => def.decode_value(type_lookup, input, is_compact),
      TypeDef::Primitive(prim) => {
        log::trace!("decode Primitive: {prim:?}, is_compact: {is_compact}");
        match prim {
          TypeDefPrimitive::Bool => match input.read_byte()? {
            0 => Ok(json!(false)),
            1 => Ok(json!(true)),
            num => Err(Error::DecodeTypeFailed(format!(
              "Invalid bool byte: {num:?}"
            ))),
          },
          TypeDefPrimitive::Char => {
            let ch = input.read_byte()? as char;
            Ok(json!(ch))
          }
          TypeDefPrimitive::Str => {
            let s = String::decode(input)?;
            Ok(json!(s))
          }
          TypeDefPrimitive::U8 => {
            let num = u8::decode(input)?;
            Ok(json!(num))
          }
          TypeDefPrimitive::U16 => {
            let num = if is_compact {
              Compact::<u16>::decode(input)?.0
            } else {
              u16::decode(input)?
            };
            Ok(json!(num))
          }
          TypeDefPrimitive::U32 => {
            let num = if is_compact {
              Compact::<u32>::decode(input)?.0
            } else {
              u32::decode(input)?
            };
            Ok(json!(num))
          }
          TypeDefPrimitive::U64 => {
            let num = if is_compact {
              Compact::<u64>::decode(input)?.0
            } else {
              u64::decode(input)?
            };
            Ok(json!(num))
          }
          TypeDefPrimitive::U128 => {
            let num = if is_compact {
              Compact::<u128>::decode(input)?.0
            } else {
              u128::decode(input)?
            };
            Ok(json!(num))
          }
          TypeDefPrimitive::U256 => {
            unimplemented!();
          }
          TypeDefPrimitive::I8 => {
            let num = i8::decode(input)?;
            Ok(json!(num))
          }
          TypeDefPrimitive::I16 => {
            let num = if is_compact {
              Compact::<u16>::decode(input)?.0 as i16
            } else {
              i16::decode(input)?
            };
            Ok(json!(num))
          }
          TypeDefPrimitive::I32 => {
            let num = if is_compact {
              Compact::<u32>::decode(input)?.0 as i32
            } else {
              i32::decode(input)?
            };
            Ok(json!(num))
          }
          TypeDefPrimitive::I64 => {
            let num = if is_compact {
              Compact::<u64>::decode(input)?.0 as i64
            } else {
              i64::decode(input)?
            };
            Ok(json!(num))
          }
          TypeDefPrimitive::I128 => {
            let num = if is_compact {
              Compact::<u128>::decode(input)?.0 as i128
            } else {
              i128::decode(input)?
            };
            Ok(json!(num))
          }
          TypeDefPrimitive::I256 => {
            unimplemented!();
          }
        }
      }
      TypeDef::Compact(def) => def.decode_value(type_lookup, input, is_compact),
    }
  }
}

fn decode_fields<I: Input>(
  fields: &Vec<Field>,
  is_struct: bool,
  type_lookup: &TypeLookup,
  input: &mut I,
  is_compact: bool,
) -> Result<Value> {
  let len = fields.len();
  if len == 0 {
    return Ok(Value::Null);
  }
  match fields.len() {
    0 => Ok(Value::Null),
    1 if fields[0].name.is_none() => {
      Ok(type_lookup.decode_value(fields[0].ty, input, is_compact)?)
    }
    len if is_struct => {
      let mut m = Map::with_capacity(len);
      for (idx, field) in fields.iter().enumerate() {
        let name = field
          .name
          .as_ref()
          .cloned()
          .unwrap_or_else(|| format!("{idx}"));
        log::trace!("decode Composite field: {name}");
        m.insert(name, type_lookup.decode_value(field.ty, input, is_compact)?);
      }
      Ok(m.into())
    }
    len => {
      log::trace!("decode Composite tuple fields");
      let mut arr = Vec::with_capacity(len);
      for field in fields.iter() {
        arr.push(type_lookup.decode_value(field.ty, input, is_compact)?);
      }
      Ok(arr.into())
    }
  }
}

impl TypeDefComposite {
  pub fn decode_value<I: Input>(
    &self,
    type_lookup: &TypeLookup,
    input: &mut I,
    is_compact: bool,
  ) -> Result<Value> {
    decode_fields(
      &self.fields,
      self.is_struct(),
      type_lookup,
      input,
      is_compact,
    )
  }
}

impl TypeDefVariant {
  pub fn decode_value<I: Input>(
    &self,
    type_lookup: &TypeLookup,
    input: &mut I,
    is_compact: bool,
    is_option: bool,
  ) -> Result<Value> {
    let val = input.read_byte()?;
    match (val, self.get_by_idx(val), is_option) {
      (0, Some(_variant), true) => Ok(Value::Null),
      (1, Some(variant), true) => decode_fields(
        &variant.fields,
        variant.is_struct(),
        type_lookup,
        input,
        is_compact,
      ),
      (_, Some(variant), _) if variant.fields.len() == 0 => Ok(json!(variant.name)),
      (_, Some(variant), _) => {
        let mut m = Map::new();
        let name = variant.name.clone();
        m.insert(
          name,
          decode_fields(
            &variant.fields,
            variant.is_struct(),
            type_lookup,
            input,
            is_compact,
          )?,
        );
        Ok(m.into())
      }
      (_, None, _) if val == 0 => Ok(Value::Null),
      (_, None, _) => {
        log::debug!(
          "Invalid variant: {}, bytes remaining: {:?}, variants: {:?}",
          val,
          input.remaining_len()?,
          self.variants
        );
        Err(Error::DecodeTypeFailed(format!("Invalid variant: {val}")))
      }
    }
  }
}

impl TypeDefSequence {
  pub fn decode_value<I: Input>(
    &self,
    type_lookup: &TypeLookup,
    input: &mut I,
    is_compact: bool,
  ) -> Result<Value> {
    let len = Compact::<u64>::decode(input)?.0 as usize;
    let mut vec = Vec::with_capacity(len.max(256));
    let ty = type_lookup
      .get_type(self.type_param)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {:?}", self.type_param)))?;
    for _ in 0..len {
      vec.push(ty.decode_value(type_lookup, input, is_compact)?);
    }
    Ok(vec.into())
  }
}

impl TypeDefArray {
  pub fn decode_value<I: Input>(
    &self,
    type_lookup: &TypeLookup,
    input: &mut I,
    is_compact: bool,
  ) -> Result<Value> {
    let len = self.len as usize;
    let mut vec = Vec::with_capacity(len);
    let ty = type_lookup
      .get_type(self.type_param)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {:?}", self.type_param)))?;
    for _ in 0..len {
      vec.push(ty.decode_value(type_lookup, input, is_compact)?);
    }
    Ok(vec.into())
  }
}

impl TypeDefTuple {
  pub fn decode_value<I: Input>(
    &self,
    type_lookup: &TypeLookup,
    input: &mut I,
    is_compact: bool,
  ) -> Result<Value> {
    match self.fields.len() {
      0 => Ok(Value::Null),
      1 => Ok(type_lookup.decode_value(self.fields[0], input, is_compact)?),
      len => {
        let mut vec = Vec::with_capacity(len);
        for field in &self.fields {
          vec.push(type_lookup.decode_value(*field, input, is_compact)?);
        }
        Ok(vec.into())
      }
    }
  }
}

impl TypeDefCompact {
  pub fn decode_value<I: Input>(
    &self,
    type_lookup: &TypeLookup,
    input: &mut I,
    _is_compact: bool,
  ) -> Result<Value> {
    type_lookup.decode_value(self.type_param, input, true)
  }
}
