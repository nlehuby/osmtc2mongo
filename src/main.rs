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

extern crate osm_transit_extractor;
extern crate osmpbfreader;
extern crate structopt;

#[macro_use]
extern crate structopt_derive;
use osm_transit_extractor::*;
use std::path::PathBuf;
use structopt::StructOpt;
#[macro_use]
extern crate log;
extern crate simple_logger;

#[derive(StructOpt)]
struct Args {
    #[structopt(long = "input", short = "i", help = "OSM PBF file")]
    input: String,

    #[structopt(
        long = "import-stops-only",
        short = "s",
        help = "Imports only stop_points and stop_areas (default is a full extraction)"
    )]
    import_stops_only: bool,

    #[structopt(
        long = "output",
        short = "o",
        default_value = ".",
        parse(from_os_str),
        help = "Output directory, can be relative (default is current dir)"
    )]
    output: PathBuf,
}

fn main() {
    simple_logger::init().unwrap();
    info!("Launching the process !");

    let args = Args::from_args();

    let mut parsed_pbf = parse_osm_pbf(&args.input);

    let osmtc_response = get_osm_tcobjects(&mut parsed_pbf, args.import_stops_only);

    write_stop_points_to_csv(
        &osmtc_response.stop_points,
        &osmtc_response.stop_areas,
        &args.output,
    );
    write_stop_areas_to_csv(&osmtc_response.stop_areas, &args.output);

    if osmtc_response.routes.is_some() {
        write_routes_to_csv(osmtc_response.routes.unwrap(), &args.output);
    }
    if osmtc_response.lines.is_some() {
        write_lines_to_csv(osmtc_response.lines.unwrap(), &args.output);
    }
    info!("end of osm-transit-extractor !")
}
