# for support wild range os

docker run --rm -it -w /project/selendra \
-v $(pwd):/project/selendra \
paritytech/ci-linux:production cargo build --profile production
