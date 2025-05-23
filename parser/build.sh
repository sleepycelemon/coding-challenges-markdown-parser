rm -rf ../ui/static/pkg

wasm-pack build --target web

mv pkg ../ui/static


