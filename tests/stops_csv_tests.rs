use tempdir::TempDir;

#[test]
pub fn osm_fixture_stoppoints() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osm_transit_extractor::get_stop_points_from_osm(&mut parsed_pbf);
    assert_eq!(stops[0].id, "node:260743996");
    assert_eq!(stops.len(), 64);
}

#[test]
pub fn osm_fixture_routes_count() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    assert_eq!(routes.len(), 3);
    for r in routes {
        if r.id == "relation:1257168" {
            assert_eq!(r.ordered_stops_id.len(), 34);
            assert_eq!(r.ordered_stops_id[0], "node:3270784465");
            assert_eq!(r.ordered_stops_id[30], "node:1577028157");
        }
    }
}

#[test]
pub fn osm_fixture_lines_count() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].routes_id.len(), 2);
}

#[test]
pub fn osm_fixture_routes_tags() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    for r in routes {
        match r.id.as_ref() {
            "relation:1257168" => {
                assert_eq!(r.colour, format!(""));
                assert_eq!(r.operator, format!("RATP"));
                assert_eq!(r.network, format!("RATP"));
                assert_eq!(r.mode, format!("bus"));
                assert_eq!(r.code, format!("57"));
                assert_eq!(r.origin, format!("Arcueil - Laplace"));
            }
            "relation:1257174" => {
                assert_eq!(r.destination, format!("Arcueil - Laplace"));
            }
            _ => {}
        }
    }
}

#[test]
pub fn osm_fixture_lines_tags() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    assert_eq!(lines[0].colour, format!("#9C983A"));
    assert_eq!(lines[0].operator, format!("RATP"));
    assert_eq!(lines[0].network, format!("RATP"));
    assert_eq!(lines[0].mode, format!("bus"));
    assert_eq!(lines[0].code, format!("57"));
}

#[test]
pub fn osm_fixture_stoppoints_csv() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osm_transit_extractor::get_stop_points_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_stop_points_to_csv(&stops, &vec![], &tmp_dir, false);
    tmp_dir.close().expect("delete temp dir");
}

#[test]
pub fn osm_fixture_routes_csv() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_routes_to_csv(routes, &tmp_dir, true);
    tmp_dir.close().expect("delete temp dir");
}

#[test]
pub fn osm_fixture_lines_csv() {
    let osm_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_lines_to_csv(lines, &tmp_dir, false);
    tmp_dir.close().expect("delete temp dir");
}
