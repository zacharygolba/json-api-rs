use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::mem;

use doc::{Data, Document, Identifier, Link, PrimaryData, Relationship};
use error::Error;
use query::Query;
use sealed::Sealed;
use value::{Key, Map, Set, Value};
use view::Render;

/// A preexisting resource. Commonly found in the document of a response or `PATCH`
/// request.
///
/// Both the [`id`] and [`type`][`kind`] field must be present if an `Object` is
/// deserialized. If you need to represent a resource object that does not already have
/// an [`id`], you can use [`NewObject`]. For more information, check out the *[resource
/// objects]* section of the JSON API specification.
///
/// # Equality
///
/// Objects are considered to be equal if they have the same [`id`] and [`kind`].
///
/// ```
/// # extern crate json_api;
/// #
/// # use json_api::Error;
/// #
/// # fn example() -> Result<(), Error> {
/// use json_api::doc::Object;
/// use json_api::value::Key;
///
/// let person = "person".parse::<Key>()?;
/// let hero = "hero".parse::<Key>()?;
///
/// let mut bruce = Object::new(person.clone(), "🦇".to_owned());
/// let mut batman = Object::new(person.clone(), "🦇".to_owned());
///
/// bruce.attributes.insert("name".parse()?, "Bruce Wayne".into());
/// batman.attributes.insert("name".parse()?, "Batman".into());
///
/// // Bruce and Batman are equal because they have the same `id` and `kind`.
/// assert!(bruce == batman);
///
/// // Let's make Batman a "hero" instead of a "person".
/// batman.kind = hero.clone();
///
/// // Bruce and Batman are no longer equal.
/// assert!(bruce != batman);
/// #
/// # Ok(())
/// # }
/// #
/// # fn main() {
/// # example().unwrap();
/// # }
/// ```
///
/// Since an [`Identifier`] is a subset of `Object` with fields necessary for
/// identification, you can compare the two.
///
/// ```
/// # extern crate json_api;
/// #
/// # use json_api::Error;
/// #
/// # fn example() -> Result<(), Error> {
/// # use json_api::doc::Object;
/// # use json_api::value::Key;
/// #
/// # let hero = "hero".parse::<Key>()?;
/// # let batman = Object::new(hero.clone(), "🦇".to_owned());
/// #
/// use json_api::doc::Identifier;
/// assert!(Identifier::from(&batman) == batman);
/// #
/// # Ok(())
/// # }
/// #
/// # fn main() {
/// # example().unwrap();
/// # }
/// ```
///
/// # Hashing
///
/// Similar to how equality works, object's are hashed by their [`id`] and [`kind`]. This
/// allows for easy and efficient deduplication when embedding related resources in a
/// response.
///
/// **Note:** The following example is to demonstrate how object's are hashed.
/// Deduplication occurs automatically if you use the [`json_api::to_doc`] function with
/// a [`Resource`] that was implemented with the [`resource!`] macro.
///
/// ```
/// # extern crate json_api;
/// #
/// # use json_api::Error;
/// #
/// # fn example() -> Result<(), Error> {
/// use json_api::doc::Object;
/// use json_api::value::{Key, Set};
///
/// let person = "person".parse::<Key>()?;
/// let hero = "hero".parse::<Key>()?;
///
/// let mut included = Set::new();
///
/// let mut diana = Object::new(person.clone(), "🛡".to_owned());
/// let mut wonder_woman = Object::new(person.clone(), "🛡".to_owned());
///
/// diana.attributes.insert("name".parse()?, "Diana Prince".into());
/// wonder_woman.attributes.insert("name".parse()?, "Wonder Woman".into());
///
/// included.insert(diana);
/// assert_eq!(included.len(), 1);
///
/// included.insert(wonder_woman.clone());
/// assert_eq!(included.len(), 1);
///
/// // Let's update Wonder Woman's kind to "hero" so we can embed her in the response.
/// wonder_woman.kind = hero.clone();
///
/// included.insert(wonder_woman.clone());
/// assert_eq!(included.len(), 2);
/// #
/// # Ok(())
/// # }
/// #
/// # fn main() {
/// # example().unwrap();
/// # }
/// ```
///
/// [`Identifier`]: ./struct.Identifier.html
/// [`NewObject`]: ./struct.NewObject.html
/// [`Resource`]: ../trait.Resource.html
/// [`resource!`]: ../macro.resource.html
/// [`json_api::to_doc`]: ../fn.to_doc.html
/// [`id`]: #structfield.id
/// [`kind`]: #structfield.kind
/// [resource objects]: https://goo.gl/55cSP7
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Object {
    /// Contains some of the object's data. If this value of this field is empty, it will
    /// not be serialized. For more information, check out the *[attributes]* section of
    /// the JSON API specification.
    ///
    /// [attributes]: https://goo.gl/TshgH1
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub attributes: Map,

    /// A string that contains a unique identfier for this resource type (`kind`). For
    /// more information, check out the *[identification]* section of the JSON API
    /// specification.
    ///
    /// [identification]: https://goo.gl/3s681i
    pub id: String,

    /// Describes resources that share common attributes and relationships. This field is
    /// derived from the `type` field if the object is deserialized. For more
    /// information, check out the *[identification]* section of the JSON API
    /// specification.
    ///
    /// [identification]: https://goo.gl/3s681i
    #[serde(rename = "type")]
    pub kind: Key,

    /// Contains relevant links. If this value of this field is empty, it will not be
    /// serialized. For more information, check out the *[links]* section of the JSON
    /// API specification.
    ///
    /// [links]: https://goo.gl/E4E6Vt
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Key, Link>,

    /// Non-standard meta information. If this value of this field is empty, it will not
    /// be serialized. For more information, check out the *[meta information]* section
    /// of the JSON API specification.
    ///
    /// [meta information]: https://goo.gl/LyrGF8
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,

    /// Describes relationships between this object and other resource objects. If this
    /// value of this field is empty, it will not be serialized. For more information,
    /// check out the *[relationships]* section of the JSON API specification.
    ///
    /// [relationships]: https://goo.gl/Gxghwc
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub relationships: Map<Key, Relationship>,

    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl Object {
    /// Returns a new `Object`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// use json_api::doc::Object;
    /// let mut obj = Object::new("users".parse()?, "1".to_owned());
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// # example().unwrap();
    /// # }
    /// ```
    pub fn new(kind: Key, id: String) -> Self {
        Object {
            id,
            kind,
            attributes: Default::default(),
            links: Default::default(),
            meta: Default::default(),
            relationships: Default::default(),
            _ext: (),
        }
    }
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.kind.hash(state);
    }
}

impl PartialEq for Object {
    fn eq(&self, rhs: &Object) -> bool {
        self.id == rhs.id && self.kind == rhs.kind
    }
}

impl PartialEq<Identifier> for Object {
    fn eq(&self, rhs: &Identifier) -> bool {
        self.id == rhs.id && self.kind == rhs.kind
    }
}

impl Render<Identifier> for Object {
    fn render(self, query: Option<&Query>) -> Result<Document<Identifier>, Error> {
        Identifier::from(self).render(query)
    }
}

impl Render<Identifier> for Vec<Object> {
    fn render(self, _: Option<&Query>) -> Result<Document<Identifier>, Error> {
        let data = self.into_iter().map(Identifier::from).collect();

        Ok(Document::Ok {
            data,
            included: Default::default(),
            jsonapi: Default::default(),
            links: Default::default(),
            meta: Default::default(),
        })
    }
}

impl Render<Object> for Object {
    fn render(mut self, _: Option<&Query>) -> Result<Document<Object>, Error> {
        let links = mem::replace(&mut self.links, Default::default());
        let meta = mem::replace(&mut self.meta, Default::default());

        Ok(Document::Ok {
            links,
            meta,
            data: Data::Member(Box::new(Some(self))),
            included: Default::default(),
            jsonapi: Default::default(),
        })
    }
}

impl Render<Object> for Vec<Object> {
    fn render(self, _: Option<&Query>) -> Result<Document<Object>, Error> {
        Ok(Document::Ok {
            data: Data::Collection(self),
            included: Default::default(),
            jsonapi: Default::default(),
            links: Default::default(),
            meta: Default::default(),
        })
    }
}

impl PrimaryData for Object {
    fn flatten(self, incl: &Set<Object>) -> Value {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let Object { id, attributes, relationships, .. } = self;
        let mut map = {
            let size = attributes.len() + relationships.len() + 1;
            Map::with_capacity(size)
        };

        map.insert(Key::from_raw("id".to_owned()), Value::String(id));
        map.extend(attributes);

        for (key, value) in relationships {
            let value = match value.data {
                Data::Member(data) => match *data {
                    Some(item) => item.flatten(incl),
                    None => Value::Null,
                },
                Data::Collection(data) => {
                    let iter = data.into_iter().map(|item| item.flatten(incl));
                    Value::Array(iter.collect())
                }
            };

            map.insert(key, value);
        }

        Value::Object(map)
    }
}

impl Sealed for Object {}

/// A resource that does not already exist. Commonly found in the document of a
/// `POST` request.
///
/// For more information, check out the *[creating resources]* section of the JSON API
/// specification.
///
/// [creating resources]: https://goo.gl/KoLQgh
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewObject {
    /// Contains some of the object's data. If this value of this field is empty, it will
    /// not be serialized. For more information, check out the *[attributes]* section of
    /// the JSON API specification.
    ///
    /// [attributes]: https://goo.gl/TshgH1
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub attributes: Map,

    /// An optional string that contains a unique identfier for this resource type
    /// (`kind`). A `Some` value here should be interpreted as *[client-generated id]*.
    /// For more information, check out the *[identification]* section of
    /// the JSON API specification.
    ///
    /// [client-generated id]: https://goo.gl/W16smj
    /// [identification]: https://goo.gl/3s681i
    pub id: Option<String>,

    /// Describes resources that share common attributes and relationships. This field
    /// is derived from the `type` field if the object is deserialized. For more
    /// information, check out the *[identification]* section of the JSON API
    /// specification.
    ///
    /// [identification]: https://goo.gl/3s681i
    #[serde(rename = "type")]
    pub kind: Key,

    /// Contains relevant links. If this value of this field is empty, it will not be
    /// serialized. For more information, check out the *[links]* section of the JSON
    /// API specification.
    ///
    /// [links]: https://goo.gl/E4E6Vt
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<Key, Link>,

    /// Non-standard meta information. If this value of this field is empty, it will not
    /// be serialized. For more information, check out the *[meta information]* section
    /// of the JSON API specification.
    ///
    /// [meta information]: https://goo.gl/LyrGF8
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,

    /// Describes relationships between this object and other resource objects. If this
    /// value of this field is empty, it will not be serialized. For more information,
    /// check out the *[relationships]* section of the JSON API specification.
    ///
    /// [relationships]: https://goo.gl/Gxghwc
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub relationships: Map<Key, Relationship>,

    /// Private field for backwards compatibility.
    #[serde(skip)]
    _ext: (),
}

impl NewObject {
    /// Returns a new `NewObject`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # use json_api::Error;
    /// #
    /// # fn example() -> Result<(), Error> {
    /// use json_api::doc::NewObject;
    /// let mut obj = NewObject::new("users".parse()?);
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// # example().unwrap();
    /// # }
    /// ```
    pub fn new(kind: Key) -> Self {
        NewObject {
            kind,
            id: Default::default(),
            attributes: Default::default(),
            links: Default::default(),
            meta: Default::default(),
            relationships: Default::default(),
            _ext: (),
        }
    }
}

impl PrimaryData for NewObject {
    fn flatten(self, _: &Set<Object>) -> Value {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let NewObject { id, attributes, relationships, .. } = self;
        let mut map = {
            let size = attributes.len() + relationships.len() + 1;
            Map::with_capacity(size)
        };

        if let Some(value) = id {
            map.insert(Key::from_raw("id".to_owned()), Value::String(value));
        }

        map.extend(attributes);

        for (key, value) in relationships {
            let value = match value.data {
                Data::Member(data) => match *data {
                    Some(Identifier { id, .. }) => Value::String(id),
                    None => Value::Null,
                },
                Data::Collection(data) => data.into_iter().map(|ident| ident.id).collect(),
            };

            map.insert(key, value);
        }

        Value::Object(map)
    }
}

impl Render<NewObject> for NewObject {
    fn render(self, _: Option<&Query>) -> Result<Document<NewObject>, Error> {
        Ok(Document::Ok {
            data: Data::Member(Box::new(Some(self))),
            included: Default::default(),
            jsonapi: Default::default(),
            links: Default::default(),
            meta: Default::default(),
        })
    }
}

impl Sealed for NewObject {}
