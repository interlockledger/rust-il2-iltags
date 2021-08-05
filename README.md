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

- 1.2.1:
    - Automatic conversion from `il2_iltags::io::ErrorKind` to `il2_iltags::tags::ErrorKind` added;
    - Applying code cleanup suggested by
    - Method `il2_iltags::tags::serialization::ByteArraySerializer::serialize_bytes()` has been deprecated;
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
