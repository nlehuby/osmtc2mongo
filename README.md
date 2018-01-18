# osm-transit-extractor [![Build status](https://travis-ci.org/CanalTP/osm-transit-extractor.svg?branch=master)](https://travis-ci.org/CanalTP/osm-transit-extractor/)

Extract public transport data from an [OpenStreetMap](http://www.openstreetmap.org/) file, and write the result to a list of csv files.
This tool uses the crate [osmpbfreader](https://github.com/TeXitoi/osmpbfreader-rs)  to read the provided [OpenStreetMap PBF
files](http://wiki.openstreetmap.org/wiki/PBF_Format) 

## How to use
Run the program with --help to display available parameters. The simplest way to use it is :
`osm-transit-extractor -i name_of_the_osm_file.osm.pbf`

This command will extract the public transport data and write them to CSV files in the current directory. The output directory can be changed with the use of the parameter `-o /path/to/the/dest/directory/`

