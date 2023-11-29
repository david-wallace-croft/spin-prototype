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

## History

- Initial release: 2023-11-25
