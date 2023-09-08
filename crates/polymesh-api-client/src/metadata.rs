#![allow(deprecated)]

#[cfg(not(feature = "std"))]
use alloc::collections::btree_map::BTreeMap;
#[cfg(feature = "std")]
use std::collections::BTreeMap;

#[cfg(any(feature = "v13", feature = "v12",))]
use frame_metadata::decode_different::{DecodeDifferent, DecodeDifferentArray};

#[cfg(feature = "v14")]
use scale_info::form::PortableForm;

#[cfg(not(feature = "std"))]
use alloc::{
  format,
  string::{String, ToString},
};
use sp_std::prelude::*;

use crate::error::*;
use crate::schema::*;
use crate::type_def::*;
use crate::*;

#[cfg(any(feature = "v13", feature = "v12",))]
fn decode_meta<B: 'static, O: 'static>(encoded: &DecodeDifferent<B, O>) -> Result<&O> {
  match encoded {
    DecodeDifferent::Decoded(val) => Ok(val),
    _ => Err(Error::MetadataParseFailed(format!(
      "Failed to decode value."
    ))),
  }
}

#[derive(Clone)]
pub struct Metadata {
  modules: BTreeMap<String, ModuleMetadata>,
  idx_map: BTreeMap<u8, String>,
}

impl Metadata {
  #[cfg(feature = "v12")]
  pub fn from_v12_metadata(
    md: frame_metadata::v12::RuntimeMetadataV12,
    lookup: &mut Types,
  ) -> Result<Self> {
    let mut api_md = Self {
      modules: BTreeMap::new(),
      idx_map: BTreeMap::new(),
    };

    // Top-level event/error/call types.
    let mut mod_events = TypeDefVariant::new();
    let mut mod_errors = TypeDefVariant::new();
    let mut mod_calls = TypeDefVariant::new();

    // Decode module metadata.
    decode_meta(&md.modules)?
      .iter()
      .try_for_each(|m| -> Result<()> {
        let m = ModuleMetadata::from_v12_meta(m, lookup)?;
        let name = m.name.clone();
        mod_events.insert(m.index, &name, m.event_ref.clone());
        mod_errors.insert(m.index, &name, m.error_ref.clone());
        mod_calls.insert(m.index, &name, m.call_ref.clone());
        api_md.idx_map.insert(m.index, name.clone());
        api_md.modules.insert(name, m);
        Ok(())
      })?;

    let raw_event_ref = lookup.insert_type("RawEvent", TypeDef::Variant(mod_events));
    lookup.insert_new_type("Event", raw_event_ref);
    let raw_error_ref = lookup.insert_type("RawError", TypeDef::Variant(mod_errors));
    lookup.insert_new_type("DispatchErrorModule", raw_error_ref);
    // Define 'RuntimeCall' type.
    lookup.insert_type("RuntimeCall", TypeDef::Variant(mod_calls));

    Ok(api_md)
  }

  #[cfg(feature = "v13")]
  pub fn from_v13_metadata(
    md: frame_metadata::v13::RuntimeMetadataV13,
    lookup: &mut Types,
  ) -> Result<Self> {
    let mut api_md = Self {
      modules: BTreeMap::new(),
      idx_map: BTreeMap::new(),
    };

    // Top-level event/error/call types.
    let mut mod_events = TypeDefVariant::new();
    let mut mod_errors = TypeDefVariant::new();
    let mut mod_calls = TypeDefVariant::new();

    // Decode module metadata.
    decode_meta(&md.modules)?
      .iter()
      .try_for_each(|m| -> Result<()> {
        let m = ModuleMetadata::from_v13_meta(m, lookup)?;
        let name = m.name.clone();
        mod_events.insert(m.index, &name, m.event_ref.clone());
        mod_errors.insert(m.index, &name, m.error_ref.clone());
        mod_calls.insert(m.index, &name, m.call_ref.clone());
        api_md.idx_map.insert(m.index, name.clone());
        api_md.modules.insert(name, m);
        Ok(())
      })?;

    let raw_event_ref = lookup.insert_type("RawEvent", TypeDef::Variant(mod_events));
    lookup.insert_new_type("Event", raw_event_ref);
    let raw_error_ref = lookup.insert_type("RawError", TypeDef::Variant(mod_errors));
    lookup.insert_new_type("DispatchErrorModule", raw_error_ref);
    // Define 'RuntimeCall' type.
    lookup.insert_type("RuntimeCall", TypeDef::Variant(mod_calls));

    Ok(api_md)
  }

  #[cfg(feature = "v14")]
  pub fn from_v14_metadata(
    md: frame_metadata::v14::RuntimeMetadataV14,
    lookup: &mut Types,
  ) -> Result<Self> {
    let mut api_md = Self {
      modules: BTreeMap::new(),
      idx_map: BTreeMap::new(),
    };

    let types = PortableRegistry::from(&md.types);

    // Import types from registry.
    lookup.import_v14_types(&types)?;

    // Top-level event/error/call types.
    let mut mod_events = TypeDefVariant::new();
    let mut mod_errors = TypeDefVariant::new();
    let mut mod_calls = TypeDefVariant::new();

    // Decode module metadata.
    md.pallets.iter().try_for_each(|m| -> Result<()> {
      let m = ModuleMetadata::from_v14_meta(m, &types, lookup)?;
      let name = m.name.clone();
      mod_events.insert(m.index, &name, m.event_ref.clone());
      mod_errors.insert(m.index, &name, m.error_ref.clone());
      mod_calls.insert(m.index, &name, m.call_ref.clone());
      api_md.idx_map.insert(m.index, name.clone());
      api_md.modules.insert(name, m);
      Ok(())
    })?;

    let raw_event_ref = lookup.insert_type("RawEvent", TypeDef::Variant(mod_events));
    lookup.insert_new_type("RuntimeEvent", raw_event_ref);

    let raw_error_ref = lookup.insert_type("RawError", TypeDef::Variant(mod_errors));
    lookup.insert_new_type("DispatchErrorModule", raw_error_ref);
    // Define 'RuntimeCall' type.
    lookup.insert_type("RuntimeCall", TypeDef::Variant(mod_calls));

    Ok(api_md)
  }

  pub fn get_module(&self, name: &str) -> Option<&ModuleMetadata> {
    self.modules.get(name)
  }
}

#[derive(Clone)]
pub struct ModuleMetadata {
  name: String,
  index: u8,
  funcs: BTreeMap<String, FuncMetadata>,
  events: BTreeMap<String, EventMetadata>,
  errors: BTreeMap<String, ErrorMetadata>,
  err_idx_map: BTreeMap<u8, String>,
  event_ref: Option<TypeId>,
  error_ref: Option<TypeId>,
  call_ref: Option<TypeId>,
}

impl ModuleMetadata {
  #[cfg(feature = "v12")]
  fn from_v12_meta(md: &frame_metadata::v12::ModuleMetadata, lookup: &mut Types) -> Result<Self> {
    let mod_idx = md.index;
    let mod_name = decode_meta(&md.name)?;
    let mut module = Self {
      name: mod_name.clone(),
      index: mod_idx,
      funcs: BTreeMap::new(),
      events: BTreeMap::new(),
      errors: BTreeMap::new(),
      err_idx_map: BTreeMap::new(),
      event_ref: None,
      error_ref: None,
      call_ref: None,
    };

    // Decode module functions.
    if let Some(calls) = &md.calls {
      // Module RawCall type.
      let mut raw_calls = TypeDefVariant::new();

      decode_meta(calls)?
        .iter()
        .enumerate()
        .try_for_each(|(func_idx, md)| -> Result<()> {
          let (func, ty_ref) =
            FuncMetadata::from_v12_meta(&mod_name, mod_idx, func_idx as u8, md, lookup)?;
          let name = func.name.clone();
          raw_calls.insert(func.func_idx, &name, ty_ref);
          module.funcs.insert(name, func);
          Ok(())
        })?;
      module.call_ref = Some(lookup.insert_type(
        &format!("{}::RawCall", mod_name),
        TypeDef::Variant(raw_calls),
      ));
    }

    // Decode module events.
    if let Some(events) = &md.event {
      // Module RawEvent type.
      let mut raw_events = TypeDefVariant::new();

      decode_meta(events)?
        .iter()
        .enumerate()
        .try_for_each(|(event_idx, md)| -> Result<()> {
          let (event, ty_ref) =
            EventMetadata::from_v12_meta(&mod_name, mod_idx, event_idx as u8, md, lookup)?;
          let name = event.name.clone();
          raw_events.insert(event.event_idx, &name, ty_ref);
          module.events.insert(name, event);
          Ok(())
        })?;
      module.event_ref = Some(lookup.insert_type(
        &format!("{}::RawEvent", mod_name),
        TypeDef::Variant(raw_events),
      ));
    }

    // Decode module errors.
    // Module RawError type.
    let mut raw_errors = TypeDefVariant::new();

    decode_meta(&md.errors)?
      .iter()
      .enumerate()
      .try_for_each(|(error_idx, md)| -> Result<()> {
        let error = ErrorMetadata::from_v12_meta(&mod_name, mod_idx, error_idx as u8, md)?;
        let name = error.name.clone();
        raw_errors.insert(error.error_idx, &name, None);
        module.err_idx_map.insert(error.error_idx, name.clone());
        module.errors.insert(name, error);
        Ok(())
      })?;
    module.error_ref = Some(lookup.insert_type(
      &format!("{}::RawError", mod_name),
      TypeDef::Variant(raw_errors),
    ));

    Ok(module)
  }

  #[cfg(feature = "v13")]
  fn from_v13_meta(md: &frame_metadata::v13::ModuleMetadata, lookup: &mut Types) -> Result<Self> {
    let mod_idx = md.index;
    let mod_name = decode_meta(&md.name)?;
    let mut module = Self {
      name: mod_name.clone(),
      index: mod_idx,
      funcs: BTreeMap::new(),
      events: BTreeMap::new(),
      errors: BTreeMap::new(),
      err_idx_map: BTreeMap::new(),
      event_ref: None,
      error_ref: None,
      call_ref: None,
    };

    // Decode module functions.
    if let Some(calls) = &md.calls {
      // Module RawCall type.
      let mut raw_calls = TypeDefVariant::new();

      decode_meta(calls)?
        .iter()
        .enumerate()
        .try_for_each(|(func_idx, md)| -> Result<()> {
          let (func, ty_ref) =
            FuncMetadata::from_v13_meta(&mod_name, mod_idx, func_idx as u8, md, lookup)?;
          let name = func.name.clone();
          raw_calls.insert(func.func_idx, &name, ty_ref);
          module.funcs.insert(name, func);
          Ok(())
        })?;
      module.call_ref = Some(lookup.insert_type(
        &format!("{}::RawCall", mod_name),
        TypeDef::Variant(raw_calls),
      ));
    }

    // Decode module events.
    if let Some(events) = &md.event {
      // Module RawEvent type.
      let mut raw_events = TypeDefVariant::new();

      decode_meta(events)?
        .iter()
        .enumerate()
        .try_for_each(|(event_idx, md)| -> Result<()> {
          let (event, ty_ref) =
            EventMetadata::from_v13_meta(&mod_name, mod_idx, event_idx as u8, md, lookup)?;
          let name = event.name.clone();
          raw_events.insert(event.event_idx, &name, ty_ref);
          module.events.insert(name, event);
          Ok(())
        })?;
      module.event_ref = Some(lookup.insert_type(
        &format!("{}::RawEvent", mod_name),
        TypeDef::Variant(raw_events),
      ));
    }

    // Decode module errors.
    // Module RawError type.
    let mut raw_errors = TypeDefVariant::new();

    decode_meta(&md.errors)?
      .iter()
      .enumerate()
      .try_for_each(|(error_idx, md)| -> Result<()> {
        let error = ErrorMetadata::from_v13_meta(&mod_name, mod_idx, error_idx as u8, md)?;
        let name = error.name.clone();
        raw_errors.insert(error.error_idx, &name, None);
        module.err_idx_map.insert(error.error_idx, name.clone());
        module.errors.insert(name, error);
        Ok(())
      })?;
    module.error_ref = Some(lookup.insert_type(
      &format!("{}::RawError", mod_name),
      TypeDef::Variant(raw_errors),
    ));

    Ok(module)
  }

  #[cfg(feature = "v14")]
  fn from_v14_meta(
    md: &frame_metadata::v14::PalletMetadata<PortableForm>,
    types: &PortableRegistry,
    lookup: &mut Types,
  ) -> Result<Self> {
    let mod_idx = md.index;
    let mod_name = &md.name;
    let mut module = Self {
      name: mod_name.clone(),
      index: mod_idx,
      funcs: BTreeMap::new(),
      events: BTreeMap::new(),
      errors: BTreeMap::new(),
      err_idx_map: BTreeMap::new(),
      event_ref: None,
      error_ref: None,
      call_ref: None,
    };

    // Decode module functions.
    if let Some(calls) = &md.calls {
      // Module RawCall type.
      let mut raw_calls = TypeDefVariant::new();

      let call_ty = types
        .resolve(calls.ty.id())
        .expect("Missing Pallet call type");
      match call_ty.type_def() {
        TypeDef::Variant(v) => {
          v.variants.iter().try_for_each(|md| -> Result<()> {
            let (func, ty_ref) =
              FuncMetadata::from_v14_meta(&mod_name, mod_idx, md, types, lookup)?;
            let name = func.name.clone();
            raw_calls.insert(func.func_idx, &name, ty_ref);
            module.funcs.insert(name, func);
            Ok(())
          })?;
        }
        _ => {
          unimplemented!("Only Variant type supported for Pallet Call type.");
        }
      }
      module.call_ref = Some(lookup.insert_type(
        &format!("{}::RawCall", mod_name),
        TypeDef::Variant(raw_calls),
      ));
    }

    // Decode module events.
    if let Some(events) = &md.event {
      // Module RawEvent type.
      let mut raw_events = TypeDefVariant::new();

      let event_ty = types
        .resolve(events.ty.id())
        .expect("Missing Pallet event type");
      match event_ty.type_def() {
        TypeDef::Variant(v) => {
          v.variants.iter().try_for_each(|md| -> Result<()> {
            let (event, ty_ref) =
              EventMetadata::from_v14_meta(&mod_name, mod_idx, md, types, lookup)?;
            let name = event.name.clone();
            raw_events.insert(event.event_idx, &name, ty_ref);
            module.events.insert(name, event);
            Ok(())
          })?;
        }
        _ => {
          unimplemented!("Only Variant type supported for Pallet Event type.");
        }
      }
      module.event_ref = Some(lookup.insert_type(
        &format!("{}::RawEvent", mod_name),
        TypeDef::Variant(raw_events),
      ));
    }

    // Decode module errors.
    if let Some(error) = &md.error {
      // Module RawError type.
      let mut raw_errors = TypeDefVariant::new();

      let extra_bytes = lookup.parse_type("[u8; 3]")?;
      let error_ty = types
        .resolve(error.ty.id())
        .expect("Missing Pallet error type");
      match error_ty.type_def() {
        TypeDef::Variant(v) => {
          v.variants.iter().try_for_each(|md| -> Result<()> {
            let error = ErrorMetadata::from_v14_meta(&mod_name, mod_idx, md)?;
            let name = error.name.clone();
            raw_errors.insert(error.error_idx, &name, Some(extra_bytes.clone()));
            module.err_idx_map.insert(error.error_idx, name.clone());
            module.errors.insert(name, error);
            Ok(())
          })?;
        }
        _ => {
          unimplemented!("Only Variant type supported for Pallet Error type.");
        }
      }
      module.error_ref = Some(lookup.insert_type(
        &format!("{}::RawError", mod_name),
        TypeDef::Variant(raw_errors),
      ));
    }

    Ok(module)
  }
}

#[derive(Debug, Clone)]
pub struct NamedType {
  pub name: String,
  pub ty_id: TypeId,
}

impl NamedType {
  pub fn new(name: &str, lookup: &mut Types) -> Result<Self> {
    let ty_id = lookup.parse_type(name)?;
    let named = Self {
      name: name.into(),
      ty_id,
    };

    Ok(named)
  }

  #[cfg(feature = "v14")]
  pub fn new_field_type(md: &Field, types: &PortableRegistry) -> Result<Self> {
    let ty = types
      .resolve(md.ty)
      .ok_or_else(|| Error::MetadataParseFailed(format!("Failed to resolve type.")))?;
    let name = md
      .type_name
      .as_ref()
      .map(|ty_name| {
        // Trim junk from `type_name`.
        let name = if ty_name.starts_with("/*«*/") {
          let end = ty_name.len() - 6;
          &ty_name[6..end]
        } else {
          &ty_name[..]
        }
        .trim();
        if is_type_compact(ty) {
          format!("Compact<{}>", name)
        } else {
          name.to_string()
        }
      })
      .unwrap_or_else(|| get_type_name(ty, types, false));
    let named = Self {
      name: name.into(),
      ty_id: md.ty,
    };

    Ok(named)
  }
}

#[derive(Clone)]
pub struct EventMetadata {
  pub mod_name: String,
  pub name: String,
  pub event_idx: u8,
  pub args: Vec<NamedType>,
  pub docs: Docs,
}

impl EventMetadata {
  #[cfg(feature = "v12")]
  fn from_v12_meta(
    mod_name: &str,
    _mod_idx: u8,
    event_idx: u8,
    md: &frame_metadata::v12::EventMetadata,
    lookup: &mut Types,
  ) -> Result<(Self, Option<TypeId>)> {
    let mut event = Self {
      mod_name: mod_name.into(),
      name: decode_meta(&md.name)?.clone(),
      event_idx,
      args: Vec::new(),
      docs: Docs::from_v12_meta(&md.documentation)?,
    };

    let mut event_tuple = Vec::new();

    // Decode event arguments.
    decode_meta(&md.arguments)?
      .iter()
      .try_for_each(|name| -> Result<()> {
        let arg = NamedType::new(name, lookup)?;
        event_tuple.push(arg.ty_id.clone());
        event.args.push(arg);
        Ok(())
      })?;

    let event_ref = if event_tuple.len() > 0 {
      let type_name = format!("{}::RawEvent::{}", mod_name, event.name);
      Some(lookup.insert_type(&type_name, TypeDef::new_tuple(event_tuple)))
    } else {
      None
    };

    Ok((event, event_ref))
  }

  #[cfg(feature = "v13")]
  fn from_v13_meta(
    mod_name: &str,
    _mod_idx: u8,
    event_idx: u8,
    md: &frame_metadata::v13::EventMetadata,
    lookup: &mut Types,
  ) -> Result<(Self, Option<TypeId>)> {
    let mut event = Self {
      mod_name: mod_name.into(),
      name: decode_meta(&md.name)?.clone(),
      event_idx,
      args: Vec::new(),
      docs: Docs::from_v13_meta(&md.documentation)?,
    };

    let mut event_tuple = Vec::new();

    // Decode event arguments.
    decode_meta(&md.arguments)?
      .iter()
      .try_for_each(|name| -> Result<()> {
        let arg = NamedType::new(name, lookup)?;
        event_tuple.push(arg.ty_id.clone());
        event.args.push(arg);
        Ok(())
      })?;

    let event_ref = if event_tuple.len() > 0 {
      let type_name = format!("{}::RawEvent::{}", mod_name, event.name);
      Some(lookup.insert_type(&type_name, TypeDef::new_tuple(event_tuple)))
    } else {
      None
    };

    Ok((event, event_ref))
  }

  #[cfg(feature = "v14")]
  fn from_v14_meta(
    mod_name: &str,
    _mod_idx: u8,
    md: &Variant,
    types: &PortableRegistry,
    lookup: &mut Types,
  ) -> Result<(Self, Option<TypeId>)> {
    let mut event = Self {
      mod_name: mod_name.into(),
      name: md.name.clone(),
      event_idx: md.index,
      args: Vec::new(),
      docs: Docs::from_v14_meta(&md.docs),
    };

    let mut event_tuple = Vec::new();

    // Decode event arguments.
    md.fields.iter().try_for_each(|md| -> Result<()> {
      let arg = NamedType::new_field_type(md, types)?;
      log::trace!("-- Event: {mod_name}.{}: field: {md:?}", event.name);
      event_tuple.push(arg.ty_id.clone());
      event.args.push(arg);
      Ok(())
    })?;

    let event_ref = if event_tuple.len() > 0 {
      log::trace!("-- Event: {mod_name}.{}({event_tuple:?})", event.name);
      let type_name = format!("{}::RawEvent::{}", mod_name, event.name);
      Some(lookup.insert_type(&type_name, TypeDef::new_tuple(event_tuple)))
    } else {
      None
    };

    Ok((event, event_ref))
  }
}

#[derive(Clone)]
pub struct ErrorMetadata {
  pub mod_name: String,
  pub name: String,
  pub error_idx: u8,
  pub docs: Docs,
}

impl ErrorMetadata {
  #[cfg(feature = "v12")]
  fn from_v12_meta(
    mod_name: &str,
    _mod_idx: u8,
    error_idx: u8,
    md: &frame_metadata::v12::ErrorMetadata,
  ) -> Result<Self> {
    Ok(Self {
      mod_name: mod_name.into(),
      name: decode_meta(&md.name)?.clone(),
      error_idx,
      docs: Docs::from_v12_meta(&md.documentation)?,
    })
  }

  #[cfg(feature = "v13")]
  fn from_v13_meta(
    mod_name: &str,
    _mod_idx: u8,
    error_idx: u8,
    md: &frame_metadata::v13::ErrorMetadata,
  ) -> Result<Self> {
    Ok(Self {
      mod_name: mod_name.into(),
      name: decode_meta(&md.name)?.clone(),
      error_idx,
      docs: Docs::from_v13_meta(&md.documentation)?,
    })
  }

  #[cfg(feature = "v14")]
  fn from_v14_meta(mod_name: &str, _mod_idx: u8, md: &Variant) -> Result<Self> {
    Ok(Self {
      mod_name: mod_name.into(),
      name: md.name.clone(),
      error_idx: md.index,
      docs: Docs::from_v14_meta(&md.docs),
    })
  }
}

#[derive(Clone)]
pub struct FuncMetadata {
  pub mod_name: String,
  pub name: String,
  pub mod_idx: u8,
  pub func_idx: u8,
  pub args: Vec<FuncArg>,
  pub docs: Docs,
}

impl FuncMetadata {
  #[cfg(feature = "v12")]
  fn from_v12_meta(
    mod_name: &str,
    mod_idx: u8,
    func_idx: u8,
    md: &frame_metadata::v12::FunctionMetadata,
    lookup: &mut Types,
  ) -> Result<(Self, Option<TypeId>)> {
    let mut func = Self {
      mod_name: mod_name.into(),
      name: decode_meta(&md.name)?.clone(),
      mod_idx,
      func_idx,
      args: Vec::new(),
      docs: Docs::from_v12_meta(&md.documentation)?,
    };

    let mut func_tuple = Vec::new();

    // Decode function arguments.
    decode_meta(&md.arguments)?
      .iter()
      .try_for_each(|md| -> Result<()> {
        let arg = FuncArg::from_v12_meta(md, lookup)?;
        func_tuple.push(arg.ty.ty_id.clone());
        func.args.push(arg);
        Ok(())
      })?;

    let func_ref = if func_tuple.len() > 0 {
      let type_name = format!("{}::RawFunc::{}", mod_name, func.name);
      Some(lookup.insert_type(&type_name, TypeDef::new_tuple(func_tuple)))
    } else {
      None
    };

    Ok((func, func_ref))
  }

  #[cfg(feature = "v13")]
  fn from_v13_meta(
    mod_name: &str,
    mod_idx: u8,
    func_idx: u8,
    md: &frame_metadata::v13::FunctionMetadata,
    lookup: &mut Types,
  ) -> Result<(Self, Option<TypeId>)> {
    let mut func = Self {
      mod_name: mod_name.into(),
      name: decode_meta(&md.name)?.clone(),
      mod_idx,
      func_idx,
      args: Vec::new(),
      docs: Docs::from_v13_meta(&md.documentation)?,
    };

    let mut func_tuple = Vec::new();

    // Decode function arguments.
    decode_meta(&md.arguments)?
      .iter()
      .try_for_each(|md| -> Result<()> {
        let arg = FuncArg::from_v13_meta(md, lookup)?;
        func_tuple.push(arg.ty.ty_id.clone());
        func.args.push(arg);
        Ok(())
      })?;

    let func_ref = if func_tuple.len() > 0 {
      let type_name = format!("{}::RawFunc::{}", mod_name, func.name);
      Some(lookup.insert_type(&type_name, TypeDef::new_tuple(func_tuple)))
    } else {
      None
    };

    Ok((func, func_ref))
  }

  #[cfg(feature = "v14")]
  fn from_v14_meta(
    mod_name: &str,
    mod_idx: u8,
    md: &Variant,
    types: &PortableRegistry,
    lookup: &mut Types,
  ) -> Result<(Self, Option<TypeId>)> {
    let mut func = Self {
      mod_name: mod_name.into(),
      name: md.name.clone(),
      mod_idx,
      func_idx: md.index,
      args: Vec::new(),
      docs: Docs::from_v14_meta(&md.docs),
    };

    let mut func_tuple = Vec::new();

    // Decode function arguments.
    md.fields.iter().try_for_each(|md| -> Result<()> {
      let arg = FuncArg::from_v14_meta(md, types)?;
      func_tuple.push(arg.ty.ty_id.clone());
      func.args.push(arg);
      Ok(())
    })?;

    let func_ref = if func_tuple.len() > 0 {
      let type_name = format!("{}::RawFunc::{}", mod_name, func.name);
      Some(lookup.insert_type(&type_name, TypeDef::new_tuple(func_tuple)))
    } else {
      None
    };

    Ok((func, func_ref))
  }
}

#[derive(Clone)]
pub struct FuncArg {
  pub name: String,
  pub ty: NamedType,
}

impl FuncArg {
  #[cfg(feature = "v12")]
  fn from_v12_meta(
    md: &frame_metadata::v12::FunctionArgumentMetadata,
    lookup: &mut Types,
  ) -> Result<Self> {
    let arg = Self {
      name: decode_meta(&md.name)?.clone(),
      ty: NamedType::new(decode_meta(&md.ty)?, lookup)?,
    };

    Ok(arg)
  }

  #[cfg(feature = "v13")]
  fn from_v13_meta(
    md: &frame_metadata::v13::FunctionArgumentMetadata,
    lookup: &mut Types,
  ) -> Result<Self> {
    let arg = Self {
      name: decode_meta(&md.name)?.clone(),
      ty: NamedType::new(decode_meta(&md.ty)?, lookup)?,
    };

    Ok(arg)
  }

  #[cfg(feature = "v14")]
  fn from_v14_meta(md: &Field, types: &PortableRegistry) -> Result<Self> {
    let arg = Self {
      name: md.name.clone().unwrap_or_default(),
      ty: NamedType::new_field_type(md, types)?,
    };

    Ok(arg)
  }
}

#[derive(Clone)]
pub struct Docs {
  pub lines: Vec<String>,
}

impl Docs {
  #[cfg(feature = "v12")]
  fn from_v12_meta(md: &DecodeDifferentArray<&'static str, String>) -> Result<Self> {
    Ok(Self {
      lines: decode_meta(md)?.clone(),
    })
  }

  #[cfg(feature = "v13")]
  fn from_v13_meta(md: &DecodeDifferentArray<&'static str, String>) -> Result<Self> {
    Ok(Self {
      lines: decode_meta(md)?.clone(),
    })
  }

  #[cfg(feature = "v14")]
  fn from_v14_meta(docs: &[String]) -> Self {
    Self {
      lines: docs.to_vec(),
    }
  }
}
