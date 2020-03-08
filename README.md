# glissando-synth

## native

### build

```sh
cargo build --release
```

### release

```sh
./scripts/osx_vst_bundler.sh Synth target/release/libglissando_synth.dylib
```

### install

```sh
sudo chown -R root:wheel Synth.vst
sudo mv Synth.vst /Library/Audio/Plug-Ins/VST
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

## notebooks

### setup

```sh
pip install jupyter numpy scipy matplotlib
```

### usage

```sh
jupyter notebook
```
