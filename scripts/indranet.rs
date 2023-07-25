#!/bin/bash

./target/release/indranet \
    --substrate-ws-endpoint ws://127.0.0.1:9944 \
    --iruntime-endpoint http://127.0.0.1:18000 \
    --no-wait \
    --use-dev-key \
    --mnemonic=//Ferdie \
    --attestation-provider none
