use quick_xml::{
  encoding::EncodingError,
  events::{attributes::AttrError, Event},
  name::PrefixDeclaration,
  Decoder, Reader,
};
use std::{
  collections::HashMap,
  io::BufRead,
  num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

mod alternate_content;
mod defs;

pub use alternate_content::AlternateContentStack;

#[derive(Error, Debug)]
pub enum SdkError {
  #[error("quick_xml error: {0:?}")]
  QuickXmlError(#[from] quick_xml::Error),
  #[error("quick_xml encoding error: {0:?}")]
  QuickEncodingError(#[from] EncodingError),
  #[error("quick_xml attr error: {0:?}")]
  AttrError(#[from] AttrError),
  #[error("ParseIntError: {0:?}")]
  ParseIntError(#[from] ParseIntError),
  #[error("ParseFloatError: {0:?}")]
  ParseFloatError(#[from] ParseFloatError),
  #[error("StdFmtError: {0:?}")]
  StdFmtError(#[from] std::fmt::Error),
  #[error("StdIoError: {0:?}")]
  StdIoError(#[from] std::io::Error),
  #[cfg(feature = "parts")]
  #[error("ZipError: {0:?}")]
  ZipError(#[from] zip::result::ZipError),
  #[error("mismatch error (expected {expected:?}, found {found:?})")]
  MismatchError { expected: String, found: String },
  #[error("`{0}` common error")]
  CommonError(String),
}

pub trait XmlReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, SdkError>;

  fn decoder(&self) -> Decoder;
}

pub struct IoReader<R: BufRead> {
  reader: Reader<R>,
  buf: Vec<u8>,
}

impl<R: BufRead> IoReader<R> {
  pub fn new(reader: Reader<R>) -> Self {
    Self {
      reader,
      buf: Vec::new(),
    }
  }
}

impl<'de, R: BufRead> XmlReader<'de> for IoReader<R> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    self.buf.clear();

    Ok(self.reader.read_event_into(&mut self.buf)?.into_owned())
  }

  #[inline]
  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}

pub struct SliceReader<'de> {
  reader: Reader<&'de [u8]>,
}

impl<'de> SliceReader<'de> {
  pub fn new(reader: Reader<&'de [u8]>) -> Self {
    Self { reader }
  }
}

impl<'de> XmlReader<'de> for SliceReader<'de> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    Ok(self.reader.read_event()?)
  }

  #[inline]
  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}

pub fn resolve_zip_file_path(path: &str) -> String {
  let mut stack = Vec::new();

  for component in path.split('/') {
    match component {
      "" | "." => {
        // Ignore empty components and current directory symbol
      }
      ".." => {
        // Go up one directory if possible
        stack.pop();
      }
      _ => {
        // Add the component to the path
        stack.push(component);
      }
    }
  }
  // Join the components back into a path
  stack.join("/")
}

pub fn combine_paths(parent_path: &str, child_path: &str) -> String {
  if child_path.starts_with("/") {
    child_path.to_string()
  } else {
    format!("{}{}", parent_path, child_path)
  }
}

#[inline]
pub(crate) fn from_reader_inner<R: BufRead>(reader: R) -> Result<IoReader<R>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_reader(reader);
  xml_reader.config_mut().check_end_names = false;

  Ok(IoReader::new(xml_reader))
}

#[inline]
pub(crate) fn from_str_inner(s: &str) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_str(s);
  xml_reader.config_mut().check_end_names = false;

  Ok(SliceReader::new(xml_reader))
}

#[inline]
pub fn parse_bool_bytes(b: &[u8]) -> Result<bool, SdkError> {
  match b {
    b"true" | b"1" | b"True" | b"TRUE" | b"t" | b"Yes" | b"YES" | b"yes" | b"y" => Ok(true),
    b"false" | b"0" | b"False" | b"FALSE" | b"f" | b"No" | b"NO" | b"no" | b"n" | b"" => Ok(false),
    other => Err(SdkError::CommonError(
      String::from_utf8_lossy(other).into_owned(),
    )),
  }
}

macro_rules! expect_event_start {
  ($xml_reader:expr, $xml_event:expr, $tag:expr, $xmlns_map:expr) => {{
    if let Some((e, empty_tag)) = $xml_event {
      (e, empty_tag)
    } else {
      let (e, empty_tag) = loop {
        match $xml_reader.next()? {
          quick_xml::events::Event::Start(b) => break (b, false),
          quick_xml::events::Event::Empty(b) => break (b, true),
          quick_xml::events::Event::Eof => Err(super::super::common::SdkError::CommonError(
            "Unexpected end of file".into(),
          ))?,
          _ => continue,
        }
      };

      match e.local_name().as_ref() {
        $tag => (),
        _ => {
          Err(super::super::common::SdkError::MismatchError {
            expected: String::from_utf8_lossy($tag).to_string(),
            found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
          })?;
        }
      }

      super::super::common::update_namespace_map($xml_reader, &e, &mut $xmlns_map)?;

      (e, empty_tag)
    }
  }};
}

pub(crate) use expect_event_start;

pub(crate) fn skip_element<'a, R>(reader: &mut R) -> Result<(), SdkError>
where
  R: XmlReader<'a>,
{
  let mut level = 1;
  while level > 0 {
    match reader.next()? {
      quick_xml::events::Event::Start(_) => level += 1,
      quick_xml::events::Event::End(_) => level -= 1,
      quick_xml::events::Event::Eof => Err(SdkError::CommonError("Unexpected end of file".into()))?,
      _ => {}
    }
  }
  Ok(())
}

pub(crate) fn read_element_text<'a, R>(
  reader: &mut R,
  start: &quick_xml::events::BytesStart<'a>,
) -> Result<Option<String>, SdkError>
where
  R: XmlReader<'a>,
{
  let mut text = None;

  loop {
    match reader.next()? {
      quick_xml::events::Event::Text(t) => {
        text = Some(text.unwrap_or_default() + t.decode()?.as_ref());
      }
      quick_xml::events::Event::GeneralRef(t) => {
        if let Some(entity) = quick_xml::escape::resolve_xml_entity(t.decode()?.as_ref()) {
          text = Some(text.unwrap_or_default() + entity);
        }
      }
      quick_xml::events::Event::End(e) if e.name() == start.name() => return Ok(text),
      quick_xml::events::Event::Eof => Err(SdkError::CommonError("Unexpected end of file".into()))?,
      event => Err(SdkError::CommonError(format!(
        "Unexpected event: {event:?}"
      )))?,
    }
  }
}

pub(crate) fn update_namespace_map<'a, R>(
  xml_reader: &R,
  e: &quick_xml::events::BytesStart<'a>,
  xmlns_map: &mut HashMap<String, String>,
) -> Result<(), SdkError>
where
  R: XmlReader<'a>,
{
  for attr in e.attributes().with_checks(false) {
    let attr = attr?;
    if let Some(namespace) = attr.key.as_namespace_binding() {
      let value = attr
        .decode_and_unescape_value(xml_reader.decoder())?
        .to_string();
      let key = match namespace {
        PrefixDeclaration::Named(prefix) => String::from_utf8_lossy(prefix),
        PrefixDeclaration::Default => "".into(),
      };
      xmlns_map.insert(key.to_string(), value);
    }
  }
  Ok(())
}

pub(crate) fn get_element_namespace<'a, 'b>(
  e: &quick_xml::events::BytesStart<'a>,
  xmlns_map: &'b HashMap<String, String>,
) -> Result<&'b str, SdkError> {
  let prefix = e
    .name()
    .prefix()
    .map(|prefix| String::from_utf8_lossy(prefix.as_ref()).to_string());

  if let Some(ns) = prefix {
    Ok(
      xmlns_map
        .get(&ns)
        .ok_or(SdkError::CommonError("Unknown prefix: {ns}".into()))?
        .as_str(),
    )
  } else {
    Ok(xmlns_map.get("").map(|ns| ns.as_str()).unwrap_or(""))
  }
}
