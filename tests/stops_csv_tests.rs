extern crate osm_transit_extractor;
extern crate osmpbfreader;


#[test]
pub fn osm_fixture_stops_count() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osm_transit_extractor::get_stops_from_osm(&mut parsed_pbf);
    assert!(stops.len() == 64)
}

#[test]
pub fn osm_fixture_routes_count() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    assert!(routes.len() == 2);
    assert!(routes[0].ordered_stops_id.len() == 31);
    assert!(routes[0].ordered_stops_id[0] == "StopPoint:Node:3270784465");
    assert!(routes[0].ordered_stops_id[30] == "StopPoint:Node:1577028157");
}

#[test]
pub fn osm_fixture_lines_count() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    assert!(lines.len() == 1);
    assert!(lines[0].routes_id.len() == 2);
}

#[test]
pub fn osm_fixture_stops_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osm_transit_extractor::get_stops_from_osm(&mut parsed_pbf);
    osm_transit_extractor::write_stops_to_csv(stops);
}

#[test]
pub fn osm_fixture_routes_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    osm_transit_extractor::write_routes_to_csv(routes);
}

#[test]
pub fn osm_fixture_lines_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    osm_transit_extractor::write_lines_to_csv(lines);
}
