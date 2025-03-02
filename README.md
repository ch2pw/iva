# Iva

The video editor alternative to AviUtl.   
Work in progress...

```rs
"avi".chars().rev().collect::<String>()
```

## Development
```sh
cd src-wasm
wasm-pack build --target web --dev
cd ..

pnpm tauri dev
```
