## OSM fixtures

The sample OSM files to use are the *.osm.pbf ones.

For now, they contain a really small subset of real OSM data.

To add data to the OSM fixture :
* open the *.osm source file, with JOSM (File > Open)
* use the "Dowload Object" dialog (File > Dowload Object..) to select existing OSM object
* save the new source file (File > Save As...)
* transform the *.osm file into an *.osm.pbf file using osmosis

Example of osmosis command line :
`osmosis --read-xml file="osm_fixture.osm" --write-pbf file="osm_fixture.osm.pbf"`

> NB: osmcnvert can be used for the conversion, but some *name* tags disapears on ways and relations

Do not forget to commit the usable *.osm.pbf file and the *.osm source file to ease the updates.

Note that if you modify the OSM data in JOSM you will need te remove the additions of the JOSM file-format in order to get a valid *.osm.pbf file.

See http://wiki.openstreetmap.org/wiki/JOSM_file_format to learn more.
