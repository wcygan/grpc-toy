# grpc-toy

A toy gRPC server written in Rust.

Requires [protoc](https://grpc.io/docs/protoc-installation/) & [grpcurl](https://github.com/fullstorydev/grpcurl).

## Usage

### Start the server

```bash
$ cargo run
```

### Send a request

```bash
grpcurl -plaintext -proto protos/hello.proto -d '{"name": "world"}' localhost:8080 hello.HelloService/SayHello
```

Reply:

```
{
  "reply": "Hello, world!"
}
```

## Integration tests

First, start the server:

```bash
$ cargo run
```

Then, run the tests:

```bash
$ cargo test
```