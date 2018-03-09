# grpcio (pingcap)

*  generate code

```bash
cargo install protobuf

cargo install grpcio-compiler

protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` example.proto

just like :

protoc --rust_out=src --grpc_out=src --plugin=protoc-gen-grpc=/Users/dalong/.cargo/bin/grpc_rust_plugin proto/user.proto

```
* golang code generate

```bash
protoc -I proto/ proto/user.proto --go_out=plugins=grpc:./golang/service/proto
```

* python code generate

```bash
python2 -m grpc_tools.protoc -I proto --python_out=./python --grpc_python_out=./python  proto/user.proto
```

*  prequire

```bash
cargo update && cargo build
```

*  run

```bash
cd src/bin
cargo run --bin server
cargo run --bin client
```