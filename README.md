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

- 1.1.0:
    - `il2_iltags::io::BorrowedVecWriter` added;
    - Adding new traits to the module `il2_iltags::io::data` to make the usage of
      the read/write functions easier to use over `Reader` and `Writer` implementators;
- 1.0.1:
    - Issue #1 - Invalid code snippets marked as ignored;
- 1.0.0:
    - Initial release;
