# CroftSoft Spin Prototype

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/spin-prototype/blob/main/LICENSE.txt

- A collection of Fermyon Spin example components
  - Rust code compiled to WebAssembly (Wasm) running as Serverless in the Cloud

## Usage

- Install the Fermyon Spin command line utility "spin"
  - https://github.com/fermyon/spin/releases/
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

## Test 0: Static File Server

- Demonstrates using a static file server
- http://localhost:3000/test0
  - Shows "test0"

## Test 1: Hello World

- Default application from the template http-rust
- http://localhost:3000/test1
  - Shows "Header spin-full-url: http://localhost:3000/test1"

## Test 2: CRUD REST API

- Create-Read-Update-Delete (CRUD) REST API example
- From a Key-Value store example
  - https://github.com/fermyon/spin/tree/main/examples/rust-key-value
- Also demonstrates how to use the Spin Router
  - https://developer.fermyon.com/spin/v2/rust-components#routing-in-a-component
- curl -i -X POST -d "ok!" localhost:3000/test2/test
  - Stores the value "ok!" for the key "test"
- curl -i --head localhost:3000/test2/test
  - A value is found for the key "test"
- curl -i -X GET localhost:3000/test2/test
  - Retrieves the value "ok!" for the key "test"
- curl -i -X DELETE localhost:3000/test2/test
  - Deletes the value "ok!" for the key "test"
- curl -i --head localhost:3000/test2/test
  - No value found for the key "test"

## Test 3: JSON I/O

- Demonstrates JSON input and output
- Post the name in a JSON object
  - Responds with a JSON object containing a message
```
curl -i -X POST \
  -d "{\"name\": \"World\"}" \
  localhost:3000/test3
```

## Test 4: AI LLM

- Demonstrates use of an Artificial Intelligence (AI) Large Language Model (LLM)
```
curl -i -X POST \
  -d "{\"characters\": [\"Gilgamesh\", \"Utnapishtim\"]}" \
  localhost:3000/test4
```

## Test 5: REST from Spin

- Demonstrates calling a REST API from a Spin Component
- http://localhost:3000/test5?name=World
  - Shows relayed output from Test 3

## TODO

- Authentication (AuthN) / Authorization (AuthZ) example
  - Might use OAuth 2.0 / OpenID Connect (OIDC)

## Links

- Fermyon Spin
  - https://www.fermyon.com/spin
- CroftSoft Advent of Spin Solutions
  - https://github.com/david-wallace-croft/advent-of-spin

## History

- Initial release: 2023-11-25
