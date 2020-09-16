# test_gnap_schema
a small test that uses [jsonschema](https://json-schema.org/) to check a json file against its schema (more specifically IETF gnap)

## dependancy
we use [jsonschema.rs](https://github.com/Stranger6667/jsonschema-rs). First test with draft7, will need to check with latest 2019-09 once this [issue](https://github.com/Stranger6667/jsonschema-rs/pull/45) is implemented.

## run test
cargo run

## manual check
this code is meant to be used as a validator within libraries, but one can also use an interactive [schema validator](https://www.jsonschemavalidator.net/) for a quick manual check.

## next step
prepare a list of test cases for gnap (working and failing) and automate.
