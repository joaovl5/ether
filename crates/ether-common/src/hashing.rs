use std::path::Path;

use itertools::Itertools;

use crate::{ExecutableArgs, GenericHash, Recipe, RecipeBuilder, RecipeInput,
            Res, cmp_res, get_file_name};

/// Builder for a hash.
pub trait HashBuilder: Sized {
  /// Recursively goes into a path and hashes its contents.
  fn with_path(&mut self, value : &Path) -> Res<&mut Self>;

  fn with_str(&mut self, value : &str) -> Res<&mut Self>;

  fn with_t<T : EtherHashable>(&mut self, value : &T) -> Res<&mut Self> {
    value.get_ether_hash(self)
  }

  fn build(&self) -> Res<GenericHash>;
}

fn len_prefix(x : impl Into<String>) -> String {
  let x : String = x.into();
  format!("{:?}:'{}'",
          x.len(),
          x.replace("\\", "\\\\").replace("'", "\\'"))
}

pub trait EtherHashable: Sized {
  fn get_ether_hash<'a, T : HashBuilder>(&self,
                                         builder : &'a mut T)
                                         -> Res<&'a mut T>;
}

impl HashBuilder for blake3::Hasher {
  fn with_path(&mut self, value : &Path) -> Res<&mut Self> {
    let tail = get_file_name(value)?;
    if !value.is_dir() {
      self.update(format!("<file name='{}'>", len_prefix(tail)).as_bytes())
          .update_mmap_rayon(value)?
          .update(b"</file>");
      return Ok(self);
    }
    self.update(format!("<dir name='{}'>", len_prefix(tail)).as_bytes());
    for entry in value.read_dir()?
                      .map(|x| x.map(|entry| entry.path()))
                      .sorted_by(cmp_res)
                      .collect::<std::io::Result<Vec<_>>>()?
    {
      self.with_path(&entry)?;
    }
    self.update(b"</dir>");

    Ok(self)
  }

  fn with_str(&mut self, value : &str) -> Res<&mut Self> {
    self.update(value.as_bytes());
    Ok(self)
  }

  fn build(&self) -> Res<GenericHash> {
    Ok(self.finalize().to_string().into())
  }
}

impl<X> EtherHashable for X where X : Into<String> + Clone,
{
  fn get_ether_hash<'a, T : HashBuilder>(&self,
                                         builder : &'a mut T)
                                         -> Res<&'a mut T> {
    let hash : String = self.clone().into();
    builder.with_str(&len_prefix(&hash))
  }
}

impl EtherHashable for ExecutableArgs {
  fn get_ether_hash<'a, T : HashBuilder>(&self,
                                         b : &'a mut T)
                                         -> Res<&'a mut T> {
    b.with_str("<executable-args>")?
     .with_str(&format!("<path to='{}'/>",
                        self.path
                            .as_ref()
                            .and_then(|x| x.to_str())
                            .map(len_prefix)
                            .unwrap_or_default()))?
     .with_str(&self.args
                    .iter()
                    .map(|x| format!("<arg>{}</arg>", len_prefix(x)))
                    .collect::<Vec<_>>()
                    .join(""))?
     .with_str(&self.env
                    .iter()
                    .sorted()
                    .map(|(k, v)| {
                      format!("<env key='{}'>{}</env>",
                              len_prefix(k),
                              len_prefix(v))
                    })
                    .collect::<Vec<_>>()
                    .join(""))?
     .with_str("</executable-args>")
  }
}

impl EtherHashable for RecipeBuilder {
  fn get_ether_hash<'a, T : HashBuilder>(&self,
                                         b : &'a mut T)
                                         -> Res<&'a mut T> {
    b.with_str("<recipe-builder>")?;
    use RecipeBuilder::*;
    match self {
      | FetchURL { url, expected_hash, } => {
        b.with_str(&format!("<fetch-url>{}</fetch-url>", len_prefix(url)))?
         .with_str("<out-hash>")?
         .with_t(expected_hash)?
         .with_str("</out-hash>")
      },
      | Executable { hash, args, } => b.with_str("<executable>")?
                                       .with_str("<hash>")?
                                       .with_t(hash)?
                                       .with_str("</hash>")?
                                       .with_t(args)?
                                       .with_str("</executable>"),
      | RecipeOutput { hash, output, args, } => {
        b.with_str("<recipe-output>")?
         .with_str(&format!("<recipe-hash output={:?}>",
                            len_prefix(output.clone())))?
         .with_t(hash)?
         .with_str("</hash>")?
         .with_t(args)?
         .with_str("</recipe-output")
      },
    }?.with_str("</recipe-builder>")
  }
}

impl EtherHashable for Recipe {
  fn get_ether_hash<'a, T : HashBuilder>(&self,
                                         b : &'a mut T)
                                         -> Res<&'a mut T> {
    b.with_str("<recipe>")?
     .with_str("<system>")?
     .with_str(&self.platform.map(|x| x.to_string()).unwrap_or_default())?
     .with_str("</system>")?
     .with_t(&self.builder)?
     .with_str(&self.outputs
                    .iter()
                    .sorted()
                    .map(|x| {
                      format!("<output>{:?}</output>", len_prefix(x.clone()))
                    })
                    .collect::<Vec<_>>()
                    .join(""))?
     .with_str(&self.inputs
                    .iter()
                    .sorted()
                    .map(|(k, v)| {
                      format!("<input key={:?}>{:?}</input>",
                              len_prefix(k.clone()),
                              match v {
                                | RecipeInput::Hash(h) =>
                                  String::from(h.clone()),
                                | RecipeInput::RecipeOutput { recipe_hash,
                                                              output, } =>
                                  format!("{:?}:{:?}", recipe_hash, output),
                              })
                    })
                    .collect::<Vec<_>>()
                    .join(""))?
     .with_str("</recipe>")
  }
}
