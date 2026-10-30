#![cfg(feature = "pptx")]

use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation;
use std::str::FromStr;

fn presentation_xml(p: &str, r: &str) -> String {
  format!(
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<{p}:presentation xmlns:{p}="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:{r}="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <{p}:sldMasterIdLst><{p}:sldMasterId id="2147483648" {r}:id="rId1"/></{p}:sldMasterIdLst>
  <{p}:sldIdLst><{p}:sldId id="256" {r}:id="rId7"/><{p}:sldId id="257" {r}:id="rId8"/></{p}:sldIdLst>
  <{p}:sldSz cx="12192000" cy="6858000"/>
  <{p}:notesSz cx="6858000" cy="9144000"/>
</{p}:presentation>"#
  )
}

fn slide_relationship_ids(presentation: &Presentation) -> Vec<&str> {
  presentation
    .slide_id_list
    .as_ref()
    .unwrap()
    .p_sld_id
    .iter()
    .map(|slide_id| slide_id.relationship_id.as_str())
    .collect()
}

#[test]
fn reads_standard_prefixes() {
  let presentation = Presentation::from_str(&presentation_xml("p", "r")).unwrap();

  assert_eq!(slide_relationship_ids(&presentation), ["rId7", "rId8"]);
}

// Files rewritten by generic XML tools (e.g. Python's ElementTree) use ns0:, ns1:, ... prefixes
#[test]
fn reads_non_standard_prefixes() {
  let presentation = Presentation::from_str(&presentation_xml("ns0", "ns1")).unwrap();

  assert_eq!(slide_relationship_ids(&presentation), ["rId7", "rId8"]);
  assert_eq!(
    presentation
      .slide_master_id_list
      .as_ref()
      .unwrap()
      .p_sld_master_id[0]
      .relationship_id,
    "rId1"
  );
}

#[test]
fn ignores_attributes_in_unknown_namespaces() {
  let xml = presentation_xml("p", "r").replace(
    r#"<p:sldId id="256" r:id="rId7"/>"#,
    r#"<p:sldId xmlns:x="urn:example" id="256" r:id="rId7" x:id="other"/>"#,
  );

  let presentation = Presentation::from_str(&xml).unwrap();

  assert_eq!(slide_relationship_ids(&presentation), ["rId7", "rId8"]);
}
