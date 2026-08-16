struct feature_QuietTiles;
impl feature_QuietTiles {
    // a label-free basemap: buildings and streets drawn, nothing named.
    // Its own cache name, so tiles of the two styles never mix.
    fn tile_style() -> String {
        "quiet".to_string()
    }

    fn tile_url(z: u32, x: i64, y: i64) -> String {
        format!("https://basemaps.cartocdn.com/dark_nolabels/{}/{}/{}.png", z, x, y)
    }

    fn tile_credit() -> String {
        "&copy; OpenStreetMap &middot; &copy; CARTO".to_string()
    }
}
