# wit-adapter

This repository contains a DRAFT version of Cerbos EPDP WIT interface, specified in the `./wit/policy.wit`.
The interface uses rich types while the existing Cerbos EPDP interface, specified in the `./workspace/wit/host.wit`, manipulates strings.

`./workspace/justfile` is provided only for the reference. Its commands compile Cerbos ePDP (Wasm core module) and convert it to a Wasm component.
