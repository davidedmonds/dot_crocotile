extern crate base64;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod map;

pub use map::MapData;

use failure::Error;
use std::fs::File;

pub fn load(filename: &str) -> Result<MapData, Error> {
    let f = File::open(filename)?;
    let map: MapData = serde_json::from_reader(f)?;
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn trying_to_load_a_missing_file_fails() {
        let error: Error =
            load("src/resources/file_not_found.crocotile").expect_err("Should have IO error here");
        let io_error: &std::io::Error = error.downcast_ref().expect("Incorrect error type");
        assert_eq!(ErrorKind::NotFound, io_error.kind());
    }

    #[test]
    fn trying_to_load_an_unparseable_file_fails() {
        let error: Error =
            load("src/resources/unparseable.crocotile").expect_err("Should have Parse error here");
        let serde_error: &serde_json::Error = error.downcast_ref().expect("Incorrect error type");
        assert!(serde_error.is_syntax());
    }

    #[test]
    fn trying_to_parse_a_valid_file_succeeds() {
        let actual = load("src/resources/swamp.crocotile").expect("Should successfully load");
        assert_eq!(16, actual.config.tilesize_x);
        assert_eq!(1, actual.model.len());

        let model = &actual.model[0];
        let expected_image = include_bytes!("resources/swamp.png").to_vec();
        assert_eq!(expected_image, model.texture);
        assert_eq!(507, model.object.len());

        let object = &model.object[0];
        assert_eq!(2.5, object.position.x);
        assert_eq!(0.0, object.position.y);
        assert_eq!(0.625, object.position.z);
        assert_eq!(4, object.vertices.len());
        assert_eq!(2, object.faces.len());
        assert_eq!(2, object.uvs.len());

        let vertice = &object.vertices[0];
        assert_eq!(0.50000000249975, vertice.x);
        assert_eq!(-0.000049995000154012814, vertice.y);
        assert_eq!(0.875, vertice.z);

        let face = &object.faces[0];
        assert_eq!(0, face[0]);
        assert_eq!(2, face[1]);
        assert_eq!(1, face[2]);

        let uv = &object.uvs[0];
        assert_eq!(0.0, uv[0].x);
        assert_eq!(0.0625, uv[0].y);
        assert_eq!(0.0625, uv[1].x);
        assert_eq!(0.125, uv[1].y);
        assert_eq!(0.0, uv[2].x);
        assert_eq!(0.125, uv[2].y);
    }
}
