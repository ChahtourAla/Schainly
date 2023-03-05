#!/bin/sh

prefix=/home/chatoor/Desktop/Hackathon/contracts/SchainlyPool/contract/target/debug/build/jemalloc-sys-9bfc59e598610f5f/out
exec_prefix=/home/chatoor/Desktop/Hackathon/contracts/SchainlyPool/contract/target/debug/build/jemalloc-sys-9bfc59e598610f5f/out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.so.2
export LD_PRELOAD
exec "$@"
