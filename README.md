# wit-adapter

This repository contains a DRAFT version of [Cerbos EPDP WIT interface](wit/policy.wit).
The interface uses rich types while the existing [Cerbos EPDP interface](./epdp-wasm/wit/host.wit) manipulates strings.

[Cerbos](https://github.com/cerbos/cerbos) is an open-core, language-agnostic, scalable authorization solution that simplifies user permissions and authorization by writing context-aware access control policies for application resources.

Cerbos service is a policy decision point (PDP).
This repository deals with embedded Cerbos PDP (ePDP) - a free tier feature of [Cerbos Hub](https://hub.cerbos.cloud/).

ePDP is built from a set of policies as a self-contained program, which implements the same [CheckResources API](https://docs.cerbos.dev/cerbos/latest/api/#check-resources).
Technically, we transpile policies to Rust code and build the WebAssembly core module. A wall clock `now` function is the only dependency ePDP has on the host.
```rust
#[link(wasm_import_module = "env")]
extern "C" {
    #[link_name = "now"]
    fn now() -> u64;
}
```

The original use cases for ePDPs include single-page applications and Node.js applications.
In both cases, [Cerbos JavaScript SDK](https://github.com/cerbos/cerbos-sdk-javascript/blob/main/packages/embedded/README.md) hides the low-level details of interacting with ePDP API, which is effectively a function `fn check(input: String) -> String`, except that the SDK needs to allocate/deallocate memory for the strings.
The SDK also converts these strings (JSON serialization) to rich types, which are then exposed to the SDK client.

Here, we explored converting a Wasm core module binary to a Wasm component. We are not building a component for the required policies from the source code; we build a module, which then upgrades to a component.

The idea was to change the ePDP source code incrementally. The increment must not add much both in terms of the binary size and the contract.
We added `wit-bindgen` crate as a dependency and the following code fragment:
```rust
wit_bindgen::generate!({
    inline: r#"
        package cerbos-hub:epdp;

        interface authorization {
            check-wasi: func(s: string, now: u64) -> string;
        }
        world policy {
            export authorization;
        }
    "#
});
struct EPDP;

impl exports::cerbos_hub::epdp::authorization::Guest for EPDP {
    #[doc = r" check-wasi: func(ptr: u32, len: u32, now: s64) -> u64;"]
    fn check_wasi(s: _rt::String, now: u64) -> _rt::String {
        policy::check_with_now(&s, now as i64)
    }
}
export!(EPDP);
```
The interface reflects the module's API, with the exception that string lifting and memory management are done by the `wit-bindgen`.

The build steps remained the same: `cargo build --release --target wasm32-unknown-unknown`.
The produced core module is backward compatible with the SDK, but it can be upgraded to a Wasm component using `wasm-tools component new` command.
The only problem is satisfying the core module import of the `now` function.
We solved this by building a core module providing a stub function with the following command:
```bash
rustc -o env.wasm --target wasm32-unknown-unknown --crate-type cdylib --edition=2021 \
 -C opt-level=z -C lto -C codegen-units=1 -C debuginfo=0  - <<EOF
#[no_mangle]
pub unsafe extern "C" fn now() -> u64 { 0 }
EOF
```

Then to create an ePDP component from an ePDP core module: `wasm-tools component new <INPUT> -o <OUTPUT> --adapt ./env.wasm`

However, we want our ePDP to use rich types so we can skip the JSON serialization/deserialization step. To achive that, we created a generic component, `epdp-wasi-adapter`.

As per the following diagram, `epdp-wasi-adapter` exports a rich interface and imports a simple one from the `cerbos-hub:epdp` package.
For the build and composition steps, please refer to the [epdp-wasi-adapter/justfile](epdp-wasi-adapter/justfile).

![Components](Components.png)

The client application `http-proxy` starts the HTTP component, then calls the `epdp-wasi-adapter` and uses its rich interface.

## Running the example
Prerequisites:
1. Rust toolchain.
2. [wasmCloud](https://wasmcloud.com/docs/installation).
3. [justfile](https://github.com/casey/just).

Let's deploy all components to wasmCloud.
1. From the `http-proxy` directory, run `just deploy`.
2. From the `epdp-wasi-adapter` directory, run `just start`, then `just link` to link the components.
3. Run `curl 'http://localhost:8080?role=user'` to invoke the http-proxy. You should see the `Effect::Allow`. Change the role to get `Effect::Deny`.
