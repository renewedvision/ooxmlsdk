#[derive(Clone, Debug, Default)]
pub struct CoreProperties {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
  pub category: Option<String>,
  pub content_status: Option<String>,
  pub created: Option<String>,
  pub creator: Option<String>,
  pub description: Option<String>,
  pub identifier: Option<String>,
  pub keywords: Option<String>,
  pub language: Option<String>,
  pub last_modified_by: Option<String>,
  pub last_printed: Option<String>,
  pub modified: Option<String>,
  pub revision: Option<String>,
  pub subject: Option<String>,
  pub title: Option<String>,
  pub version: Option<String>,
}

impl std::str::FromStr for CoreProperties {
  type Err = super::super::common::SdkError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = super::super::common::from_str_inner(s)?;

    Self::deserialize_inner(&mut xml_reader, None, Default::default())
  }
}

impl CoreProperties {
  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_reader_inner(reader)?;

    Self::deserialize_inner(&mut xml_reader, None, Default::default())
  }

  pub fn deserialize_inner<'de, R: super::super::common::XmlReader<'de>>(
    xml_reader: &mut R,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    mut xmlns_map: std::collections::HashMap<String, String>,
  ) -> Result<Self, super::super::common::SdkError> {
    let (e, empty_tag) = super::super::common::expect_event_start!(
      xml_reader,
      xml_event,
      b"coreProperties",
      xmlns_map
    );

    let mut xmlns = None;

    let mut mc_ignorable = None;

    let mut category: Option<String> = None;

    let mut content_status: Option<String> = None;

    let mut created: Option<String> = None;

    let mut creator: Option<String> = None;

    let mut description: Option<String> = None;

    let mut identifier: Option<String> = None;

    let mut keywords: Option<String> = None;

    let mut language: Option<String> = None;

    let mut last_modified_by: Option<String> = None;

    let mut last_printed: Option<String> = None;

    let mut modified: Option<String> = None;

    let mut revision: Option<String> = None;

    let mut subject: Option<String> = None;

    let mut title: Option<String> = None;

    let mut version: Option<String> = None;

    for attr in e.attributes() {
      let attr = attr?;
      match attr.key.as_ref() {
        b"xmlns" => {
          xmlns = Some(
            attr
              .decode_and_unescape_value(xml_reader.decoder())?
              .to_string(),
          );
        }
        b"mc:Ignorable" => {
          mc_ignorable = Some(
            attr
              .decode_and_unescape_value(xml_reader.decoder())?
              .to_string(),
          );
        }
        _ => (),
      }
    }

    if !empty_tag {
      loop {
        match xml_reader.next()? {
          quick_xml::events::Event::Start(e) => {
            let element_text = super::super::common::read_element_text(xml_reader, &e)?;
            const CP: &str =
              "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
            const DCTERMS: &str = "http://purl.org/dc/terms/";
            const DC: &str = "http://purl.org/dc/elements/1.1/";
            match (
              e.local_name().as_ref(),
              super::super::common::get_element_namespace(&e, &xmlns_map)?,
            ) {
              (b"category", CP) => category = element_text,
              (b"contentStatus", CP) => content_status = element_text,
              (b"created", DCTERMS) => created = element_text,
              (b"creator", DC) => creator = element_text,
              (b"description", DC) => description = element_text,
              (b"identifier", DC) => identifier = element_text,
              (b"keywords", CP) => keywords = element_text,
              (b"language", DC) => language = element_text,
              (b"lastModifiedBy", CP) => last_modified_by = element_text,
              (b"lastPrinted", CP) => last_printed = element_text,
              (b"modified", DCTERMS) => modified = element_text,
              (b"revision", CP) => revision = element_text,
              (b"subject", DC) => subject = element_text,
              (b"title", DC) => title = element_text,
              (b"version", CP) => version = element_text,
              _ => Err(super::super::common::SdkError::CommonError(
                "coreProperties".to_string(),
              ))?,
            }
          }
          quick_xml::events::Event::End(e) => {
            if e.local_name().as_ref() == b"coreProperties" {
              break;
            }
          }
          quick_xml::events::Event::Eof => Err(super::super::common::SdkError::CommonError(
            "Unexpected end of file".into(),
          ))?,
          _ => (),
        }
      }
    }

    Ok(Self {
      xmlns,
      xmlns_map,
      mc_ignorable,
      category,
      content_status,
      created,
      creator,
      description,
      identifier,
      keywords,
      language,
      last_modified_by,
      last_printed,
      modified,
      revision,
      subject,
      title,
      version,
    })
  }
}

impl CoreProperties {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    self.to_string_inner(if let Some(xmlns) = &self.xmlns {
      xmlns != "http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
    } else {
      true
    })
  }

  pub fn to_string_inner(&self, with_xmlns: bool) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut writer = String::new();

    writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;

    writer.write_char('<')?;

    if with_xmlns {
      writer.write_str("cp:coreProperties")?;
    } else {
      writer.write_str("coreProperties")?;
    }

    if let Some(xmlns) = &self.xmlns {
      writer.write_str(r#" xmlns=""#)?;
      writer.write_str(xmlns)?;
      writer.write_str("\"")?;
    }

    for (k, v) in &self.xmlns_map {
      writer.write_str(" xmlns:")?;
      writer.write_str(k)?;
      writer.write_str("=\"")?;
      writer.write_str(v)?;
      writer.write_str("\"")?;
    }

    if let Some(mc_ignorable) = &self.mc_ignorable {
      writer.write_str(r#" mc:Ignorable=""#)?;
      writer.write_str(mc_ignorable)?;
      writer.write_str("\"")?;
    }

    writer.write_char('>')?;

    if let Some(category) = &self.category {
      writer.write_str("<cp:category>")?;
      writer.write_str(&quick_xml::escape::escape(category))?;
      writer.write_str("</cp:category>")?;
    }

    if let Some(content_status) = &self.content_status {
      writer.write_str("<cp:contentStatus>")?;
      writer.write_str(&quick_xml::escape::escape(content_status))?;
      writer.write_str("</cp:contentStatus>")?;
    }

    if let Some(created) = &self.created {
      writer.write_str(r#"<dcterms:created xsi:type="dcterms:W3CDTF">"#)?;
      writer.write_str(&quick_xml::escape::escape(created))?;
      writer.write_str("</dcterms:created>")?;
    }

    if let Some(creator) = &self.creator {
      writer.write_str("<dc:creator>")?;
      writer.write_str(&quick_xml::escape::escape(creator))?;
      writer.write_str("</dc:creator>")?;
    }

    if let Some(description) = &self.description {
      writer.write_str("<dc:description>")?;
      writer.write_str(&quick_xml::escape::escape(description))?;
      writer.write_str("</dc:description>")?;
    }

    if let Some(identifier) = &self.identifier {
      writer.write_str("<dc:identifier>")?;
      writer.write_str(&quick_xml::escape::escape(identifier))?;
      writer.write_str("</dc:identifier>")?;
    }

    if let Some(keywords) = &self.keywords {
      writer.write_str("<cp:keywords>")?;
      writer.write_str(&quick_xml::escape::escape(keywords))?;
      writer.write_str("</cp:keywords>")?;
    }

    if let Some(language) = &self.language {
      writer.write_str("<dc:language>")?;
      writer.write_str(&quick_xml::escape::escape(language))?;
      writer.write_str("</dc:language>")?;
    }

    if let Some(last_modified_by) = &self.last_modified_by {
      writer.write_str("<cp:lastModifiedBy>")?;
      writer.write_str(&quick_xml::escape::escape(last_modified_by))?;
      writer.write_str("</cp:lastModifiedBy>")?;
    }

    if let Some(last_printed) = &self.last_printed {
      writer.write_str("<cp:lastPrinted>")?;
      writer.write_str(&quick_xml::escape::escape(last_printed))?;
      writer.write_str("</cp:lastPrinted>")?;
    }

    if let Some(modified) = &self.modified {
      writer.write_str(r#"<dcterms:modified xsi:type="dcterms:W3CDTF">"#)?;
      writer.write_str(&quick_xml::escape::escape(modified))?;
      writer.write_str("</dcterms:modified>")?;
    }

    if let Some(revision) = &self.revision {
      writer.write_str("<cp:revision>")?;
      writer.write_str(&quick_xml::escape::escape(revision))?;
      writer.write_str("</cp:revision>")?;
    }

    if let Some(subject) = &self.subject {
      writer.write_str("<dc:subject>")?;
      writer.write_str(&quick_xml::escape::escape(subject))?;
      writer.write_str("</dc:subject>")?;
    }

    if let Some(title) = &self.title {
      writer.write_str("<dc:title>")?;
      writer.write_str(&quick_xml::escape::escape(title))?;
      writer.write_str("</dc:title>")?;
    }

    if let Some(version) = &self.version {
      writer.write_str("<cp:version>")?;
      writer.write_str(&quick_xml::escape::escape(version))?;
      writer.write_str("</cp:version>")?;
    }

    writer.write_str("</")?;

    if with_xmlns {
      writer.write_str("cp:coreProperties")?;
    } else {
      writer.write_str("coreProperties")?;
    }

    writer.write_char('>')?;

    Ok(writer)
  }
}
