use ron::ser::PrettyConfig;

pub fn prettier_config() -> PrettyConfig {
    PrettyConfig::new()
        .separate_tuple_members(true)
        .enumerate_arrays(true)
        .indentor("  ".into())
}
