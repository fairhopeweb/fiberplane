pub mod operations;
pub mod protocols;
pub mod query_data;
pub mod sorting;
pub mod text_util;

fn debug_print_bytes(bytes: impl AsRef<[u8]>) -> String {
    let bytes = bytes.as_ref();
    if bytes.len() > 100 {
        format!("{}...", String::from_utf8_lossy(&bytes[..100]))
    } else {
        String::from_utf8_lossy(bytes).to_string()
    }
}
