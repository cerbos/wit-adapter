# wit-adapter

This repository contains a DRAFT version of [Cerbos EPDP WIT interface](wit/policy.wit).

The interface uses rich types while the existing [Cerbos EPDP interface](./workspace/wit/host.wit) manipulates strings.

`./workspace/justfile` is provided only for the reference. It compiles Cerbos ePDP as a core module, generates a stub for a module import, and creates a component.

# Diagram
The `cerbos-adapter` component imports the interface of the Cerbos ePDP component, encoded from the (Wasm Core module).
![Components](Components.png)
