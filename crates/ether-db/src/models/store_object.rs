use ether_common::{FileLike, Recipe, Res, StoreObject, StoreObjectValue};
use toasty::{Db, Deferred, Embed, Json, Model};

use crate::utils::{DbID, DbStoreHash, DbStoreRefType, FromPrimitive,
                   IntoPrimitive};

/* Massive changes to how `DbStoreObject` will be shaped:
 *
 * - Use a tagged union instead of a plain struct
 *  - Variants: FilesystemObject; RecipeObject
 *  - Also change in primitives
 * - Research some way to avoid duplicating types
 */

#[derive(Debug, Model)]
pub struct DbStoreObject {
  #[key]
  #[auto]
  pub id : DbID,

  #[index]
  pub hash : DbStoreHash,

  pub value : DbStoreObjectValue,

  /// References where this object is on the *LEFT SIDE*
  #[has_many(pair = lhs)]
  pub lhs_refs : Deferred<Vec<DbStoreObjectRef>>,

  /// References where this object is on the *RIGHT SIDE*
  #[has_many(pair = rhs)]
  pub rhs_refs : Deferred<Vec<DbStoreObjectRef>>,

  /// Referenced objects that are "sources" (left-sides) from this object.
  #[has_many(via = rhs_refs.lhs)]
  pub lhs_objs : Deferred<Vec<DbStoreObject>>,

  /// Referenced objects that are "targets" (right-sides) to this object.
  #[has_many(via = lhs_refs.rhs)]
  pub rhs_objs : Deferred<Vec<DbStoreObject>>,
}

#[derive(Debug, Embed)]
pub enum DbStoreObjectValue {
  FileLike {
    #[column(type = text)]
    content : Json<FileLike>,
  },
  Recipe {
    #[column(type = text)]
    content : Json<Box<Recipe>>,
  },
}

impl IntoPrimitive for DbStoreObjectValue {
  type P = StoreObjectValue;

  async fn into_primitive(self, _db : &mut Db) -> Res<Self::P> {
    let out = match self {
      | Self::FileLike { content, } => StoreObjectValue::FileLike(content.0),
      | Self::Recipe { content, } => StoreObjectValue::Recipe(content.0),
    };
    Ok(out)
  }
}

impl FromPrimitive for DbStoreObjectValue {
  type P = StoreObjectValue;

  async fn from_primitive(_db : &mut Db, prim : Self::P) -> Res<Self> {
    let out = match prim {
      | StoreObjectValue::FileLike(v) => {
        Self::FileLike { content : Json::from(v) }
      },
      | StoreObjectValue::Recipe(v) => {
        Self::Recipe { content : Json::from(v) }
      },
    };

    Ok(out)
  }
}

#[derive(Debug, Model)]
#[key(lhs_id, rhs_id)]
pub struct DbStoreObjectRef {
  #[index]
  pub lhs_id : DbID,

  #[index]
  pub rhs_id : DbID,

  #[belongs_to(key = lhs_id, references = id)]
  pub lhs : Deferred<DbStoreObject>,

  #[belongs_to(key = rhs_id, references = id)]
  pub rhs : Deferred<DbStoreObject>,

  #[index]
  pub ref_type : DbStoreRefType,
}

impl IntoPrimitive for DbStoreObject {
  type P = StoreObject;
  async fn into_primitive(self, db : &mut Db) -> Res<Self::P> {
    Ok(Self::P { hash : self.hash.clone().into(),
                 value : self.value.into_primitive(db).await? })
  }
}

impl FromPrimitive for DbStoreObject {
  type P = StoreObject;
  async fn from_primitive(db : &mut Db, prim : Self::P) -> Res<Self> {
    let obj = Self::create();
    let obj_hash : DbStoreHash = prim.hash.into();

    Ok(obj.hash(obj_hash)
          .value(DbStoreObjectValue::from_primitive(db, prim.value).await?)
          .exec(db)
          .await?)
  }
}
