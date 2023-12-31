mkdir -p web_test # Make the directory if it doesn't exist
rm -fr web_test/* # -r to remove the folders, -f to silence if it doesn't exist.
cp index.html web_test
cargo build --example test --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir web_test --target web target/wasm32-unknown-unknown/release/examples/test.wasm
