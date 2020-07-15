# Todo

*Main stuff*
 - `ShardID::from_shard()` calculation
 - Compression
 - Encryption
 - Database
 - File::new() ShardDB prepping (null node addresses but give them the shard)

*Refactoring*
 - Make all of the XError types MODError. So, the crypto module has only a CryptoError type, etc...
 - Make the CanSerialize from_bytes() generic

*Other*
 - crypto load key
 - crypto decrypt



*Problems*
 - Trais need to be more generalized
 - Check the typing of the `decrypt<T>` method and `from_bytes<T>` method. Those make no sense because they are not properly restricted.
