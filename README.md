# glissando-vst

## native

### build

```sh
cargo build --release
```

### release

```sh
./scripts/osx_vst_bundler.sh Glissando target/release/libglissando_vst.dylib
```

### install

```sh
sudo chown -R root:wheel Glissando.vst
sudo mv Glissando.vst /Library/Audio/Plug-Ins/VST
```

## wasm

### setup

```sh
npm config set access public
```

### build

```sh
wasm-pack build --target web --scope glissando --release
```

### release

```sh
wasm-pack publish
```
