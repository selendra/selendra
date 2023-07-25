#!/bin/bash

export ROCKET_PORT=18000
export RUST_LOG=info,pruntime=trace,pink=trace,contracts=trace

./relayer/iruntime/target/release/iruntime \
    -c 0 \
    --allow-cors \
    --address 0.0.0.0 \
    --port 18000 \
|& tee pruntime.log
