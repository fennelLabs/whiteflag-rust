## Port From Java

This module contains ported logic from WfBinaryBuffer.java and WfMessageCodec.java

- codec (binary.rs, hexadecimal.rs, latlong.rs, encoding.rs, constants.rs)
  - binary
  - hexadecimal
  - latlong
  - encodeField
  - decodeField
- buffer (common.rs)
  - convertToByteArray
  - convertToHexString
  - shiftRight
  - shiftLeft
  - appendBits
  - extractBits
  - concatinateBits
  - cropBits
  - byteLength
