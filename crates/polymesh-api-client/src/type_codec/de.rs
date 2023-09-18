use serde::de::{
  self, value::StringDeserializer, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess,
  SeqAccess, VariantAccess, Visitor,
};

use crate::error::{Error, Result};

use super::*;

impl de::Error for Error {
  fn custom<T>(msg: T) -> Self
  where
    T: core::fmt::Display,
  {
    Self::DecodeTypeFailed(format!("{msg}"))
  }
}

pub struct TypeDeserializer<'de> {
  type_lookup: &'de TypeLookup,
  type_id: TypeId,
  data: &'de [u8],
  is_compact: bool,
  is_top: bool,
}

impl<'de> TypeDeserializer<'de> {
  pub fn from_slice(ty_codec: &'de TypeCodec, data: &'de [u8]) -> Self {
    Self {
      type_lookup: &ty_codec.type_lookup,
      type_id: ty_codec.id,
      data,
      is_compact: false,
      is_top: true,
    }
  }

  pub fn remaining_len(&mut self) -> Result<Option<usize>> {
    Ok(self.data.remaining_len()?)
  }

  pub fn is_empty(&mut self) -> Result<bool> {
    Ok(self.remaining_len()?.unwrap_or_default() == 0)
  }

  fn read_byte(&mut self) -> Result<u8> {
    Ok(self.data.read_byte()?)
  }

  fn decode<T: Decode>(&mut self) -> Result<T> {
    Ok(T::decode(&mut self.data)?)
  }

  fn deserialize_type_id<V: Visitor<'de>>(
    &mut self,
    type_id: TypeId,
    visitor: V,
  ) -> Result<V::Value> {
    let ty = self
      .type_lookup
      .get_type(type_id)
      .ok_or_else(|| Error::DecodeTypeFailed(format!("Missing type_id: {type_id:?}")))?;
    if ty.path().is_empty() {
      log::trace!("deserialize type[{type_id:?}]");
    } else {
      log::trace!("deserialize type[{type_id:?}]: {}", ty.path());
    }
    self.deserialize_type(ty, visitor)
  }

  fn deserialize_type_sequence<V: Visitor<'de>>(
    &mut self,
    type_id: TypeId,
    len: usize,
    visitor: V,
  ) -> Result<V::Value> {
    visitor.visit_seq(SequenceVisitor::new(self, type_id, len))
  }

  fn deserialize_type<V: Visitor<'de>>(&mut self, ty: Type, visitor: V) -> Result<V::Value> {
    if !ty.path().is_empty() {
      log::trace!("deserialize SCALE type: {}", ty.path());
    }
    self.deserialize_type_def(ty, visitor)
  }

  fn deserialize_type_def<V: Visitor<'de>>(&mut self, ty: Type, visitor: V) -> Result<V::Value> {
    match &ty.type_def {
      TypeDef::Composite(def) => self.deserialize_composite(def, visitor),
      TypeDef::Variant(def) => {
        if ty.path().segments == &["Option"] {
          self.deserialize_option(def, visitor)
        } else {
          self.deserialize_variant(def, visitor)
        }
      }
      TypeDef::Sequence(def) => self.deserialize_sequence(def, visitor),
      TypeDef::Array(def) => self.deserialize_array(def, visitor),
      TypeDef::Tuple(def) => self.deserialize_tuple(def, visitor),
      TypeDef::Primitive(prim) => {
        let is_compact = self.is_compact;
        log::trace!("decode Primitive: {prim:?}, is_compact: {is_compact}");
        match prim {
          TypeDefPrimitive::Bool => match self.read_byte()? {
            0 => Ok(visitor.visit_bool::<Error>(false)?),
            1 => Ok(visitor.visit_bool::<Error>(true)?),
            num => Err(Error::DecodeTypeFailed(format!(
              "Invalid bool byte: {num:?}"
            ))),
          },
          TypeDefPrimitive::Char => {
            let ch = self.read_byte()? as char;
            Ok(visitor.visit_char::<Error>(ch)?)
          }
          TypeDefPrimitive::Str => {
            let s = self.decode::<String>()?;
            Ok(visitor.visit_str::<Error>(&s)?)
          }
          TypeDefPrimitive::U8 => {
            let num = self.decode::<u8>()?;
            Ok(visitor.visit_u8::<Error>(num)?)
          }
          TypeDefPrimitive::U16 => {
            let num = if is_compact {
              self.decode::<Compact<u16>>()?.0
            } else {
              self.decode::<u16>()?
            };
            Ok(visitor.visit_u16::<Error>(num)?)
          }
          TypeDefPrimitive::U32 => {
            let num = if is_compact {
              self.decode::<Compact<u32>>()?.0
            } else {
              self.decode::<u32>()?
            };
            Ok(visitor.visit_u32::<Error>(num)?)
          }
          TypeDefPrimitive::U64 => {
            let num = if is_compact {
              self.decode::<Compact<u64>>()?.0
            } else {
              self.decode::<u64>()?
            };
            Ok(visitor.visit_u64::<Error>(num)?)
          }
          TypeDefPrimitive::U128 => {
            let num = if is_compact {
              self.decode::<Compact<u128>>()?.0
            } else {
              self.decode::<u128>()?
            };
            Ok(visitor.visit_u128::<Error>(num)?)
          }
          TypeDefPrimitive::U256 => {
            unimplemented!();
          }
          TypeDefPrimitive::I8 => {
            let num = self.decode::<i8>()?;
            Ok(visitor.visit_i8::<Error>(num)?)
          }
          TypeDefPrimitive::I16 => {
            let num = if is_compact {
              self.decode::<Compact<u16>>()?.0 as i16
            } else {
              self.decode::<i16>()?
            };
            Ok(visitor.visit_i16::<Error>(num)?)
          }
          TypeDefPrimitive::I32 => {
            let num = if is_compact {
              self.decode::<Compact<u32>>()?.0 as i32
            } else {
              self.decode::<i32>()?
            };
            Ok(visitor.visit_i32::<Error>(num)?)
          }
          TypeDefPrimitive::I64 => {
            let num = if is_compact {
              self.decode::<Compact<u64>>()?.0 as i64
            } else {
              self.decode::<i64>()?
            };
            Ok(visitor.visit_i64::<Error>(num)?)
          }
          TypeDefPrimitive::I128 => {
            let num = if is_compact {
              self.decode::<Compact<u128>>()?.0 as i128
            } else {
              self.decode::<i128>()?
            };
            Ok(visitor.visit_i128::<Error>(num)?)
          }
          TypeDefPrimitive::I256 => {
            unimplemented!();
          }
        }
      }
      TypeDef::Compact(def) => {
        self.is_compact = true;
        let res = self.deserialize_type_id(def.type_param, visitor);
        self.is_compact = false;
        res
      }
    }
  }

  fn deserialize_composite<V: Visitor<'de>>(
    &mut self,
    def: &TypeDefComposite,
    visitor: V,
  ) -> Result<V::Value> {
    let fields = def.fields.as_slice();
    let is_struct = def.is_struct();
    if is_struct {
      log::trace!("deserialize struct: len={:?}", fields.len());
      visitor.visit_map(FieldsVisitor::new_struct(self, fields))
    } else {
      match fields.len() {
        0 => {
          log::trace!("deserialize unit struct");
          visitor.visit_unit()
        }
        1 => {
          log::trace!("deserialize newtype struct");
          self.deserialize_type_id(fields[0].ty, visitor)
        }
        len => {
          log::trace!("deserialize tuple struct: len={len:?}");
          visitor.visit_seq(FieldsVisitor::new_tuple(self, fields))
        }
      }
    }
  }

  fn deserialize_option<V: Visitor<'de>>(
    &mut self,
    def: &TypeDefVariant,
    visitor: V,
  ) -> Result<V::Value> {
    let val = self.read_byte()?;
    log::trace!(
      "deserialize option: len={:?}, idx={:?}",
      def.variants.len(),
      val
    );
    match (val, def.get_by_idx(val)) {
      (0, Some(variant)) => {
        log::trace!("deserialize None: {:?}", variant);
        visitor.visit_none()
      }
      (1, Some(variant)) if variant.fields.len() == 1 => {
        log::trace!("deserialize Some: {:?}", variant);
        self.type_id = variant.fields[0].ty;
        visitor.visit_some(self)
      }
      _ => {
        log::debug!(
          "Invalid Option variant: {}, bytes remaining: {:?}, variants: {:?}",
          val,
          self.remaining_len()?,
          def.variants
        );
        Err(Error::DecodeTypeFailed(format!("Invalid variant: {val}")))
      }
    }
  }

  fn deserialize_variant<V: Visitor<'de>>(
    &mut self,
    def: &TypeDefVariant,
    visitor: V,
  ) -> Result<V::Value> {
    let val = self.read_byte()?;
    log::trace!(
      "deserialize variant: len={:?}, idx={:?}",
      def.variants.len(),
      val
    );
    match def.get_by_idx(val) {
      Some(variant) => {
        log::trace!("deserialize variant: {:?}", variant);
        visitor.visit_enum(FieldsVisitor::new_enum(self, variant))
      }
      // Empty enum with no variants.
      None if val == 0 => visitor.visit_unit(),
      None => {
        log::debug!(
          "Invalid variant: {}, bytes remaining: {:?}, variants: {:?}",
          val,
          self.remaining_len()?,
          def.variants
        );
        Err(Error::DecodeTypeFailed(format!("Invalid variant: {val}")))
      }
    }
  }

  fn deserialize_sequence<V: Visitor<'de>>(
    &mut self,
    def: &TypeDefSequence,
    visitor: V,
  ) -> Result<V::Value> {
    let len = self.decode::<Compact<u64>>()?.0 as usize;
    log::trace!("deserialize sequence: len={len:?}");
    self.deserialize_type_sequence(def.type_param, len, visitor)
  }

  fn deserialize_array<V: Visitor<'de>>(
    &mut self,
    def: &TypeDefArray,
    visitor: V,
  ) -> Result<V::Value> {
    let len = def.len as usize;
    log::trace!("deserialize array: len={len:?}");
    self.deserialize_type_sequence(def.type_param, len, visitor)
  }

  fn deserialize_tuple<V: Visitor<'de>>(
    &mut self,
    def: &TypeDefTuple,
    visitor: V,
  ) -> Result<V::Value> {
    log::trace!("deserialize tuple: len={:?}", def.fields.len());
    visitor.visit_seq(TupleVisitor::new(self, def.fields.as_slice()))
  }
}

struct TupleVisitor<'de, 'a, 'b> {
  de: &'a mut TypeDeserializer<'de>,
  fields: &'b [TypeId],
}

impl<'de, 'a, 'b> TupleVisitor<'de, 'a, 'b> {
  fn new(de: &'a mut TypeDeserializer<'de>, fields: &'b [TypeId]) -> Self {
    Self { de, fields }
  }
}

impl<'de, 'a, 'b> SeqAccess<'de> for TupleVisitor<'de, 'a, 'b> {
  type Error = Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
  where
    T: DeserializeSeed<'de>,
  {
    // Check if there are more fields.
    match self.fields.split_first() {
      None => Ok(None),
      Some((type_id, fields)) => {
        self.fields = fields;
        self.de.type_id = *type_id;
        let val = seed.deserialize(&mut *self.de)?;
        Ok(Some(val))
      }
    }
  }
}

struct FieldsVisitor<'de, 'a, 'b> {
  de: &'a mut TypeDeserializer<'de>,
  fields: &'b [Field],
  variant: Option<&'b Variant>,
  is_struct: bool,
  next: usize,
}

impl<'de, 'a, 'b> FieldsVisitor<'de, 'a, 'b> {
  fn new_struct(de: &'a mut TypeDeserializer<'de>, fields: &'b [Field]) -> Self {
    Self {
      de,
      fields,
      variant: None,
      is_struct: true,
      next: 0,
    }
  }

  fn new_tuple(de: &'a mut TypeDeserializer<'de>, fields: &'b [Field]) -> Self {
    Self {
      de,
      fields,
      variant: None,
      is_struct: false,
      next: 0,
    }
  }

  fn new_enum(de: &'a mut TypeDeserializer<'de>, variant: &'b Variant) -> Self {
    Self {
      de,
      fields: variant.fields.as_slice(),
      is_struct: variant.is_struct(),
      variant: Some(variant),
      next: 0,
    }
  }
}

impl<'de, 'a, 'b> EnumAccess<'de> for FieldsVisitor<'de, 'a, 'b> {
  type Error = Error;
  type Variant = Self;

  fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant)>
  where
    V: DeserializeSeed<'de>,
  {
    let variant = self.variant.take().expect("tried to decode a non-enum");
    log::trace!("deserialize enum variant: {:?}", variant.name);
    let val =
      seed.deserialize::<StringDeserializer<Error>>(variant.name.clone().into_deserializer())?;
    Ok((val, self))
  }
}

impl<'de, 'a, 'b> VariantAccess<'de> for FieldsVisitor<'de, 'a, 'b> {
  type Error = Error;

  // If the `Visitor` expected this variant to be a unit variant, the input
  // should have been the plain string case handled in `deserialize_enum`.
  fn unit_variant(self) -> Result<()> {
    log::trace!("deserialize unit variant: {:?}", self.variant);
    let len = self.fields.len();
    if len == 0 {
      Ok(())
    } else {
      Err(Error::DecodeTypeFailed(format!(
        "expect unit variant, but have variant with {} fields",
        len
      )))
    }
  }

  // Newtype variants `enum E { N(u8)`
  fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
  where
    T: DeserializeSeed<'de>,
  {
    log::trace!("deserialize newtype variant: {:?}", self.variant);
    match self.fields.split_first() {
      Some((field, fields)) if fields.len() == 0 => {
        self.de.type_id = field.ty;
        seed.deserialize(&mut *self.de)
      }
      _ => Err(Error::DecodeTypeFailed(format!(
        "expect newtype variant with only one field, got variant with {} fields",
        self.fields.len()
      ))),
    }
  }

  // Tuple variants `enum E { T(u8, u8)`
  fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    log::trace!("deserialize tuple variant: {:?}", self.variant);
    visitor.visit_seq(self)
  }

  // Struct variants `enum E { S { r: u8, g: u8, b: u8 } }`
  fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    log::trace!("deserialize struct variant: {:?}", self.variant);
    if self.is_struct {
      visitor.visit_map(self)
    } else {
      // The variant doesn't have named fields, can't decode it as a struct variant.
      Err(Error::DecodeTypeFailed(
        "expect struct variant, got tuple variant".into(),
      ))
    }
  }
}

impl<'de, 'a, 'b> SeqAccess<'de> for FieldsVisitor<'de, 'a, 'b> {
  type Error = Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
  where
    T: DeserializeSeed<'de>,
  {
    // Check if there are more fields.
    log::trace!("deserialize tuple next field");
    match self.fields.split_first() {
      None => Ok(None),
      Some((field, fields)) => {
        log::trace!(
          "deserialize tuple field[{}] = {field:?}, remaining={}",
          self.next,
          fields.len()
        );
        self.fields = fields;
        self.de.type_id = field.ty;
        self.next += 1;
        let val = seed.deserialize(&mut *self.de)?;
        log::trace!("deserialize tuple field, return value");
        Ok(Some(val))
      }
    }
  }
}

impl<'de, 'a, 'b> MapAccess<'de> for FieldsVisitor<'de, 'a, 'b> {
  type Error = Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
  where
    K: DeserializeSeed<'de>,
  {
    // Check if there are more fields.
    match self.fields.first() {
      None => Ok(None),
      Some(field) => {
        let name = if let Some(name) = &field.name {
          log::trace!("deserialize struct name: {:?}", name);
          // Deserialize field name.
          seed.deserialize::<StringDeserializer<Error>>(name.clone().into_deserializer())?
        } else {
          return Err(Error::DecodeTypeFailed("struct field missing name".into()));
        };
        Ok(Some(name))
      }
    }
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
  where
    V: DeserializeSeed<'de>,
  {
    // Check if there are more fields.
    match self.fields.split_first() {
      None => Err(Error::DecodeTypeFailed(format!("no more fields to decode"))),
      Some((field, fields)) => {
        log::trace!("deserialize struct field[{}] = {field:?}", self.next);
        self.fields = fields;
        self.de.type_id = field.ty;
        self.next += 1;
        // Deserialize field value.
        seed.deserialize(&mut *self.de)
      }
    }
  }
}

struct SequenceVisitor<'de, 'a> {
  de: &'a mut TypeDeserializer<'de>,
  type_id: TypeId,
  len: usize,
}

impl<'de, 'a> SequenceVisitor<'de, 'a> {
  fn new(de: &'a mut TypeDeserializer<'de>, type_id: TypeId, len: usize) -> Self {
    Self { de, type_id, len }
  }
}

impl<'de, 'a> SeqAccess<'de> for SequenceVisitor<'de, 'a> {
  type Error = Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
  where
    T: DeserializeSeed<'de>,
  {
    // Check if there are no more elements.
    if self.len == 0 {
      return Ok(None);
    }
    self.len -= 1;
    // Deserialize an array element.
    self.de.type_id = self.type_id;
    seed.deserialize(&mut *self.de).map(Some)
  }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut TypeDeserializer<'de> {
  type Error = Error;

  fn is_human_readable(&self) -> bool {
    false
  }

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    let is_top = self.is_top;
    self.is_top = false;
    let res = self.deserialize_type_id(self.type_id, visitor)?;
    if is_top && !self.is_empty()? {
      log::trace!("----- buffer still has data: {:?}", self.remaining_len());
      return Err(Error::DecodeTypeFailed("buffer still has data left".into()));
    }
    Ok(res)
  }

  serde::forward_to_deserialize_any! {
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
    bytes byte_buf option unit unit_struct newtype_struct seq tuple
    tuple_struct map struct enum identifier ignored_any
  }
}
