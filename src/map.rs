use base64;
use serde::Deserializer;
use serde::de::{Error, Visitor};
use std::fmt;

/// Container for Crocotile Map data
#[derive(Debug, Deserialize, Serialize)]
pub struct MapData {
    /// Configuration for the map.
    pub config: Config,
    /// All the models contained within the map
    pub model: Vec<Model>,
}

/// Map-wide configuration
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// The width of a tile from the texture map
    pub tilesize_x: u16,
    /// The height of a tile from the texture map
    pub tilesize_y: u16,
    /// Texture to use on the skybox
    #[serde(default)]
    #[serde(deserialize_with = "decode_base64_opt")]
    pub skybox: Option<Vec<u8>>,
}

/// A model, which is made from a texture map and a `Vec<Object>`
#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    /// The texture used on the model
    #[serde(deserialize_with = "decode_base64")]
    pub texture: Vec<u8>,
    /// The individual parts of the model
    pub object: Vec<Object>,
}

/// A distinct part of a model.
#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    /// Object-level translation
    pub position: Vertex,
    /// List of all vertices used to display this object
    pub vertices: Vec<Vertex>,
    /// Indexes of vertices grouped by triangle
    pub faces: Vec<Triangle>,
    /// Texture coordinates for each triangle
    pub uvs: Vec<UV>,
}

/// A vertex representing a point of an object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Vertex {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Z coordinate
    pub z: f32,
}

/// The three indexes into the vertex list used to draw a triangle.
pub type Triangle = [u8; 3];
/// The Texture coordinates to use for each vertex of a triangle.
pub type UV = [UVCoord; 3];

/// Texture coordinates
#[derive(Debug, Deserialize, Serialize)]
pub struct UVCoord {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
}

fn decode_base64_opt<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer
        .deserialize_bytes(Base64Visitor)
        .map(|data| Some(data))
}

fn decode_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_bytes(Base64Visitor)
}

struct Base64Visitor;

impl<'de> Visitor<'de> for Base64Visitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64 byte array")
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        base64::decode_config(&v[22..], base64::MIME).map_err(Error::custom)
    }
}
