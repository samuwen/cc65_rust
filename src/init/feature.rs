use crate::errors::unknown_option;

pub enum Feature {
  Unknown,
  DollarIsPc,
  LabelsWithoutColons,
  LooseStringTerm,
  LooseCharTerm,
  AtInIdentifiers,
  DollarInIdentifiers,
  LeadingDotInIdentifiers,
  OrgPerSeg,
  PcAssignment,
  MissingCharTerm,
  UbiquitousIdents,
  CComments,
  ForceRange,
  UnderlineInNumbers,
  Addrsize,
  BracketAsIndirect,
  StringEscapes,
}

impl Feature {
  pub fn from_string(s: String) -> Feature {
    match s.to_ascii_lowercase().as_ref() {
      "Unknown" => Feature::Unknown,
      "DollarIsPc" => Feature::DollarIsPc,
      "LabelsWithoutColons" => Feature::LabelsWithoutColons,
      "LooseStringTerm" => Feature::LooseStringTerm,
      "LooseCharTerm" => Feature::LooseCharTerm,
      "AtInIdentifiers" => Feature::AtInIdentifiers,
      "DollarInIdentifiers" => Feature::DollarInIdentifiers,
      "LeadingDotInIdentifiers" => Feature::LeadingDotInIdentifiers,
      "OrgPerSeg" => Feature::OrgPerSeg,
      "PcAssignment" => Feature::PcAssignment,
      "MissingCharTerm" => Feature::MissingCharTerm,
      "UbiquitousIdents" => Feature::UbiquitousIdents,
      "CComments" => Feature::CComments,
      "ForceRange" => Feature::ForceRange,
      "UnderlineInNumbers" => Feature::UnderlineInNumbers,
      "Addrsize" => Feature::Addrsize,
      "BracketAsIndirect" => Feature::BracketAsIndirect,
      "StringEscapes" => Feature::StringEscapes,
      _ => unknown_option(&s),
    }
  }
}
