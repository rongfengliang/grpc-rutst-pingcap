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

* nodejs code generate (static code generate)

```bash
npm install -g grpc-tools // maybe you need add --unsafe-perm

grpc_tools_node_protoc --js_out=import_style=commonjs,binary:./nodejs --grpc_out=./nodejs --plugin=protoc-ge
n-grpc=/usr/local/lib/node_modules/grpc-tools/bin/grpc_node_plugin ./proto/user.proto
```

* csharp code generate

```bash
csharp/Grpc.Tools.1.10.0/tools/macosx_x64/protoc -I. ./proto/user.proto --csharp_out csharp/src --grpc_out csharp/src --plugin=protoc-gen-grpc=csharp/Grpc.Tools.1.10.0/tools/macosx_x64/grpc_csharp_plugin

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