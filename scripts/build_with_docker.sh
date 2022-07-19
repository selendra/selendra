# for support wild range os

docker run --rm -it -w /shellhere/polkadot \
-v $(pwd):/shellhere/polkadot \
paritytech/ci-linux:production cargo build --release
