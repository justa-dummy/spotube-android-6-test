# Android 6 backport: on 32-bit ARM, JNA hands the u16 checksum values back without the
# expected extension, so every API checksum "mismatches" and the library refuses to load.
# The checksums are only a build-pipeline sanity check, so skip them.
[bindings.kotlin]
omit_checksums = true
