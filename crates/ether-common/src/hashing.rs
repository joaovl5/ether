use std::path::Path;

use itertools::Itertools;

use crate::{ExecutableArgs, GenericHash, Recipe, RecipeBuilder, Res, cmp_res,
            get_file_name};

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

pub trait EtherHashable: Sized {
  fn get_ether_hash<'a, T : HashBuilder>(&self,
                                         builder : &'a mut T)
                                         -> Res<&'a mut T>;
}

impl HashBuilder for blake3::Hasher {
  fn with_path(&mut self, value : &Path) -> Res<&mut Self> {
    let tail = get_file_name(value)?;
    if !value.is_dir() {
      self.update(format!("<file name='{}'>", tail).as_bytes())
          .update_mmap_rayon(value)?
          .update(b"</file>");
      return Ok(self);
    }
    self.update(format!("<dir name='{}'>", tail).as_bytes());
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
    builder.with_str(&hash)
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
                            .unwrap_or("")))?
     .with_str(&self.args
                    .iter()
                    .map(|x| format!("<arg>{}</arg>", x))
                    .collect::<Vec<_>>()
                    .join(""))?
     .with_str(&self.env
                    .iter()
                    .map(|(k, v)| format!("<env key='{}'>{}</env>", k, v))
                    .collect::<Vec<_>>()
                    .join(""))?
     .with_str("</executable-args>")
  }
}

impl EtherHashable for RecipeBuilder {
  fn get_ether_hash<'a, T : HashBuilder>(&self,
                                         b : &'a mut T)
                                         -> Res<&'a mut T> {
    use RecipeBuilder::*;

    match self {
      | FetchURL { url, expected_hash, } => {
        b.with_str(&format!("<fetch-url>{}</fetch-url>", url))?
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
         .with_str(&format!("<recipe-hash output={:?}>", output))?
         .with_t(hash)?
         .with_str("</hash>")?
         .with_t(args)?
         .with_str("</recipe-output")
      },
    }
  }
}

impl EtherHashable for Recipe {
  fn get_ether_hash<'a, T : HashBuilder>(&self,
                                         b : &'a mut T)
                                         -> Res<&'a mut T> {
    b.with_str("<recipe>")?
     .with_str("<system>")?
     .with_str(self.system.map(|x| Into::<&str>::into(&x)).unwrap_or(""))?
     .with_str("</system>")?
     .with_str(&self.outputs
                    .iter()
                    .map(|x| format!("<output>{:?}</output>", x))
                    .collect::<Vec<_>>()
                    .join(""))?
     .with_str(&self.inputs
                    .iter()
                    .map(|(k, v)| format!("<input key={:?}>{:?}</input>", k, v))
                    .collect::<Vec<_>>()
                    .join(""))?
     .with_str("</recipe>")
  }
}
