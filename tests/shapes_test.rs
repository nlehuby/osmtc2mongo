#[test]
pub fn osm_shapes_filter_way_with_only_one_existing_node() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/sample-lite.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    assert_eq!(lines[0].shape.len(), 1);
    assert_eq!(routes[0].shape.len(), 1);
}
