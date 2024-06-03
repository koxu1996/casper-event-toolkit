use casper_hashing::Digest;

/// Recreate `Digest` to avoid leaking incompatible `casper-client` types.
pub fn digest_to_client_types(digest: Digest) -> casper_client_hashing::Digest {
    let raw_bytes: [u8; Digest::LENGTH] = digest.into();
    let digest = casper_client_hashing::Digest::from(raw_bytes);

    digest
}

/// Recreate incompatible `Digest` from `casper-client` types.
pub fn digest_from_client_types(digest: casper_client_hashing::Digest) -> Digest {
    let raw_bytes: [u8; Digest::LENGTH] = digest.into();
    let digest = Digest::from(raw_bytes);

    digest
}
