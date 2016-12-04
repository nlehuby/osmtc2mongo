// Copyright © 2016, Canal TP and/or its affiliates. All rights reserved.
//
// This file is part of Navitia,
//     the software to build cool stuff with public transport.
//
// Hope you'll enjoy and contribute to this project,
//     powered by Canal TP (www.canaltp.fr).
// Help us simplify mobility and open public transport:
//     a non ending quest to the responsive locomotion way of traveling!
//
// LICENCE: This program is free software; you can redistribute it
// and/or modify it under the terms of the GNU Affero General Public
// License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this program. If not, see
// <http://www.gnu.org/licenses/>.
//
// Stay tuned using
// twitter @navitia
// IRC #navitia on freenode
// https://groups.google.com/d/forum/navitia
// www.navitia.io

#[macro_use]
extern crate osmpbfreader;
extern crate docopt;
extern crate rustc_serialize;
extern crate csv;
use std::collections::BTreeMap;

pub type OsmPbfReader = osmpbfreader::OsmPbfReader<std::fs::File>;

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_input: String,
    flag_connection_string: String,
    flag_import_stop_points: bool,
}

static USAGE: &'static str = "
Usage:
    osmtc2mongo --help
    osmtc2mongo --input=<file> [--connection-string=<connection-string>] [--import-stop-points]

Options:
    -h, --help                  Show this message.
    -i, --input=<file>          OSM PBF file.
    -s, --import-stop-points    Import stop_points
    -c, --connection-string=<connection-string>
                                Mongo parameters, [default: http://localhost:9200/osmtc]
";

fn parse_osm_pbf(path: &str) -> OsmPbfReader {
    let path = std::path::Path::new(&path);
    osmpbfreader::OsmPbfReader::new(std::fs::File::open(&path).unwrap())
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
pub struct Coord {
    lat: f64,
    lon: f64,
}
impl Coord {
    fn new(lat_param: f64, lon_param: f64) -> Coord {
        Coord { lat: lat_param, lon: lon_param }
    }
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
pub struct StopPoint {
    pub id: String,
    pub coord: Coord,
    pub name: String,
}


fn is_stop_point(obj: &osmpbfreader::OsmObj) -> bool{
    match *obj {
        osmpbfreader::OsmObj::Relation(ref rel) => {
            rel.tags.get("public_transport").map_or(false, |v| v == "platform")
        },
        osmpbfreader::OsmObj::Way(ref w) => {
            w.tags.get("public_transport").map_or(false, |v| v == "platform")
        },
        osmpbfreader::OsmObj::Node(ref node) => {
            node.tags.get("public_transport").map_or(false, |v| v == "platform") ||
            node.tags.get("highway").map_or(false, |v| v == "bus_stop")
        },
    }
}

fn get_way_coord(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                 way: &osmpbfreader::objects::Way)
                 -> Coord {
    //Coord::new(0., 0.)
    way.nodes
        .iter()
        .filter_map(|node_id| {
            obj_map.get(&osmpbfreader::OsmId::Node(*node_id))
                .and_then(|obj| obj.node())
                .map(|node| Coord::new(node.lat, node.lon))
        })
        .next()
        .unwrap_or(Coord::new(0., 0.))
}

fn get_rel_coord(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                 rel: &osmpbfreader::objects::Relation)
                 -> Coord {
    //Coord::new(0., 0.)
    rel.refs
        .iter()
        .filter_map(|refe| {
            obj_map.get(&refe.member).or_else(|| {
                None
            })
        })
        .filter_map(|osm_obj| {
            if let &osmpbfreader::OsmObj::Way(ref way) = osm_obj {
                Some(get_way_coord(obj_map, way))
            } else if let &osmpbfreader::OsmObj::Node(ref node) = osm_obj {
                Some(Coord::new(node.lat, node.lon))
            } else {
                None
            }
        })
        .next()
        .unwrap_or(Coord::new(0., 0.))
}

fn osm_obj_2_stop_point(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                        obj: &osmpbfreader::OsmObj)
                        -> StopPoint {
    match *obj {
        osmpbfreader::OsmObj::Relation(ref rel) => {
            let mut sp_id : String = "StopPoint:Relation:".to_string();
            sp_id.push_str(&rel.id.to_string());
            StopPoint{id: sp_id,
                      name: rel.tags.get("name").unwrap_or(&"".to_string()).to_string(),
                      coord: get_rel_coord(obj_map, rel) }
        }
        osmpbfreader::OsmObj::Way(ref way) => {
            let mut sp_id : String = "StopPoint:Way:".to_string();
            sp_id.push_str(&way.id.to_string());
            StopPoint{id: sp_id.to_string(),
                      name: way.tags.get("name").unwrap_or(&"".to_string()).to_string(),
                      coord: get_way_coord(obj_map, way) }
        }
        osmpbfreader::OsmObj::Node(ref node) => {
            let mut sp_id : String = "StopPoint:Node:".to_string();
            sp_id.push_str(&node.id.to_string());
            StopPoint{id: sp_id.to_string(),
                      name: node.tags.get("name").unwrap_or(&"".to_string()).to_string(),
                      coord: Coord{lat: node.lat, lon: node.lon} }
        }
    }
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut parsed_pbf = parse_osm_pbf(&args.flag_input);

    if args.flag_import_stop_points {
        let objects = osmpbfreader::get_objs_and_deps(&mut parsed_pbf, is_stop_point).unwrap();

        let csv_file = std::path::Path::new("/tmp/osmtc2mongo.csv");
        let mut wtr = csv::Writer::from_file(csv_file).unwrap();

        for (_, obj) in &objects {
            if !is_stop_point(&obj) {
                continue;
            }
            let sp = osm_obj_2_stop_point(&objects, obj);
            let result = wtr.encode(sp);
            assert!(result.is_ok());
        }
    }
    println!("end of osmtc2mongo !")
}
