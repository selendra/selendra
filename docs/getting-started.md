### How to Run

- Build Selendra
```sh
cargo build --release
```

- You first need to setup up the node, which means you need to load the genesis state into your file system:
```sh
./target/release/selendra setup --chain=dev --from-local ./config 
```
- Now, you can start the node in development mode
```sh
./target/release/selendra --dev
```