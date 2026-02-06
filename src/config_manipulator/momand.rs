use kdl::KdlDocument;
use std::fs;

/// Reads the builtin monitor name from the moman configuration file.
/// Parses a KDL configuration file and extracts the builtin monitor name from the
/// `builtin-monitor` node. If the node exists but has no value, defaults to "eDP-1".
///
/// # Arguments
/// * `file_path` - Path to the moman KDL configuration file
///
/// # Returns
/// * `Some(String)` - The monitor name if the `builtin-monitor` node exists
/// * `None` - If the `builtin-monitor` node is not present in the configuration
///
/// # Panics
/// Panics if the file cannot be read or if the KDL document is malformed.
pub fn config_builtin_monitor(file_path: &str) -> Option<String> {
    let kdl_str = fs::read_to_string(file_path).expect("Error reading momand config KDL file");
    let doc: KdlDocument = kdl_str.parse().expect("Failed to parse momand config KDL document");

    let node = doc.get("builtin-monitor")?;
    node.get(0)
        .map(|v| v.to_string())
        .or_else(|| Some("eDP-1".to_string()))
}
