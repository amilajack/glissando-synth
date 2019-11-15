# glissando-vst

## setup

```sh
npm config set access public
```

## build

```sh
wasm-pack build --target web --scope glissando --release
```

## release

```sh
wasm-pack publish
```