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
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate wkt;
use std::collections::BTreeMap;
use std::path::Path;
use osmpbfreader::OsmObj::*;
use serde::Serializer;
use std::collections::btree_set::BTreeSet;

pub type OsmPbfReader = osmpbfreader::OsmPbfReader<std::fs::File>;

#[derive(Serialize, Debug, Clone)]
pub struct Coord {
    lat: f64,
    lon: f64,
}
impl Coord {
    fn new(lat_param: f64, lon_param: f64) -> Coord {
        Coord {
            lat: lat_param,
            lon: lon_param,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StopPoint {
    pub id: String,
    pub coord: Coord,
    pub name: String,
    pub all_osm_tags: osmpbfreader::objects::Tags,
}

#[derive(Debug, Clone)]
pub struct StopArea {
    pub id: String,
    pub coord: Coord,
    pub name: String,
    pub all_osm_tags: osmpbfreader::objects::Tags,
    pub stop_point_ids: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Route {
    pub id: String,
    pub name: String,
    pub code: String,
    pub destination: String,
    pub origin: String,
    pub colour: String,
    pub operator: String,
    pub network: String,
    pub mode: String,
    #[serde(skip)]
    pub ordered_stops_id: Vec<String>,
    #[serde(serialize_with = "shape_to_wkt")]
    pub shape: Vec<Vec<Coord>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Line {
    pub id: String,
    pub name: String,
    pub code: String,
    pub colour: String,
    pub operator: String,
    pub network: String,
    pub mode: String,
    #[serde(serialize_with = "shape_to_wkt")]
    pub shape: Vec<Vec<Coord>>,
    #[serde(skip)]
    pub routes_id: Vec<String>,
}

#[allow(dead_code)]
/* to_multilinestring is to be used when issue #8 is resolved*/
impl Route {
    fn to_multilinestring(&self) -> wkt::types::MultiLineString {
        let wkt_linestrings = self.shape
            .iter()
            .map(|vec_coord| {
                vec_coord.iter()
                    .map(|coord| {
                        wkt::types::Coord {
                            x: coord.lon,
                            y: coord.lat,
                            z: None,
                            m: None,
                        }
                    })
                    .collect()
            })
            .map(|wkt_coords| wkt::types::LineString(wkt_coords))
            .collect();
        wkt::types::MultiLineString(wkt_linestrings)
    }
}


fn shape_to_wkt<S>(shape: &Vec<Vec<Coord>>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    if shape.len() == 0 {
        serializer.serialize_none()
    } else {
        let linestring: String = shape.iter()
            .map(|vec_coord| {
                vec_coord.iter()
                    .map(|coord| format!("{} {}", coord.lon.to_string(), coord.lat.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .collect::<Vec<String>>()
            .join("), (");
        serializer.serialize_str(&format!("MULTILINESTRING(({}))", linestring))
    }
}

pub struct OsmTcResponse {
    pub stop_points: Vec<StopPoint>,
    pub stop_areas: Vec<StopArea>,
    pub routes: Option<Vec<Route>>,
    pub lines: Option<Vec<Line>>,
}

pub fn parse_osm_pbf(path: &str) -> OsmPbfReader {
    let path = std::path::Path::new(&path);
    osmpbfreader::OsmPbfReader::new(std::fs::File::open(&path).unwrap())
}

fn is_stop_point(obj: &osmpbfreader::OsmObj) -> bool {
    obj.tags().contains("public_transport", "plateform") ||
    (obj.is_node() && obj.tags().contains("highway", "bus_stop"))
}

fn is_stop_area(obj: &osmpbfreader::OsmObj) -> bool {
    obj.is_relation() && obj.tags().contains("public_transport", "stop_area")
}

fn is_pt_route_type(route_type: Option<&String>) -> bool {
    // let non_pt_route_type = vec!["bicycle",
    //                               "canoe",
    //                               "detour",
    //                               "fitness_trail",
    //                               "foot",
    //                               "hiking",
    //                               "horse",
    //                               "inline_skates",
    //                               "mtb",
    //                               "nordic_walking",
    //                               "pipeline",
    //                               "piste",
    //                               "power",
    //                               "proposed",
    //                               "road",
    //                               "running",
    //                               "ski",
    //                               "historic",
    //                               "path"];
    let pt_route_type = vec![
        "trolleybus",
        "bus",
        "train",
        "subway",
        "light_rail",
        "monorail",
        "tram",
        "railway",
        "ferry",
        "coach",
        "aerialway",
        "funicular",
        "rail",
    ];
    match route_type {
        Some(r) => pt_route_type.contains(&r.as_str()),
        None => false
    }
}

fn is_line(obj: &osmpbfreader::OsmObj) -> bool {
    obj.is_relation() &&
        obj.tags().contains("type", "route_master") &&
        is_pt_route_type(obj.tags().get("route_master"))
}

fn is_route(obj: &osmpbfreader::OsmObj) -> bool {
    obj.is_relation() &&
        obj.tags().contains("type", "route") &&
        is_pt_route_type(obj.tags().get("route"))
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
        .filter_map(|osm_obj| match *osm_obj {
            Way(ref way) => Some(get_one_coord_from_way(obj_map, way)),
            Node(ref node) => Some(Coord::new(node.lat(), node.lon())),
            Relation(..) => None,
        })
        .next()
        .unwrap_or(Coord::new(0., 0.))
}

fn osm_way_to_vec(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                  osm_way: &osmpbfreader::Way)
                  -> Vec<Coord> {
    osm_way.nodes
        .iter()
        .filter_map(|id| obj_map.get(&osmpbfreader::OsmId::Node(*id)))
        .filter_map(|osm_obj| osmpbfreader::OsmObj::node(osm_obj))
        .map(|node| Coord::new(node.lat(), node.lon()))
        .collect()
}

fn osm_route_to_shape(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                      osm_relation: &osmpbfreader::Relation)
                      -> Vec<Vec<Coord>> {
    osm_relation.refs
        .iter()
        .filter_map(|refe| obj_map.get(&refe.member))
        .filter_map(|osm_obj| osmpbfreader::OsmObj::way(osm_obj))
        .filter_map(|osm_way| Some(osm_way_to_vec(obj_map, osm_way)))
        .collect()
}

fn osm_line_to_shape(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                     osm_relations_ref: &Vec<osmpbfreader::Ref>)
                     -> Vec<Vec<Coord>> {

    osm_relations_ref.iter()
        .filter_map(|refe| obj_map.get(&refe.member))
        .filter_map(|osm_obj| osmpbfreader::OsmObj::relation(osm_obj))
        .flat_map(|relation| osm_route_to_shape(obj_map, relation))
        .collect()
}

fn osm_route_to_stop_list(osm_relation: &osmpbfreader::Relation) -> Vec<String> {
    let stop_roles = vec!["stop",
                          "platform",
                          "stop_exit_only",
                          "stop_entry_only",
                          "platform_exit_only",
                          "platform_entry_only",
                          "fixme"];
    osm_relation.refs
        .iter()
        .filter(|refe| stop_roles.contains(&refe.role.as_str()))
        .map(|refe| match refe.member {
            osmpbfreader::OsmId::Node(obj_id) => format!("StopPoint:Node:{}", obj_id.0),
            osmpbfreader::OsmId::Way(obj_id) => format!("StopPoint:Way:{}", obj_id.0),
            osmpbfreader::OsmId::Relation(obj_id) => format!("StopPoint:Relation:{}", obj_id.0),
        })
        .collect()
}

fn osm_line_to_routes_list(route_master: &osmpbfreader::Relation) -> Vec<String> {
    route_master.refs
        .iter()
        .filter_map(|refe| match refe.member {
            osmpbfreader::OsmId::Relation(rel_id) => Some(format!("Route:Relation:{}", rel_id.0)),
            _ => None,
        })
        .collect()
}

fn osm_obj_to_route(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                    obj: &osmpbfreader::OsmObj)
                    -> Option<Route> {
    obj.relation().map(|rel| {
        Route {
            id: format!("Route:Relation:{}", rel.id.0),
            name: rel.tags
                .get("name")
                .cloned()
                .unwrap_or_default(),
            code: rel.tags
                .get("ref")
                .cloned()
                .unwrap_or_default(),
            destination: rel.tags
                .get("to")
                .cloned()
                .unwrap_or_default(),
            origin: rel.tags
                .get("from")
                .cloned()
                .unwrap_or_default(),
            mode: rel.tags
                .get("route")
                .cloned()
                .unwrap_or_default(),
            colour: rel.tags
                .get("colour")
                .cloned()
                .unwrap_or_default(),
            operator: rel.tags
                .get("operator")
                .cloned()
                .unwrap_or_default(),
            network: rel.tags
                .get("network")
                .cloned()
                .unwrap_or_default(),
            ordered_stops_id: osm_route_to_stop_list(rel),
            shape: osm_route_to_shape(obj_map, rel),
        }
    })
}

fn osm_obj_to_line(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                   obj: &osmpbfreader::OsmObj)
                   -> Option<Line> {


    obj.relation().map(|rel| {
        Line {
            id: format!("Line:Relation:{}", rel.id.0),
            name: rel.tags
                .get("name")
                .cloned()
                .unwrap_or_default(),
            code: rel.tags
                .get("ref")
                .cloned()
                .unwrap_or_default(),
            colour: rel.tags
                .get("colour")
                .cloned()
                .unwrap_or_default(),
            mode: rel.tags
                .get("route_master")
                .cloned()
                .unwrap_or_default(),
            operator: rel.tags
                .get("operator")
                .cloned()
                .unwrap_or_default(),
            network: rel.tags
                .get("network")
                .cloned()
                .unwrap_or_default(),
            shape: osm_line_to_shape(obj_map, &rel.refs),
            routes_id: osm_line_to_routes_list(rel),
        }
    })
}

fn osm_obj_to_stop_point(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                         obj: &osmpbfreader::OsmObj)
                         -> StopPoint {
    let (obj_type, obj_id, coord) = match *obj {
        Relation(ref rel) => ("Relation", rel.id.0, get_one_coord_from_rel(obj_map, rel)),
        Way(ref way) => ("Way", way.id.0, get_one_coord_from_way(obj_map, way)),
        Node(ref node) => {
            ("Node",
             node.id.0,
             Coord {
                 lat: node.lat(),
                 lon: node.lon(),
             })
        }
    };
    let name = obj.tags()
        .get("name")
        .cloned()
        .unwrap_or_default();
    let id = format!("StopPoint:{}:{}", obj_type, obj_id);
    let osm_tags = obj.tags().clone();
    StopPoint {
        id: id,
        name: name,
        coord: coord,
        all_osm_tags: osm_tags,
    }
}

fn osm_obj_to_stop_area(obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
                         obj: &osmpbfreader::OsmObj)
                         -> StopArea {
    let rel = &*obj.relation().unwrap();
    let (obj_type, obj_id, coord) = ("Relation", rel.id.0, get_one_coord_from_rel(obj_map, &rel));
    let name = obj.tags()
        .get("name")
        .cloned()
        .unwrap_or_default();
    let osm_tags = obj.tags().clone();
    StopArea {
        id: format!("StopArea:{}:{}", obj_type, obj_id),
        name: name,
        coord: coord,
        all_osm_tags: osm_tags,
        stop_point_ids: osm_stop_area_to_stop_point_list(rel),
    }
}

fn osm_stop_area_to_stop_point_list(osm_relation: &osmpbfreader::Relation) -> Vec<String> {
    osm_relation.refs
        .iter()
        .filter(|refe| refe.role.as_str() == "platform")
        .map(|refe| match refe.member {
            osmpbfreader::OsmId::Node(obj_id) => format!("StopPoint:Node:{}", obj_id.0),
            osmpbfreader::OsmId::Way(obj_id) => format!("StopPoint:Way:{}", obj_id.0),
            osmpbfreader::OsmId::Relation(obj_id) => format!("StopPoint:Relation:{}", obj_id.0),
        })
        .collect()
}

pub fn get_stop_points_from_osm(pbf: &mut OsmPbfReader) -> Vec<StopPoint> {
    let objects = pbf.get_objs_and_deps(is_stop_point).unwrap();
    objects.values()
        .filter(|x| is_stop_point(*x))
        .map(|obj| osm_obj_to_stop_point(&objects, obj))
        .collect()
}

pub fn get_stop_areas_from_osm(pbf: &mut OsmPbfReader) -> Vec<StopArea> {
    let objects = pbf.get_objs_and_deps(is_stop_area).unwrap();
    objects.values()
        .filter(|x| is_stop_area(*x))
        .map(|obj| osm_obj_to_stop_area(&objects, obj))
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

pub fn get_osm_tcobjects(parsed_pbf: &mut OsmPbfReader, stops_only: bool) -> OsmTcResponse {
    let stop_points = get_stop_points_from_osm(parsed_pbf);
    let stop_areas = get_stop_areas_from_osm(parsed_pbf);
    if stops_only {
        OsmTcResponse {
            stop_points: stop_points,
            stop_areas: stop_areas,
            routes: None,
            lines: None,
        }
    } else {
        let routes = get_routes_from_osm(parsed_pbf);
        let lines = get_lines_from_osm(parsed_pbf);
        OsmTcResponse {
            stop_points: stop_points,
            stop_areas: stop_areas,
            routes: Some(routes),
            lines: Some(lines),
        }
    }
}


fn get_stop_area_ids_for_stop_point(stop_point_id: &String,
                                    stop_areas: &Vec<StopArea>)
                                    -> Vec<String> {
    stop_areas.iter()
              .filter(|sa| sa.stop_point_ids.contains(stop_point_id))
              .map(|sa| sa.id.to_string())
              .collect()
}

pub fn write_stop_points_to_csv<P: AsRef<Path>>(stop_points: &Vec<StopPoint>,
                                                stop_areas: &Vec<StopArea>,
                                                output_dir: P) {
    let output_dir = output_dir.as_ref();
    let csv_file = output_dir.join("osm-transit-extractor_stop_points.csv");

    let mut wtr = csv::Writer::from_path(csv_file).unwrap();
    let osm_tag_list: BTreeSet<String> =
        stop_points.iter().flat_map(|s| s.all_osm_tags.keys().map(|s| s.to_string())).collect();
    let osm_header = osm_tag_list.iter().map(|s| format!("osm:{}", s));
    let v: Vec<_> = ["stop_point_id", "lat", "lon", "name", "fist_stop_area_id"]
        .iter()
        .map(|s| s.to_string())
        .chain(osm_header)
        .collect();
    wtr.serialize(v).unwrap();

    for sp in stop_points {
        let stop_area_ids = get_stop_area_ids_for_stop_point(&sp.id, &stop_areas);
        let csv_row = vec![sp.id.to_string(),
                           sp.coord.lat.to_string(),
                           sp.coord.lon.to_string(),
                           sp.name.to_string(),
                           stop_area_ids.iter().next().unwrap_or(&"".to_string()).to_string(),
                           ];
        let csv_row: Vec<_> = csv_row.into_iter()
            .chain(osm_tag_list.iter()
                .map(|k| sp.all_osm_tags.get(k).map_or("", |s| s.as_str()).to_string()))
            .collect();
        wtr.serialize(csv_row).unwrap();
    }
}

pub fn write_stop_areas_to_csv<P: AsRef<Path>>(stop_areas: &Vec<StopArea>, output_dir: P) {
    let output_dir = output_dir.as_ref();
    let csv_file = output_dir.join("osm-transit-extractor_stop_areas.csv");

    let mut wtr = csv::Writer::from_path(csv_file).unwrap();
    let osm_tag_list: BTreeSet<String> =
        stop_areas.iter().flat_map(|s| s.all_osm_tags.keys().map(|s| s.to_string())).collect();
    let osm_header = osm_tag_list.iter().map(|s| format!("osm:{}", s));
    let v: Vec<_> = ["stop_area_id", "lat", "lon", "name"]
        .iter()
        .map(|s| s.to_string())
        .chain(osm_header)
        .collect();
    wtr.serialize(v).unwrap();

    for sa in stop_areas {
        let csv_row = vec![sa.id.to_string(),
                           sa.coord.lat.to_string(),
                           sa.coord.lon.to_string(),
                           sa.name.to_string()];
        let csv_row: Vec<_> = csv_row.into_iter()
            .chain(osm_tag_list.iter()
                .map(|k| sa.all_osm_tags.get(k).map_or("", |s| s.as_str()).to_string()))
            .collect();
        wtr.serialize(csv_row).unwrap();
    }
}
pub fn write_routes_to_csv<P: AsRef<Path>>(routes: Vec<Route>, output_dir: P) {
    let output_dir = output_dir.as_ref();
    let csv_route_file = output_dir.join("osm-transit-extractor_routes.csv");
    let csv_route_stops_file = output_dir.join("osm-transit-extractor_route_stops.csv");
    let mut wtr_route = csv::Writer::from_path(csv_route_file).unwrap();
    let mut wtr_stops = csv::Writer::from_path(csv_route_stops_file).unwrap();
    wtr_stops.serialize(("route_id", "stop_id")).unwrap();
    wtr_route.serialize(("route_id",
                    "name",
                    "code",
                    "destination",
                    "origin",
                    "colour",
                    "operator",
                    "network",
                    "mode",
                    "shape"))
        .unwrap();

    for r in &routes {
        for s in &r.ordered_stops_id {
            let row = vec![r.id.to_string(), s.to_string()];
            wtr_stops.write_record(row.into_iter()).unwrap();
        }
        wtr_route.serialize(r).unwrap();
    }
}

pub fn write_lines_to_csv<P: AsRef<Path>>(lines: Vec<Line>, output_dir: P) {
    let output_dir = output_dir.as_ref();
    let lines_csv_file = output_dir.join("osm-transit-extractor_lines.csv");
    let mut lines_wtr = csv::Writer::from_path(lines_csv_file).unwrap();
    lines_wtr.serialize(("line_id",
                    "name",
                    "code",
                    "colour",
                    "operator",
                    "network",
                    "mode",
                    "shape"))
        .unwrap();

    let csv_file = output_dir.join("osm-transit-extractor_line_routes.csv");
    let mut wtr = csv::Writer::from_path(csv_file).unwrap();
    wtr.serialize(("line_id", "route_id")).unwrap();

    for l in &lines {
        lines_wtr.serialize(l).unwrap();
        for r in &l.routes_id {
            wtr.serialize((&l.id, &r)).unwrap();
        }
    }
}
