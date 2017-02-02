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

extern crate osmpbfreader;
extern crate rustc_serialize;
extern crate csv;
extern crate wkt;
use std::collections::BTreeMap;
use osmpbfreader::OsmObj::*;
use rustc_serialize::Encodable;
use rustc_serialize::Encoder;

pub type OsmPbfReader = osmpbfreader::OsmPbfReader<std::fs::File>;
pub type WktLineString = wkt::types::LineString;


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

#[derive(Debug, Clone)]
pub struct Route {
    pub id: String,
    pub name: String,
    pub code: String,
    pub shape: Vec<Vec<Coord>>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub id: String,
    pub name: String,
    pub code: String,
    pub shape: Vec<Vec<Coord>>,
    pub routes_id: Vec<String>,
}

#[allow(dead_code)]
/* to_multilinestring is to be used when issue #8 is resolved*/
impl Route {
    fn to_multilinestring(&self) -> wkt::types::MultiLineString {
        let wkt_linestrings = self.shape.iter()
            .map(|vec_coord|
                vec_coord.iter()
                    .map(|coord| wkt::types::Coord {x: coord.lon, y: coord.lat, z: None, m: None})
                    .collect()
            )
            .map(|wkt_coords| wkt::types::LineString(wkt_coords))
            .collect();
        wkt::types::MultiLineString(wkt_linestrings)
    }
}

impl Encodable for Route {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Route", 4, |s| {
            try!(s.emit_struct_field("id", 0, |s| {
                s.emit_str(&self.id)
            }));
            try!(s.emit_struct_field("name", 1, |s| {
                s.emit_str(&self.name)
            }));
            try!(s.emit_struct_field("code", 2, |s| {
                s.emit_str(&self.code)
            }));
            try!(s.emit_struct_field("shape", 3, |s| {
                if self.shape.len() == 0 {
                    s.emit_str(&"")
                } else {
                    let linestring : String = self.shape.iter()
                        .map(|vec_coord| vec_coord.iter()
                            .map(|coord| format!("{} {}", coord.lon.to_string(), coord.lat.to_string()))
                            .collect::<Vec<String>>()
                            .join(", ")
                        )
                        .collect::<Vec<String>>()
                        .join("), (");
                    s.emit_str(&format!("MULTILINESTRING(({}))", linestring))
                }
            }));
            Ok(())
        })
    }
}

pub fn parse_osm_pbf(path: &str) -> OsmPbfReader {
    let path = std::path::Path::new(&path);
    osmpbfreader::OsmPbfReader::new(std::fs::File::open(&path).unwrap())
}

fn is_stop_point(obj: &osmpbfreader::OsmObj) -> bool {
    obj.tags().get("public_transport").map_or(false, |v| v == "platform") ||
    obj.node().and_then(|n| n.tags.get("highway")).map_or(false, |v| v == "bus_stop")
}

fn is_line(obj: &osmpbfreader::OsmObj) -> bool {
    obj.relation()
        .and_then(|r| r.tags.get("type"))
        .map_or(false, |v| v == "route_master")
}

fn is_route(obj: &osmpbfreader::OsmObj) -> bool {
    obj.relation()
        .and_then(|r| r.tags.get("type"))
        .map_or(false, |v| v == "route")
}

fn get_one_coord_from_way(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                 way: &osmpbfreader::objects::Way)
                 -> Coord {
    way.nodes
        .iter()
        .filter_map(|node_id| {
            obj_map.get(&(*node_id).into())
                .and_then(|obj| obj.node())
                .map(|node| Coord::new(node.lat(), node.lon()))
        })
        .next()
        .unwrap_or(Coord::new(0., 0.))
}

fn get_one_coord_from_rel(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                 rel: &osmpbfreader::objects::Relation)
                 -> Coord {
    rel.refs
        .iter()
        .filter_map(|refe| obj_map.get(&refe.member))
        .filter_map(|osm_obj| {
            match *osm_obj {
                Way(ref way) => Some(get_one_coord_from_way(obj_map, way)),
                Node(ref node) => Some(Coord::new(node.lat(), node.lon())),
                Relation(..) => None,
            }
        })
        .next()
        .unwrap_or(Coord::new(0., 0.))
}

fn osm_way_to_vec(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                  osm_way: &osmpbfreader::Way) -> Vec<Coord> {
    osm_way.nodes.iter()
        .filter_map(|id| obj_map.get(&osmpbfreader::OsmId::Node(*id)))
        .filter_map(|osm_obj| osmpbfreader::OsmObj::node(osm_obj))
        .map(|node| Coord::new(node.lat(), node.lon()))
        .collect()
}

fn osm_route_to_shape(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                      osm_relation: &osmpbfreader::Relation) -> Vec<Vec<Coord>> {
    osm_relation.refs
        .iter()
        .filter_map(|refe| obj_map.get(&refe.member))
        .filter_map(|osm_obj| osmpbfreader::OsmObj::way(osm_obj))
        .filter_map(|osm_way| Some(osm_way_to_vec(obj_map, osm_way)))
        .collect()
}

fn osm_obj_to_route(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                    obj: &osmpbfreader::OsmObj) -> Option<Route> {
    match *obj {
        Relation(ref rel)=> {
            Some(Route { id: format!("Route:Relation:{}", rel.id.0),
                         name: rel.tags.get("name").cloned().unwrap_or("".to_string()),
                         code: rel.tags.get("ref").cloned().unwrap_or("".to_string()),
                         shape: osm_route_to_shape(obj_map, rel)
                })
        },
        _ => None
    }
}

fn osm_obj_to_line(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                    obj: &osmpbfreader::OsmObj) -> Option<Line> {
    match *obj {
        Relation(ref rel)=> {
            Some(Line { id: format!("Line:Relation:{}", rel.id.0),
                         name: rel.tags.get("name").cloned().unwrap_or("".to_string()),
                         code: rel.tags.get("ref").cloned().unwrap_or("".to_string()),
                         shape: osm_route_to_shape(obj_map, rel),
                         routes_id : vec![] //TODO
                })
        },
        _ => None
    }
}


fn osm_obj_to_stop_point(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                        obj: &osmpbfreader::OsmObj)
                        -> StopPoint {
    let (obj_type, obj_id, coord) = match *obj {
        Relation(ref rel) => ("Relation", rel.id.0, get_one_coord_from_rel(obj_map, rel)),
        Way(ref way) => ("Way", way.id.0, get_one_coord_from_way(obj_map, way)),
        Node(ref node) => ("Node", node.id.0, Coord { lat: node.lat(), lon: node.lon() })
    };
    let name = obj.tags().get("name").cloned().unwrap_or("".to_string());
    let id = format!("StopPoint:{}:{}", obj_type, obj_id);
    StopPoint { id: id, name: name, coord: coord }
}

pub fn get_stops_from_osm(pbf: &mut OsmPbfReader) -> Vec<StopPoint> {
    let objects = pbf.get_objs_and_deps(is_stop_point).unwrap();
    objects.values()
        .filter(|x| is_stop_point(*x))
        .map(|obj| osm_obj_to_stop_point(&objects, obj))
        .collect()
}

pub fn get_routes_from_osm(pbf: &mut OsmPbfReader) -> Vec<Route> {
    let objects = pbf.get_objs_and_deps(is_route).unwrap();
    objects.values()
        .filter(|x| is_route(*x))
        .filter_map(|obj| osm_obj_to_route(&objects, obj))
        .collect()
}

pub fn get_lines_from_osm(pbf: &mut OsmPbfReader) -> Vec<Line> {
    let objects = pbf.get_objs_and_deps(is_line).unwrap();
    objects.values()
        .filter(|x| is_line(*x))
        .filter_map(|obj| osm_obj_to_line(&objects, obj))
        .collect()
}

pub fn write_stops_to_csv(stops : Vec<StopPoint>) {
    let csv_file = std::path::Path::new("/tmp/osmtc2mongo.csv");
    let mut wtr = csv::Writer::from_file(csv_file).unwrap();

    for sp in &stops {
        wtr.encode(sp).unwrap();
    }
}

pub fn write_routes_to_csv(routes : Vec<Route>) {
    let csv_file = std::path::Path::new("/tmp/osmtc2mongo_routes.csv");
    let mut wtr = csv::Writer::from_file(csv_file).unwrap();

    for r in &routes {
        wtr.encode(r).unwrap();
    }
}

pub fn write_lines_to_csv(lines : Vec<Line>) {
    let csv_file = std::path::Path::new("/tmp/osmtc2mongo_lines.csv");
    let mut wtr = csv::Writer::from_file(csv_file).unwrap();

    for r in &lines {
        wtr.encode((&r.id, &r.name, &r.code, &r.routes_id)).unwrap();
    }
}
