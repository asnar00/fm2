struct feature_LightBasemap;
impl feature_LightBasemap {
    // fieldnote's basemap was CARTO voyager, keyless; CARTO now watermarks
    // every keyless tile, so the working equivalent is the OSM cartography
    // voyager restyles, straight from the source. MISO_TILE_URL still wins.
    fn tiles_default_url() -> String {
        "https://tile.openstreetmap.org/{z}/{x}/{y}.png".to_string()
    }
}
