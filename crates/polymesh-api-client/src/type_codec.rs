use codec::{Compact, Decode, Encode, Input, Output};
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

  pub fn encode_to<T: Output + ?Sized>(&self, value: &Value, dest: &mut T) -> Result<()> {
    self.ty.encode_to(&self.type_lookup, value, dest, false)
  }

  pub fn encode(&self, value: &Value) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(1024);
    self.encode_to(value, &mut buf)?;
    Ok(buf)
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

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    type_id: TypeId,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
  ) -> Result<()> {
    let ty = self
      .get_type(type_id)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {type_id:?}")))?;
    if ty.path().is_empty() {
      log::trace!("encode type[{type_id:?}]");
    } else {
      log::trace!("encode type[{type_id:?}]: {}", ty.path());
    }
    ty.encode_to(self, value, dest, is_compact)
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

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
  ) -> Result<()> {
    if !self.path().is_empty() {
      log::trace!("encode type: {}", self.path());
    }
    self
      .type_def
      .encode_to(self, type_lookup, value, dest, is_compact)
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
        let is_option = ty.path().segments == &["Option"];
        def.decode_value(type_lookup, input, is_compact, is_option)
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

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    ty: &Type,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
  ) -> Result<()> {
    match self {
      TypeDef::Composite(def) => def.encode_to(type_lookup, value, dest, is_compact),
      TypeDef::Variant(def) => {
        let is_option = ty.path().segments == &["Option"];
        def.encode_to(type_lookup, value, dest, is_compact, is_option)
      }
      TypeDef::Sequence(def) => def.encode_to(type_lookup, value, dest, is_compact),
      TypeDef::Array(def) => def.encode_to(type_lookup, value, dest, is_compact),
      TypeDef::Tuple(def) => def.encode_to(type_lookup, value, dest, is_compact),
      TypeDef::Primitive(prim) => {
        log::trace!("encode Primitive: {prim:?}, is_compact: {is_compact}");
        match prim {
          TypeDefPrimitive::Bool => match value.as_bool() {
            Some(v) => {
              dest.push_byte(if v { 1 } else { 0 });
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a bool, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::Char => match value.as_str() {
            Some(v) if v.len() == 1 => {
              let ch = v.as_bytes()[0];
              dest.push_byte(ch);
              Ok(())
            }
            _ => Err(Error::EncodeTypeFailed(format!(
              "Expected a char (string), got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::Str => match value.as_str() {
            Some(v) => {
              v.encode_to(dest);
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a string, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::U8 => match value.as_u64() {
            Some(num) => {
              let num: u8 = num
                .try_into()
                .map_err(|e| Error::EncodeTypeFailed(format!("Not a u8 number: {e:?}")))?;
              num.encode_to(dest);
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::U16 => match value.as_u64() {
            Some(num) => {
              let num: u16 = num
                .try_into()
                .map_err(|e| Error::EncodeTypeFailed(format!("Not a u16 number: {e:?}")))?;
              if is_compact {
                Compact(num).encode_to(dest);
              } else {
                num.encode_to(dest);
              }
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::U32 => match value.as_u64() {
            Some(num) => {
              let num: u32 = num
                .try_into()
                .map_err(|e| Error::EncodeTypeFailed(format!("Not a u32 number: {e:?}")))?;
              if is_compact {
                Compact(num).encode_to(dest);
              } else {
                num.encode_to(dest);
              }
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::U64 => match value.as_u64() {
            Some(num) => {
              if is_compact {
                Compact(num).encode_to(dest);
              } else {
                num.encode_to(dest);
              }
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::U128 => match value.as_u64() {
            Some(num) => {
              let num: u128 = num
                .try_into()
                .map_err(|e| Error::EncodeTypeFailed(format!("Not a u128 number: {e:?}")))?;
              if is_compact {
                Compact(num).encode_to(dest);
              } else {
                num.encode_to(dest);
              }
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::U256 => {
            unimplemented!();
          }
          TypeDefPrimitive::I8 => match value.as_i64() {
            Some(num) => {
              let num: i8 = num
                .try_into()
                .map_err(|e| Error::EncodeTypeFailed(format!("Not a i8 number: {e:?}")))?;
              num.encode_to(dest);
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::I16 => match value.as_i64() {
            Some(num) => {
              let num: i16 = num
                .try_into()
                .map_err(|e| Error::EncodeTypeFailed(format!("Not a i16 number: {e:?}")))?;
              if is_compact {
                Compact(num as u128).encode_to(dest);
              } else {
                num.encode_to(dest);
              }
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::I32 => match value.as_i64() {
            Some(num) => {
              let num: i32 = num
                .try_into()
                .map_err(|e| Error::EncodeTypeFailed(format!("Not a i32 number: {e:?}")))?;
              if is_compact {
                Compact(num as u128).encode_to(dest);
              } else {
                num.encode_to(dest);
              }
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::I64 => match value.as_i64() {
            Some(num) => {
              if is_compact {
                Compact(num as u128).encode_to(dest);
              } else {
                num.encode_to(dest);
              }
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::I128 => match value.as_i64() {
            Some(num) => {
              let num: i128 = num
                .try_into()
                .map_err(|e| Error::EncodeTypeFailed(format!("Not a i128 number: {e:?}")))?;
              if is_compact {
                Compact(num as u128).encode_to(dest);
              } else {
                num.encode_to(dest);
              }
              Ok(())
            }
            None => Err(Error::EncodeTypeFailed(format!(
              "Expected a number, got {:?}",
              value
            ))),
          },
          TypeDefPrimitive::I256 => {
            unimplemented!();
          }
        }
      }
      TypeDef::Compact(def) => def.encode_to(type_lookup, value, dest, is_compact),
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

fn encode_struct_fields<T: Output + ?Sized>(
  fields: &Vec<Field>,
  type_lookup: &TypeLookup,
  value: &Value,
  dest: &mut T,
  is_compact: bool,
) -> Result<()> {
  let len = fields.len();
  match value {
    Value::Object(map) if map.len() == len => {
      for field in fields {
        let value = field.name.as_ref().and_then(|n| map.get(n));
        match value {
          Some(value) => {
            log::trace!("encode Composite struct field: {:?}", field);
            type_lookup.encode_to(field.ty, value, dest, is_compact)?;
          }
          None => {
            return Err(Error::EncodeTypeFailed(format!(
              "Encode struct missing field {:?}",
              field.name
            )));
          }
        }
      }
      Ok(())
    }
    Value::Object(map) => Err(Error::EncodeTypeFailed(format!(
      "Encode struct expected {len} field, got {}",
      map.len()
    ))),
    _ => Err(Error::EncodeTypeFailed(format!(
      "Encode struct expect an object got {:?}",
      value
    ))),
  }
}

fn encode_tuple_fields<T: Output + ?Sized>(
  fields: &Vec<Field>,
  type_lookup: &TypeLookup,
  value: &Value,
  dest: &mut T,
  is_compact: bool,
) -> Result<()> {
  let len = fields.len();
  if len == 1 {
    return type_lookup.encode_to(fields[0].ty, value, dest, is_compact);
  }
  match value.as_array() {
    Some(arr) if arr.len() == len => {
      for (v, field) in arr.into_iter().zip(fields.iter()) {
        log::trace!("encode Composite tuple field: {:?}", field);
        type_lookup.encode_to(field.ty, v, dest, is_compact)?;
      }
      Ok(())
    }
    Some(arr) => Err(Error::EncodeTypeFailed(format!(
      "Encode struct tuple expect array with length {len}, got {}",
      arr.len()
    ))),
    None => Err(Error::EncodeTypeFailed(format!(
      "Encode struct tuple expect array value got {:?}",
      value
    ))),
  }
}

fn encode_fields<T: Output + ?Sized>(
  fields: &Vec<Field>,
  is_struct: bool,
  type_lookup: &TypeLookup,
  value: &Value,
  dest: &mut T,
  is_compact: bool,
) -> Result<()> {
  if is_struct {
    encode_struct_fields(fields, type_lookup, value, dest, is_compact)
  } else {
    encode_tuple_fields(fields, type_lookup, value, dest, is_compact)
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

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
  ) -> Result<()> {
    encode_fields(
      &self.fields,
      self.is_struct(),
      type_lookup,
      value,
      dest,
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

  fn encode_option<T: Output + ?Sized>(
    &self,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
  ) -> Result<()> {
    if value.is_null() {
      dest.push_byte(0);
      return Ok(());
    }
    dest.push_byte(1);
    let variant = self
      .variants
      .get(1)
      .ok_or_else(|| Error::EncodeTypeFailed("Option type doesn't have a Some variant".into()))?;
    encode_fields(
      &variant.fields,
      variant.is_struct(),
      type_lookup,
      value,
      dest,
      is_compact,
    )
  }

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
    is_option: bool,
  ) -> Result<()> {
    if is_option {
      return self.encode_option(type_lookup, value, dest, is_compact);
    }
    let len = self.variants.len();
    match value {
      Value::Null if len == 0 => {
        // unit
        dest.push_byte(0);
        Ok(())
      }
      Value::String(s) => match self.get_by_name(&s) {
        Some(v) if v.fields.len() == 0 => {
          log::trace!("encode enum variant: {:?}", v);
          dest.push_byte(v.index);
          Ok(())
        }
        Some(v) => Err(Error::EncodeTypeFailed(format!(
          "Variant {} has fields, got just the name.",
          v.name
        ))),
        None => Err(Error::EncodeTypeFailed(format!("Unknown variant name {s}"))),
      },
      Value::Object(map) if map.len() == 1 => match map.iter().next() {
        Some((key, value)) => match self.get_by_name(&key) {
          Some(v) if v.fields.len() == 0 => {
            log::trace!("encode enum variant: {:?}", v);
            dest.push_byte(v.index);
            Ok(())
          }
          Some(v) => {
            log::trace!("encode enum variant: {:?}", v);
            dest.push_byte(v.index);
            if v.fields.len() > 0 {
              encode_fields(
                &v.fields,
                v.is_struct(),
                type_lookup,
                value,
                dest,
                is_compact,
              )
            } else {
              Ok(())
            }
          }
          None => Err(Error::EncodeTypeFailed(format!(
            "Unknown variant {:?}",
            map
          ))),
        },
        None => Err(Error::EncodeTypeFailed(format!(
          "Unknown variant {:?}",
          map
        ))),
      },
      Value::Object(map) => Err(Error::EncodeTypeFailed(format!(
        "Expect a variant, got a map with the wrong number of fields {}",
        map.len()
      ))),
      value => Err(Error::EncodeTypeFailed(format!(
        "Expect a variant, got {value:?}"
      ))),
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
    let ty = type_lookup
      .get_type(self.type_param)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {:?}", self.type_param)))?;
    if ty.is_u8() {
      log::trace!("--- decode byte sequence[{len}]: {:?}", ty);
      let mut vec = Vec::with_capacity(len);
      // Byte array.
      for _ in 0..len {
        vec.push(input.read_byte()?);
      }
      Ok(Value::String(hex::encode(vec)))
    } else {
      let mut vec = Vec::with_capacity(len.max(256));
      for _ in 0..len {
        vec.push(ty.decode_value(type_lookup, input, is_compact)?);
      }
      Ok(vec.into())
    }
  }

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
  ) -> Result<()> {
    let ty = type_lookup
      .get_type(self.type_param)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {:?}", self.type_param)))?;
    match value {
      Value::Array(arr) => {
        let len = Compact::<u64>(arr.len() as u64);
        len.encode_to(dest);
        log::trace!("encode sequence: len={}", arr.len());
        for v in arr {
          ty.encode_to(type_lookup, v, dest, is_compact)?;
        }
        Ok(())
      }
      Value::String(s) if ty.is_u8() => {
        let off = if s.starts_with("0x") { 2 } else { 0 };
        let arr = hex::decode(&s[off..])?;
        log::trace!("--- encode byte sequence[{}]: {:?}", arr.len(), ty);
        let len = Compact::<u64>(arr.len() as u64);
        len.encode_to(dest);
        // Try hex decoding for byte arrays.
        dest.write(&arr[..]);
        Ok(())
      }
      _ => Err(Error::EncodeTypeFailed(format!(
        "Encode sequence expect array value got {:?}",
        value
      ))),
    }
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
    let ty = type_lookup
      .get_type(self.type_param)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {:?}", self.type_param)))?;
    if ty.is_u8() {
      log::trace!("--- decode byte array[{len}]: {:?}", ty);
      let mut vec = Vec::with_capacity(len);
      // Byte array.
      for _ in 0..len {
        vec.push(input.read_byte()?);
      }
      Ok(Value::String(hex::encode(vec)))
    } else {
      let mut vec = Vec::with_capacity(len);
      for _ in 0..len {
        vec.push(ty.decode_value(type_lookup, input, is_compact)?);
      }
      Ok(vec.into())
    }
  }

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
  ) -> Result<()> {
    let len = self.len as usize;
    let ty = type_lookup
      .get_type(self.type_param)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {:?}", self.type_param)))?;
    match value {
      Value::Array(arr) if arr.len() == len => {
        log::trace!("encode array: len={len}");
        for v in arr {
          ty.encode_to(type_lookup, v, dest, is_compact)?;
        }
        Ok(())
      }
      Value::Array(arr) => Err(Error::EncodeTypeFailed(format!(
        "Expect array with length {len}, got {}",
        arr.len()
      ))),
      Value::String(s) if ty.is_u8() && s.len() >= 2 * len => {
        log::trace!("--- encode byte array[{len}]: {:?}", ty);
        // Try hex decoding for byte arrays.
        let off = if s.starts_with("0x") { 2 } else { 0 };
        let arr = hex::decode(&s[off..])?;
        dest.write(&arr[..]);
        Ok(())
      }
      _ => Err(Error::EncodeTypeFailed(format!(
        "Expect array value got {:?}",
        value
      ))),
    }
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

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    is_compact: bool,
  ) -> Result<()> {
    let len = self.fields.len();
    log::trace!("encode tuple: len={len}");
    if len == 1 {
      return type_lookup.encode_to(self.fields[0], value, dest, is_compact);
    }
    match value.as_array() {
      Some(arr) if arr.len() == len => {
        for (v, field) in arr.into_iter().zip(self.fields.iter()) {
          type_lookup.encode_to(*field, v, dest, is_compact)?;
        }
        Ok(())
      }
      Some(arr) => Err(Error::EncodeTypeFailed(format!(
        "Encode tuple expect array with length {len}, got {}",
        arr.len()
      ))),
      None => Err(Error::EncodeTypeFailed(format!(
        "Encode tuple expect array value got {:?}",
        value
      ))),
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

  pub fn encode_to<T: Output + ?Sized>(
    &self,
    type_lookup: &TypeLookup,
    value: &Value,
    dest: &mut T,
    _is_compact: bool,
  ) -> Result<()> {
    type_lookup.encode_to(self.type_param, value, dest, true)
  }
}
