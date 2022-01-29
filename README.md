# rust_rdkit


## How to use
- clone rdkitcffi (https://github.com/chrissly31415/rdkitcffi)
- clone this repo

```
$ gh repo clone chrissly31415/rdkitcffi
$ gh repo clone iwatobipen/rust_rdkit
```

- edit rdkitcffi/build.rs

```
from relative path
    let shared_lib_dir = "./lib/rdkitcffi_linux/linux-64/";
to absolute path
    let shared_lib_dir = "/home/user/hogehoge/lib/rdkitcffi_linux/linux-64/";
```


## edit Cargo.toml in the rust_rdkit then build the code (path of rdkitcffi)

```
$ cd rust_rdkit
$ cargo build
```


## Done!

```
$ ./target/debug/rdkrust cdk2.sdf
```
