# Raft

generate sources
----------------

```
$ cargo install protobuf
$ cargo install grpcio-compiler
$ protoc --rust_out=src --grpc_out=src --plugin=protoc-gen-grpc=`which grpc_rust_plugin` proto/raftpb.proto
```

