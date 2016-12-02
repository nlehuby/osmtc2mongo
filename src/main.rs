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

pub type OsmPbfReader = osmpbfreader::OsmPbfReader<std::fs::File>;

/*
use std::collections::{BTreeSet, BTreeMap};
use std::rc::Rc;
use std::cell::Cell;
*/

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

/*
#[derive(Debug, Clone)]
pub struct StopPoint {
    pub id: String,
    pub name: String,
    pub coord: Coord,
}

pub type StopPointsVec = Vec<StopPoint>;

fn get_stop_points(pbf: &mut OsmPbfReader) -> StopPointsVec {
    let matcher = PoiMatcher::new(poi_types);
    let objects = osmpbfreader::get_objs_and_deps(pbf, |o| matcher.is_poi(o)).unwrap();
    objects.iter()
        .filter(|&(_, obj)| matcher.is_poi(&obj))
        .map(|(_, obj)| parse_poi(obj, &objects, admins, city_level))
        .collect()
}
*/

fn is_stop_point(obj: &osmpbfreader::OsmObj) -> bool{
    match *obj {
        osmpbfreader::OsmObj::Relation(ref rel) => {
            rel.tags.get("public_transport").map_or(false, |v| v == "platform")
        }
        osmpbfreader::OsmObj::Node(ref node) => {
            node.tags.get("public_transport").map_or(false, |v| v == "platform") ||
            node.tags.get("highway").map_or(false, |v| v == "bus_stop")
        }
       _ => false,
    }
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut parsed_pbf = parse_osm_pbf(&args.flag_input);

    if args.flag_import_stop_points {
        let objects = osmpbfreader::get_objs_and_deps(&mut parsed_pbf, is_stop_point);

        let mut wtr = csv::Writer::from_memory();

        for (_, obj) in &objects {
            if !is_stop_point(&obj) {
                continue;
            }

            let result = wtr.encode(obj);
            assert!(result.is_ok());
/*
            let name = obj.tags
                .get("name")
                .and_then(|s| s.parse().ok());
            let coord = Coord::new(obj.lat, obj.lon);
            */
        }
        /*
        let mut poi_types = PoiTypes::new();
        poi_types.insert("amenity".to_string(), default_amenity_types());
        poi_types.insert("leisure".to_string(), default_leisure_types());

        info!("Extracting pois from osm");
        let pois = pois(&mut parsed_pbf, poi_types, &admins, city_level);

        info!("Importing pois into Mimir");
        let nb_pois = rubber.index("poi", &args.flag_dataset, pois.iter())
            .unwrap();

        info!("Nb of indexed pois: {}", nb_pois);
        */
    }
    println!("end of osmtc2mongo !")
}
