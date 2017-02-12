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
extern crate osmtc2mongo;
use osmtc2mongo::*;

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_input: String,
    flag_connection_string: String,
    flag_import_stop_points_only: bool,
}

static USAGE: &'static str = "
Usage:
    osmtc2mongo --help
    osmtc2mongo --input=<file> [--connection-string=<connection-string>] [--import-stop-points] [--import-routes]

Options:
    -h, --help                      Show this message.
    -i, --input=<file>              OSM PBF file.
    -s, --import-stop-points-only   Import only stop_points
    -c, --connection-string=<connection-string>
                                    Mongo parameters, [default: http://localhost:9200/osmtc]
";

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut parsed_pbf = parse_osm_pbf(&args.flag_input);

    let osmtc_response = get_osm_tcobjects(&mut parsed_pbf, args.flag_import_stop_points_only);
    write_stops_to_csv(osmtc_response.stop_points);

    if osmtc_response.routes.is_some() {
        write_routes_to_csv(osmtc_response.routes.unwrap());
    }
    if osmtc_response.lines.is_some() {
        write_lines_to_csv(osmtc_response.lines.unwrap());
    }
    println!("end of osmtc2mongo !")
}
