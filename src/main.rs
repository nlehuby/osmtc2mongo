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
extern crate osm_transit_extractor;
extern crate structopt;

#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;
use osm_transit_extractor::*;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Args {
    #[structopt(long = "input", short = "i", help = "OSM PBF file")]
    input: String,

    #[structopt(long = "import-stop-points-only", short = "s",
                help = "Imports only stop_points (default is a full extraction)")]
    import_stop_points_only: bool,

    #[structopt(long = "output", short = "o", default_value = ".", parse(from_os_str),
                help = "Output directory, can be relative (default is current dir)")]
    output: PathBuf,
}

fn main() {
    let args = Args::from_args();

    let mut parsed_pbf = parse_osm_pbf(&args.input);

    let osmtc_response = get_osm_tcobjects(&mut parsed_pbf, args.import_stop_points_only);

    write_stops_to_csv(osmtc_response.stop_points, &args.output);

    if osmtc_response.routes.is_some() {
        write_routes_to_csv(osmtc_response.routes.unwrap(), &args.output);
    }
    if osmtc_response.lines.is_some() {
        write_lines_to_csv(osmtc_response.lines.unwrap(), &args.output);
    }
    println!("end of osm-transit-extractor !")
}
