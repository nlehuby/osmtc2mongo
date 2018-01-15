extern crate osm_transit_extractor;
extern crate osmpbfreader;
extern crate tempdir;
use tempdir::TempDir;


#[test]
pub fn osm_fixture_stoppoints_count() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osm_transit_extractor::get_stop_points_from_osm(&mut parsed_pbf);
    assert!(stops.len() == 64)
}

#[test]
pub fn osm_fixture_routes_count() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    assert!(routes.len() == 3);
    for r in routes {
        if r.id == "Route:Relation:1257168" {
            assert!(r.ordered_stops_id.len() == 34);
            assert!(r.ordered_stops_id[0] == "StopPoint:Node:3270784465");
            assert!(r.ordered_stops_id[30] == "StopPoint:Node:1577028157");
        }
    }
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
pub fn osm_fixture_routes_tags() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    for r in routes {
        match r.id.as_ref() {
            "Route:Relation:1257168" => {
                assert!(r.colour == format!(""));
                assert!(r.operator == format!("RATP"));
                assert!(r.network == format!("RATP"));
                assert!(r.mode == format!("bus"));
                assert!(r.code == format!("57"));
                assert!(r.origin == format!("Arcueil - Laplace"));
            },
            "Route:Relation:1257174" => {
                assert!(r.destination == format!("Arcueil - Laplace"));
            },
            _ => {},
        }
    }
}

#[test]
pub fn osm_fixture_lines_tags() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    assert!(lines[0].colour == format!("#9C983A"));
    assert!(lines[0].operator == format!("RATP"));
    assert!(lines[0].network == format!("RATP"));
    assert!(lines[0].mode == format!("bus"));
    assert!(lines[0].code == format!("57"));
}

#[test]
pub fn osm_fixture_stoppoints_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let stops = osm_transit_extractor::get_stop_points_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_stop_points_to_csv(&stops, &vec![], &tmp_dir);
    tmp_dir.close().expect("delete temp dir");
}

#[test]
pub fn osm_fixture_routes_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let routes = osm_transit_extractor::get_routes_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_routes_to_csv(routes, &tmp_dir);
    tmp_dir.close().expect("delete temp dir");
}

#[test]
pub fn osm_fixture_lines_csv() {
    let osm_path = std::env::current_dir().unwrap().join("tests/fixtures/osm_fixture.osm.pbf");
    let mut parsed_pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(&osm_path).unwrap());
    let lines = osm_transit_extractor::get_lines_from_osm(&mut parsed_pbf);
    let tmp_dir = TempDir::new("osm_transit_extractor").expect("create temp dir");
    osm_transit_extractor::write_lines_to_csv(lines, &tmp_dir);
    tmp_dir.close().expect("delete temp dir");
}
