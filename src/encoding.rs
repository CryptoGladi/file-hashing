//! Data translation

use digest::DynDigest;

/// Convert hash to readable **hex lower**
pub(crate) fn get_lowerhex<HashType: DynDigest + Clone>(
    hash: &mut HashType,
) -> String {
    data_encoding::HEXLOWER.encode(Box::new(hash.clone()).finalize().as_ref())
}
