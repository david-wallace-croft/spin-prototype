# Spin Prototype

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/spin-prototype/blob/main/LICENSE.txt

- My first Fermyon Spin application

## Usage

- Install the Fermyon Spin command line utility "spin"
  - https://developer.fermyon.com/spin/v2/install
- spin build
- spin up
  - If running on Windows, you might have to do some workarounds
    - See https://github.com/fermyon/spin/issues/2112
    - Build spin-fileserver in a sibling directory
      - cd ../spin-fileserver
      - cargo build --release
      - cd ../spin-prototype
    - Update the path to spin_static_fs.wasm in spin.toml
    - spin up --direct-mounts

## Test 0
- Demonstrates using a static file server
- http://localhost:3000/test0
  - Shows "test0"

## Test 1

- Default application from the template http-rust
- http://localhost:3000/test1
  - Shows "Hello, Fermyon"

## Test 2

- From a Key-Value store example
  - https://github.com/fermyon/spin/tree/main/examples/rust-key-value
- curl -i -X POST -d "ok!" localhost:3000/test2/test
  - Stores the value "ok!" for the key "test"
- curl -i -X GET localhost:3000/test2/test
  - Retrieves the value "ok!" for the key "test"
- curl -i -X DELETE localhost:3000/test2/test
  - Deletes the value "ok!" for the key "test"
- curl -i -X GET localhost:3000/test2/test
  - Fails to retrieve the deleted value for the key "test"

## Test 3

- Demonstrates JSON input and output
- Post the name in a JSON object
  - Responds with a JSON object containing a message
```
curl -i -X POST \
  -d "{\"name\": \"World\"}" \
  localhost:3000/test3
```

## Test 4

- Demonstrates use of a Large Language Model (LLM)
```
curl -i -X POST \
  -d "{\"characters\": [\"Gilgamesh\", \"Utnapishtim\"]}" \
  localhost:3000/test4
```

## Links

- Fermyon Spin
  - https://www.fermyon.com/spin

## History

- Initial release: 2023-11-25
