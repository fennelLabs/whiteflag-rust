## Measuring Port

- Go through methods in java codebase and reference rust codebase where they have been implemented
  - 15 units tests from WfBinaryBufferTest.java
  - ✅ WfMessageFieldTest.java
- auth options?
- api/json?
- all crypto kinds?
- encrypt/decrypt
- api: encode/decode

| Mark       | Meaning                |
| ---------- | ---------------------- |
| P-COMPLETE | port is complete       |
| P-PARTIAL  | port is in progress    |
| P-TODO     | port has not yet begun |

## Resource Links

- [protocol v1](https://standard.whiteflagprotocol.org/v1/)
- [tests being taken from here](https://github.com/WhiteflagProtocol/whiteflag-java/blob/master/src/test/java/org/whiteflagprotocol/java/WfMessageTest.java#L67)
- [WhiteflagMessage](https://github.com/WhiteflagProtocol/whiteflag-java/blob/master/src/main/java/org/whiteflagprotocol/java/WfMessage.java)
- [WhiteflagMessageCreator](https://github.com/WhiteflagProtocol/whiteflag-java/blob/57db4b6963a4a7913afdeb596e7ce11d46d9d93b/src/main/java/org/whiteflagprotocol/java/core/WfMessageCreator.java#L20)
- [encode()](https://github.com/WhiteflagProtocol/whiteflag-java/blob/57db4b6963a4a7913afdeb596e7ce11d46d9d93b/src/main/java/org/whiteflagprotocol/java/core/WfMessageSegment.java#L377)
- [decode()](https://github.com/WhiteflagProtocol/whiteflag-java/blob/57db4b6963a4a7913afdeb596e7ce11d46d9d93b/src/main/java/org/whiteflagprotocol/java/core/WfMessageSegment.java#L399)
