use super::{defs, SdkError, XmlReader};

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
  ) -> Result<Option<quick_xml::events::BytesStart<'a>>, SdkError>
  where
    R: XmlReader<'a>,
  {
    match e.local_name().as_ref() {
      b"AlternateContent" => {
        self.stack.push(false);
        Ok(None)
      }
      b"Choice" => {
        let mut skip = *self.stack.last().ok_or(SdkError::UnknownError)?;
        // If we are not skipping choices, see if we should skip this choice
        if !skip {
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            match attr.key.as_ref() {
              b"Requires" => {
                let value = attr.decode_and_unescape_value(xml_reader.decoder())?;
                skip = !defs::SUPPORTED_NAMESPACES.contains(&value.as_ref());
                break;
              }
              _ => (),
            }
          }
        }

        if skip {
          Self::skip_element(xml_reader)?;
        }
        Ok(None)
      }
      b"Fallback" => {
        let skip = *self.stack.last().ok_or(SdkError::UnknownError)?;
        if skip {
          Self::skip_element(xml_reader)?;
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
        self.stack.pop().ok_or(SdkError::UnknownError)?;
      }
      // We'll only hit this if we don't skip the choice, so we want to skip all choices after this
      b"Choice" | b"Fallback" => {
        let last = self.stack.last_mut().ok_or(SdkError::UnknownError)?;
        *last = true;
      }
      _ => (),
    }
    Ok(())
  }

  fn skip_element<'a, R>(reader: &mut R) -> Result<(), SdkError>
  where
    R: XmlReader<'a>,
  {
    let mut level = 1;
    while level > 0 {
      match reader.next()? {
        quick_xml::events::Event::Start(_) => level += 1,
        quick_xml::events::Event::End(_) => level -= 1,
        quick_xml::events::Event::Eof => Err(SdkError::UnknownError)?,
        _ => {}
      }
    }
    Ok(())
  }
}
