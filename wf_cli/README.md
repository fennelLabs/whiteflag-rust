


#### Encode

```
cargo run -- encode "{\"prefix\":\"WF\",\"version\":\"1\",\"encryptionIndicator\":\"0\",\"duressIndicator\":\"0\",\"messageCode\":\"A\",\"referenceIndicator\":\"0\",\"referencedMessage\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"verificationMethod\":\"1\",\"verificationData\":\"https://organisation.int/whiteflag\"}"
```

#### Decode

```
cargo run -- decode 5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380
```