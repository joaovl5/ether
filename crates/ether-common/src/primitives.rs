use std::collections::HashMap;
use std::path::PathBuf;

use derive_more::{Display, From, Into};

#[derive(Debug, PartialEq, Eq, Hash, From, Into, Clone, PartialOrd, Ord)]
pub struct StoreHash(String);

/// Any sort of hash that is *not* addressing a store object necessarily.
#[derive(Debug, PartialEq, Eq, Hash, From, Into, Clone, PartialOrd, Ord)]
pub struct GenericHash(String);

#[derive(Debug, PartialEq, Eq)]
pub struct StoreObject {
  pub hash : StoreHash,
  pub comes_from : Option<StoreHash>,
}

#[derive(Debug, PartialEq, Eq, Hash, From, Into, Clone, PartialOrd, Ord)]
pub struct RecipeInputName(String);

#[derive(Debug, PartialEq, Eq, Hash, From, Into, Clone, PartialOrd, Ord)]
pub struct RecipeOutputName(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Recipe {
  /// Optionally tie recipe to a single platform type.
  pub platform : Option<Platform>,
  /// What builder to use, and how.
  pub builder : RecipeBuilder,
  /// What will be exposed as outputs to the builder.
  ///
  /// By default, it will default to a single builder called `main`. All the
  /// outputs will be available through envvars like `ETHER_OUTPUT_MAIN`, where
  /// `MAIN` is derived from the output name. The names are case-insensitive
  /// and will all be upper-cased to the builder.
  ///
  /// Output paths are calculated by combining the recipe's hash with the
  /// output name.
  pub outputs : Vec<RecipeOutputName>,
  /// What will be exposed as inputs to the builder.
  ///
  /// These all will come from store paths, derived from the `StoreHash`
  /// values, and will get mapped to envvars like `ETHER_INPUT_SRC`, where
  /// `SRC` would be the name of an input. As with outputs, they're all
  /// upper-cased and thus are case-insensitive.
  pub inputs : HashMap<RecipeInputName, RecipeInput>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecipeInput {
  Hash(StoreHash),
  RecipeOutput {
    recipe_hash : StoreHash,
    output : RecipeOutputName,
  },
}

#[derive(Debug, PartialEq, Eq)]
pub enum RecipeBuilder {
  /// Fetch from some URL with some expected hash.
  FetchURL {
    url : String,
    expected_hash : GenericHash,
  },
  /// Runs an executable by the given arguments.
  Executable {
    /// Used to get the base store path of the executable.
    hash : StoreHash,
    args : ExecutableArgs,
  },
  /// Uses the executable output of another recipe as the builder.
  RecipeOutput {
    hash : StoreHash,
    output : RecipeOutputName,
    args : ExecutableArgs,
  },
}

/// Arguments for an executable builder.
#[derive(Debug, PartialEq, Eq)]
pub struct ExecutableArgs {
  /// Path relative to the store location. If store location is itself
  /// an executable and not a folder, this should be `None`.
  pub path : Option<PathBuf>,
  /// Arguments to pass into the executable at call time.
  pub args : Vec<String>,
  /// Environment variables for executable.
  pub env : HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Display)]
pub enum Platform {
  x86_64_linux,
  aarch64_linux,
}
