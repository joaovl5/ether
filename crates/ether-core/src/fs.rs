use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum FSObject {
  File(FSRegularFile),
  Directory(FSDirectory),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FSRegularFile {
  executable : bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FSDirectory {
  entries : HashMap<String, FSRegularFile>,
}
