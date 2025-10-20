use super::{defs, get_element_namespace, update_namespace_map, SdkError, XmlReader};

#[derive(Default)]
pub struct AlternateContentStack {
  // Each stack element indicates whether we are skipping choices.
  // This is set to true after we process a Choice or Fallback element
  stack: Vec<bool>,
}

impl AlternateContentStack {
  // Handle an element starting tag
  pub fn on_start<'a, R>(
    &mut self,
    e: quick_xml::events::BytesStart<'a>,
    xml_reader: &mut R,
    xmlns_map: &mut std::collections::HashMap<String, String>,
  ) -> Result<Option<quick_xml::events::BytesStart<'a>>, SdkError>
  where
    R: XmlReader<'a>,
  {
    update_namespace_map(xml_reader, &e, xmlns_map)?;
    match (
      e.local_name().as_ref(),
      get_element_namespace(&e, xmlns_map)?,
    ) {
      (b"AlternateContent", "http://schemas.openxmlformats.org/markup-compatibility/2006") => {
        self.stack.push(false);
        Ok(None)
      }
      (b"Choice", "http://schemas.openxmlformats.org/markup-compatibility/2006") => {
        let mut skip = *wrap_error(self.stack.last())?;
        // If we are not skipping choices, see if we should skip this choice
        if !skip {
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            if attr.key.as_ref() == b"Requires" {
              let value = attr.decode_and_unescape_value(xml_reader.decoder())?;
              skip = !defs::SUPPORTED_NAMESPACES.contains(&value.as_ref());
              break;
            }
          }
        }

        if skip {
          super::skip_element(xml_reader)?;
        }
        Ok(None)
      }
      (b"Fallback", "http://schemas.openxmlformats.org/markup-compatibility/2006") => {
        let skip = *wrap_error(self.stack.last())?;
        if skip {
          super::skip_element(xml_reader)?;
        }
        Ok(None)
      }
      _ => Ok(Some(e)),
    }
  }

  // Handle an element closing tag
  pub fn on_end<'a>(&mut self, e: quick_xml::events::BytesEnd<'a>) -> Result<(), SdkError> {
    match e.local_name().as_ref() {
      b"AlternateContent" => {
        wrap_error(self.stack.pop())?;
      }
      // We'll only hit this if we don't skip the choice, so we want to skip all choices after this
      b"Choice" | b"Fallback" => {
        let last = wrap_error(self.stack.last_mut())?;
        *last = true;
      }
      _ => (),
    }
    Ok(())
  }
}

fn wrap_error<T>(opt: Option<T>) -> Result<T, SdkError> {
  opt.ok_or(SdkError::CommonError(
    "Error parsing AlternateContent".into(),
  ))
}
