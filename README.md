# InterlockLedger ILTag for Rust

## Description

This is the implementation of **InterlockLedger** **ILTag** for **Rust**. For more information, see
the [specification of IL2](https://github.com/interlockledger/specification/tree/master).

## Dependencies

This library has been developed to run without any external dependencies aside for the
Rust standard library.

## License

This library is licensed under a 3-Clause BSD license.

## Maintainers

- Fabio Jun Takada Chino
- Cesar Luiz Ferracin

## Version history

- 1.4.0:
    - `il2_iltags::io::array:ByteArrayWriter` is back;
    - Some traits are now enforcing `Sync`;
    - `ILSignedILInt64Tag` and `IL_SIGNED_ILINT_TAG_ID` are now reexported by `il2_iltags::tags::standard`;
    - `ILRawTag.set_value()` added;
    - `Deref` and `DerefMut` implemented for `ILGenericPayloadTag` as an experimental feature for now;
    - Documentation updated;
    - Method `Reader::skip_u64()` added;
    - Implementation of `Reader` for `std::io::Read` + `std::io::Seek` added;
    - Implementation of `Writer` for `std::io::Write` + `std::io::Seek` added;    
    - Implementation of `std::convert::Into<Vec<u8>>` for `VecWriter`added; 
- 1.3.0:
    - Automatic conversion from `il2_iltags::io::ErrorKind` to `il2_iltags::tags::ErrorKind` added;
    - Applying code cleanup suggested by clippy;
    - Method `il2_iltags::tags::serialization::ByteArraySerializer::serialize_bytes()` has been deprecated;
    - Support to the new **ILInt** sign encoding added;
    - Support to the new **ILIntSigned** tag added;
    - Method `ILTagFactory.deserialize_into()` added;
    - Exposing `UntouchbleTagFactory` to the public API;
- 1.2.0:
    - Version history updated;
    - Documentation of `ILGenericPayloadTag` updated. Now the example presented in the documentation
      is a valid Rust code;
    - New set of IO wrapper traits added. This should make the implementation of custom tags easier;
    - Minor adjustments in the unit test code in order to make it easier to be reused within other unit tests;
- 1.1.1:
    - Additional `ILTag` downcast functions added;
    - Base tag for custom payloads added;
    - All `ErrorKind` defined by this library now implements Debug trait;
- 1.1.0:
    - `il2_iltags::io::BorrowedVecWriter` added;
    - Adding new traits to the module `il2_iltags::io::data` to make the usage of
      the read/write functions easier to use over `Reader` and `Writer` implementators;
- 1.0.1:
    - Issue #1 - Invalid code snippets marked as ignored;
- 1.0.0:
    - Initial release;
