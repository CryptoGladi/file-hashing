//! Data translation

use digest::Digest;

/// Convert hash to readable **hex lower**
pub(crate) fn get_lowerhex<HashType: Digest + Clone>(
    hash: &mut HashType,
) -> String {
    data_encoding::HEXLOWER.encode(hash.clone().finalize().as_ref())
}
