# Polymesh offline signer

Utility for offline signing of transaction for the [Polymesh](https://polymesh.network/) blockchain.

## Usage

### Install with cargo

```bash
cargo install polymesh-offline-signer
```

### Prepare a transaction:

Preparing a transaction requires a Polymesh node to query the account nonce.

Command:

```bash
# Prepare a POLYX transfer from Alice to Bob.
polymesh-offline-signer prepare \
	-u ws://localhost:9944/ -a 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
	balance-transfer 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 1.0 >./prepared_tx.hex
```

The output is the prepared transaction in hex saved to file `prepared_tx.hex`.

```
0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d808d5b000400000088db578301a1f5b2556b84a148e489d7d41825265a9d1cf534d4cca8346977ba88db578301a1f5b2556b84a148e489d7d41825265a9d1cf534d4cca8346977ba0018000500008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4802093d00
```

### Offline signing of the prepared transaction:

This signing step doesn't require any network connection.

Command:

```bash
# Use Alice's key to sign the transaction.
# The `prepared_tx.hex` file is the output from the "Prepare" step above.
polymesh-offline-signer offline-sign --suri //Alice ./prepared_tx.hex >./signed_tx.hex
```

The output is the signed transaction in hex saved to file `signed_tx.hex`.

```
0x35028400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01fe4c7ebb7fe01595e0d3d30d3f29167b0ef48c959bdec09729692549b71a8843726628ffd7108774ea90063a9ae178e603727175f8b3fe6d38092d3c09c228800018000500008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4802093d00
```

### Submit the signed transaction for execution:

Submit the signed transaction and wait for it to be finalized.

Command:

```bash
# Submit the signed transaction to the network
# The `signed_tx.hex` file is the output from the "Offline signing" step above.
polymesh-offline-signer submit --finalized -u ws://localhost:9944/ ./signed_tx.hex
```

output:

```
In block: 0x45daf1b4a3d790d1b91b3ad2cd7d6ba100e71d0d753fb735b3df0db7384afc4c
Successful
```

