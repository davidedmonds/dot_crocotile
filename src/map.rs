use base64;
use serde::Deserializer;
use serde::de::{Error, Visitor};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct MapData {
    pub config: Config,
    pub model: Vec<Model>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub tilesize_x: u16,
    pub tilesize_y: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    #[serde(deserialize_with = "decode_base64")] pub texture: Vec<u8>,
    pub object: Vec<Object>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    pub position: Vertex,
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Triangle>,
    pub uvs: Vec<UV>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

type Triangle = [u8; 3];
type UV = [UVCoord; 3];

#[derive(Debug, Deserialize, Serialize)]
pub struct UVCoord {
    pub x: f32,
    pub y: f32,
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
