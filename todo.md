# Todo

*Main stuff*
 - Compression
 - File::new() ShardDB prepping (null node addresses but give them the shard)

*Refactoring*
 - Make all of the XError types MODError. So, the crypto module has only a CryptoError type, etc...
 - Make those encryption methods a macro plz

*Security*
 - Shard digital signatures
 - Checksums on `Shard`s and `File`s
 - Checking methods for validation in `from_bytes` reconstruction
