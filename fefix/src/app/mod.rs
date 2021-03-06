//! Data structures and interfaces to work with FIX messages.

use rust_embed::RustEmbed;
use std::fmt;

pub mod fix42;
pub mod slr;

pub trait FieldsIterator {
    fn next(&mut self) -> Option<(u32, &slr::FixFieldValue)>;
}

pub trait TsrMessage {
    fn get_field(&self, msg_type: i32) -> slr::FixFieldValue;
    fn set_field(&mut self, msg_type: i32, val: slr::FixFieldValue);
}

pub trait TsrMessageRef: Default
//where
//    for<'a> &'a Self: Iterator<Item = slr::FixFieldValue>,
{
    fn get_field(&self, msg_type: u32) -> Option<&slr::FixFieldValue>;
    fn set_field(&mut self, msg_type: u32, val: slr::FixFieldValue);
}

/// Which [`Dictionary`](fefix::Dictionary) version to use.
#[derive(Copy, Debug, Clone)]
#[non_exhaustive]
pub enum Version {
    Fix40,
    Fix41,
    Fix42,
    Fix43,
    Fix44,
    Fix50,
    Fix50SP1,
    Fix50SP2,
    Fixt11,
}

impl Version {
    /// Returns a [`String`](String) with the QuickFIX definition file for `self`
    /// as its
    /// content. The QuickFix definition files are extracted and decompressed
    /// from the binary without filesystem access.
    pub fn get_quickfix_spec(&self) -> String {
        let filename = match self {
            Version::Fix40 => "FIX-4.0.xml",
            Version::Fix41 => "FIX-4.1.xml",
            Version::Fix42 => "FIX-4.2.xml",
            Version::Fix43 => "FIX-4.3.xml",
            Version::Fix44 => "FIX-4.4.xml",
            Version::Fix50 => "FIX-5.0.xml",
            Version::Fix50SP1 => "FIX-5.0-SP1.xml",
            Version::Fix50SP2 => "FIX-5.0-SP2.xml",
            Version::Fixt11 => "FIXT-1.1.xml",
        };
        let xml_spec = QuickFixDicts::get(filename).expect(filename);
        std::str::from_utf8(&*xml_spec).unwrap().to_string()
    }

    #[cfg(test)]
    pub(crate) fn all() -> impl Iterator<Item = Self> {
        vec![
            Version::Fix40,
            Version::Fix41,
            Version::Fix42,
            Version::Fix43,
            Version::Fix44,
            Version::Fix50,
            Version::Fix50SP1,
            Version::Fix50SP2,
            Version::Fixt11,
        ]
        .into_iter()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str = match self {
            Version::Fix40 => "FIX-4.0",
            Version::Fix41 => "FIX-4.1",
            Version::Fix42 => "FIX-4.2",
            Version::Fix43 => "FIX-4.3",
            Version::Fix44 => "FIX-4.4",
            Version::Fix50 => "FIX-5.0",
            Version::Fix50SP1 => "FIX-5.0-SP1",
            Version::Fix50SP2 => "FIX-5.0-SP2",
            Version::Fixt11 => "FIXT-1.1",
        };
        write!(f, "{}", as_str)
    }
}

#[derive(RustEmbed)]
#[folder = "resources/quickfix/"]
struct QuickFixDicts;

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_versions_have_quickfix_spec() {
        assert!(Version::all()
            .map(|version| version.get_quickfix_spec())
            .all(|spec| spec.len() > 0));
    }

    #[test]
    fn all_versions_have_different_quickfix_spec() {
        let mut set: HashSet<String> = HashSet::default();
        Version::all()
            .map(|version| set.insert(version.get_quickfix_spec()))
            .count();
        assert_eq!(set.len(), Version::all().count());
    }

    #[test]
    fn all_versions_have_xml_valid_quickfix_spec() {
        assert!(Version::all()
            .map(|version| version.get_quickfix_spec())
            .all(|spec| roxmltree::Document::parse(spec.as_str()).is_ok()));
    }
}
