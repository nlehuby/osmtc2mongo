extern crate osmtc2mongo;
extern crate osmpbfreader;
use osmtc2mongo::*;


#[test]
pub fn osm_fixture_stops_count() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osmtc2mongo::get_stops_from_osm(&mut parsed_pbf);
    assert!(stops.len() == 64)
}

#[test]
pub fn osm_fixture_routes_count() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osmtc2mongo::get_routes_from_osm(&mut parsed_pbf);
    assert!(routes.len() == 2)
}

#[test]
pub fn osm_fixture_lines_count() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osmtc2mongo::get_lines_from_osm(&mut parsed_pbf);
    assert!(lines.len() == 1);
    assert!(lines[0].routes_id.len() == 2);
}

#[test]
pub fn osm_fixture_stops_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osmtc2mongo::get_stops_from_osm(&mut parsed_pbf);
    osmtc2mongo::write_stops_to_csv(stops);
}

#[test]
pub fn osm_fixture_routes_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osmtc2mongo::get_routes_from_osm(&mut parsed_pbf);
    osmtc2mongo::write_routes_to_csv(routes);
}

#[test]
pub fn osm_fixture_lines_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osmtc2mongo::get_lines_from_osm(&mut parsed_pbf);
    osmtc2mongo::write_lines_to_csv(lines);
}
