---
title: Create a Custom Chain Spec
---


In this example we will create a two-node network, but the process generalizes to more nodes in a
straight-forward manner.

## Create a Chain Specification

Last time around, we used `--chain local` which is a predefined "chain spec" that has Alice and Bob
specified as validators along with many other useful defaults.

Rather than writing our chain spec completely from scratch, we'll just make a few modifications to
the one we used before. To start, we need to export the chain spec to a file named
`customSpec.json`. Remember, further details about all of these commands are available by running
`node-template --help`.

```bash
# Export the local chainspec to json
$ ./target/release/selendra build-spec --disable-default-bootnode --chain selendra-local > customSpec.json
2022-06-27 15:42:06 Building chain spec
```


The file we just created contains several fields, and you can learn a lot by exploring them. By far
the largest field is a single binary blob that is the Wasm binary of our runtime. It is part of what
you built earlier when you ran the `cargo build --release` command.

The portion of the file we're interested in is the Aura authorities used for creating blocks,
indicated by **"babe"** field below, and GRANDPA authorities used for finalizing blocks, indicated
by **"grandpa"** field. That section looks like this

```json
{
  //-- snip --
   "genesis": {
    "runtime": {
      "system": {
        //-- snip --
      },
      "babe": {
        "authorities": [],
        "epochConfig": {
          "c": [
            1,
            4
          ],
          "allowed_slots": "PrimaryAndSecondaryPlainSlots"
        }
      },
      //-- snip --
       "grandpa": {
        "authorities": []
      },
      //-- snip --
    }
  }
}
```

> Validators should not share the same keys, even for learning purposes. If two validators have the
> same keys, they will produce conflicting blocks.

Once the chain spec is prepared, convert it to a "raw" chain spec. The raw chain spec contains all
the same information, but it contains the encoded storage keys that the node will use to reference
the data in its local storage. Distributing a raw spec ensures that each node will store the data at
the proper storage keys.


```bash
$ ./target/release/selendra build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
2022-06-27 15:49:29 Building chain spec
```

Finally share the `customSpecRaw.json` with your all the other validators in the network.

> A single person should create the chain spec and share the resulting **`customSpecRaw.json`** file
> with their fellow validators.
>
> Because Rust -> Wasm optimized builds aren't "reproducible", each person will get a slightly
> different Wasm blob which will break consensus if each participant generates the file themselves.
