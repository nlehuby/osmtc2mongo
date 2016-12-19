extern crate osmtc2mongo;
extern crate osmpbfreader;
use osmtc2mongo::*;


#[test]
pub fn osm_fixture_stops_count(){
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops : Vec<Box<StopPoint>> = osmtc2mongo::get_stops_from_osm(&mut parsed_pbf);
    assert!(stops.len() == 64)
}

#[test]
pub fn osm_fixture_routes_count(){
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops : Vec<Box<Route>> = osmtc2mongo::get_routes_from_osm(&mut parsed_pbf);
    assert!(stops.len() == 3)
}

#[test]
pub fn osm_fixture_stops_csv(){
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops : Vec<Box<StopPoint>> = osmtc2mongo::get_stops_from_osm(&mut parsed_pbf);
    osmtc2mongo::write_stops_to_csv(stops);
}
