# Deno Core 0.300.0 API Research Report

**Date:** 2026-02-22 21:19
**Research Focus:** deno_core 0.300 Rust crate - Extension system, ops registration, JsRuntime API

---

## 1. Extension Definition - `deno_core::extension!` Macro

### Syntax Overview
The `extension!` macro replaces the old `Extension::builder().ops().build()` pattern. It's a compile-time declarative macro that defines Deno extensions combining Rust ops and JavaScript modules.

### Complete Macro Signature
```rust
deno_core::extension!(
    extension_name,
    deps = [dependency1, dependency2],
    ops = [op1, op2, op3],
    esm = ["module1.ts", "module2.js"],
    lazy_loaded_esm = ["lazy_module.ts"],
    state = |state, options| {
        // Initialize extension state
        state.put::<MyStateType>(my_state);
    },
    config = MyConfigType,
    parameters = [PARAM: ParamType],
    options = { my_option: OptionType },
    middleware = |op| {
        // Middleware transformation for ops
        op
    },
    docs = "Documentation string"
);
```

### Real-World Example (from deno_http)
```rust
#[cfg(not(feature = "default_property_extractor"))]
deno_core::extension!(
    deno_http,
    deps = [deno_web, deno_net, deno_fetch, deno_websocket],
    parameters = [ HTTP: HttpPropertyExtractor ],
    ops = [
        op_http_accept,
        op_http_write,
        op_http_headers,
        op_http_shutdown,
        // ... 30+ operations
    ],
    esm = ["00_serve.ts", "01_http.js", "02_websocket.ts"],
    options = { options: Options },
    state = |state, options| {
        state.put::<Options>(options.options);
    }
);
```

### Key Parameters Explained
- **deps**: List of other extensions this one depends on
- **ops**: Vector of operation declarations (Vec<OpDecl>)
- **esm/lazy_loaded_esm**: Embedded JavaScript/TypeScript modules
- **state**: Closure initializing OpState with custom data structures
- **config**: Configuration type passed at runtime
- **parameters**: Generic parameters with type constraints
- **middleware**: Optional transformation function for ops
- **docs**: Documentation string for the extension

**Status:** This is the modern replacement for old-style `Extension::builder()`. No need to manually call `.ops()` or `.build()` anymore.

---

## 2. Ops Registration - From `op_name::decl()` to `#[op2]`

### Old Pattern (Deprecated)
```rust
// Old way - manually declaring op functions with decl()
op_sum::decl(),
op_read_file::decl(),
op_write_file::decl(),
```

### New Pattern - #[op2] Attribute Macro
All ops now use the `#[op2]` attribute. The macro name `op2` indicates it's "the in-progress replacement for `#[op]`."

**Purpose:** Provides "an extremely fast V8->Rust interface layer" with automatic serialization/deserialization.

### Basic Synchronous Op
```rust
#[op2]
fn op_sum(#[buffer] arr: &[f64]) -> f64 {
    arr.iter().sum()
}
```

Called from JavaScript as:
```javascript
const sum = Deno.core.ops.op_sum(new Float64Array([1, 2, 3]));
console.log(sum); // 6
```

### Op with SMI (Small Integer) Parameter
```rust
#[op2]
fn op_http_headers(
    state: &mut OpState,
    #[smi] rid: u32,  // SMI = small integer (optimized)
) -> Result<Vec<(ByteString, ByteString)>, HttpError> {
    let stream = state.resource_table.get::<HttpStreamReadResource>(rid)?;
    Ok(stream.get_headers())
}
```

### Op with String Parameters
```rust
#[op2]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}
```

The `#[string]` attribute on the function indicates return type is String, and `#[string]` on parameters indicates those args are Strings.

### Op with Buffer Parameters
```rust
#[op2]
async fn op_http_write(
    state: Rc<RefCell<OpState>>,
    #[smi] rid: ResourceId,
    #[buffer] buf: JsBuffer,  // Direct buffer from JavaScript
) -> Result<(), HttpError> {
    let stream = state.borrow().resource_table.get::<HttpStreamWriteResource>(rid)?;
    stream.write_all(&buf).await?;
    Ok(())
}
```

**Parameter Attributes:**
- `#[smi]` - Small integer (optimized for small ints)
- `#[string]` - String parameter/return
- `#[buffer]` - JavaScript buffer (JsBuffer type)
- `#[state]` - Custom state type extraction from OpState
- No attribute - Serializable via serde_v8 (default)

---

## 3. Async Ops - `#[op2(async)]` and `#[op2(async(deferred))]`

### Basic Async Op with OpState
```rust
#[op2(async)]
async fn op_http_accept(
    state: Rc<RefCell<OpState>>,
    #[smi] rid: ResourceId,
) -> Result<Option<NextRequestResponse>, HttpError> {
    let conn = state.borrow().resource_table.get::<HttpConnResource>(rid)?;
    match conn.accept().await {
        Ok(Some((read_stream, write_stream, method, url))) => {
            // Store streams, return response
            Ok(Some(NextRequestResponse { /* ... */ }))
        }
        Ok(None) => Ok(None),
        Err(err) => Err(err.into()),
    }
}
```

**Critical:** In async ops:
- Must use `Rc<RefCell<OpState>>` (not `&mut OpState`)
- Must use `.borrow()` or `.borrow_mut()` to access state
- Cannot pass `&mut OpState` directly (would cause borrow checker panic across await point)
- Mutable OpState is only supported in synchronous ops

### Async Op with Buffer
```rust
#[op2(async)]
async fn op_write_file(
    #[string] path: String,
    #[buffer] data: JsBuffer,
) -> Result<(), AnyError> {
    tokio::fs::write(path, &data[..]).await?;
    Ok(())
}
```

### Async Op without OpState
```rust
#[op2(async)]
#[string]
async fn op_fetch_url(#[string] url: String) -> Result<String, AnyError> {
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    Ok(body)
}
```

These ops don't need OpState if they don't access resources.

### Deferred Async Ops
```rust
#[op2(async(deferred))]
async fn op_expensive_computation(#[buffer] data: &[u8]) -> Result<Vec<u8>, AnyError> {
    // Heavy computation here
    Ok(compute_result(data).await)
}
```

The `deferred` variant allows the runtime to schedule computation more flexibly.

---

## 4. JsRuntime::execute_script - Method Signature & IntoModuleCodeString

### Method Signature
```rust
pub fn execute_script(
    &mut self,
    name: &'static str,
    source_code: impl IntoModuleCodeString,
) -> Result<Global<Value>, Error>
```

### Parameters Explained
- **name**: String identifier for the script (must be valid 7-bit ASCII)
  - Examples: `"/some/file/path.js"`, `"[native code]"`, `"<anon>"`
  - Used for stack traces and debugging

- **source_code**: Any type implementing `IntoModuleCodeString` trait
  - Accepts: `String`, `&'static str`, `FastString`, `ascii_str!()` output
  - Returns: `Global<Value>` handle to the last expression evaluated

### Return Type
Returns `Result<Global<Value>, Error>`:
- **Ok(Global<Value>)**: Last expression result
- **Err(Error)**: JavaScript exception or runtime error

### Usage Examples

#### Using ascii_str! for Best Performance
```rust
use deno_core::ascii_str;

let result = runtime.execute_script(
    "/app/init.js",
    ascii_str!("console.log('Hello'); 42"),
)?;
```

**Why ascii_str!?** Compile-time validated 7-bit ASCII, stored optimally for V8. Fastest option. No runtime overhead.

#### Using Static String
```rust
let result = runtime.execute_script(
    "[runjs:runtime.js]",
    include_str!("./runtime.js"),  // &'static str
)?;
```

#### Using Owned String (format! macro)
```rust
let code = format!("const PI = {}; PI * 2", std::f64::consts::PI);
let result = runtime.execute_script(
    "<computed>",
    code,  // Owned String
)?;
```

#### Calling Custom Ops from Script
```rust
let script = ascii_str!(
    "(() => { return Deno.core.ops.op_sum([1, 2, 3]) })()"
);
let result = runtime.execute_script("[main]", script)?;
// result is a Global<Value> with the op's return value
```

**Note:** `execute_script` executes **non-module JavaScript**. For ES modules, use `load_main_module()` or `load_side_module()` instead.

---

## 5. JsRuntime::resolve_value - Deprecated Method

### Original Signature (Deprecated)
```rust
pub async fn resolve_value(
    &mut self,
    global: Global<Value>,
) -> Result<Global<Value>, Error>
```

### Status: DEPRECATED
This method is **marked as deprecated**. Recommended replacements:

### Modern Replacement Methods

#### resolve() - Primary Replacement
```rust
pub fn resolve(
    &mut self,
    promise: Global<Value>,
) -> impl Future<Output = Result<Global<Value>, Error>>
```

Usage:
```rust
let promise = runtime.execute_script("[main]", ascii_str!("Promise.resolve(42)"))?;
let resolved = runtime.resolve(promise).await?;
```

#### scoped_resolve() - For HandleScope Context
```rust
pub fn scoped_resolve(
    scope: &mut HandleScope<'_>,
    promise: Global<Value>,
) -> impl Future<Output = Result<Global<Value>, Error>>
```

#### call_and_await() - For Functions
```rust
pub async fn call_and_await(
    &mut self,
    function: &Global<Function>,
) -> Result<Global<Value>, Error>
```

#### call_with_args_and_await() - For Functions with Arguments
```rust
pub async fn call_with_args_and_await(
    &mut self,
    function: &Global<Function>,
    args: &[Global<Value>],
) -> Result<Global<Value>, Error>
```

**Why the change?** The newer methods provide:
- Clearer intent (e.g., `resolve()` vs `call_and_await()`)
- Better performance characteristics
- Scoped variants for unsafe V8 handle management
- Consistency with overall API redesign

---

## 6. Parameter Type - Global<Value> Ownership

### resolve_value/resolve Signature Detail
```rust
pub fn resolve(
    &mut self,
    promise: Global<Value>,  // Takes OWNED Global<Value>
) -> impl Future<Output = Result<Global<Value>, Error>>
```

### Key Points
- **Takes ownership:** Moves `Global<Value>` into the method
- **Returns owned:** Produces `Result<Global<Value>, Error>`
- **Not borrowed:** Does NOT take `&Global<Value>` reference
- **Why?** V8 handle semantics: `Global` represents owned V8 handle, cannot be safely borrowed

### Example with Handle Scope
```rust
let mut scope = runtime.handle_scope();
let local_value: Local<Value> = /* ... */;
let global_value = Global::new(&mut scope, local_value);

// Must pass owned value, not reference
let resolved = runtime.resolve(global_value).await?;
// global_value is now moved into runtime.resolve()
```

---

## 7. Custom State in #[op2] without OpState Parameter

### Option 1: Extract Custom State Type Using #[state]
```rust
#[derive(Clone)]
struct CustomData {
    counter: usize,
}

#[op2]
fn op_increment(#[state] custom: CustomData) -> usize {
    // state.get::<CustomData>() is called automatically
    custom.counter + 1
}
```

Used in extension:
```rust
deno_core::extension!(
    my_ext,
    ops = [op_increment],
    state = |state, _| {
        state.put::<CustomData>(CustomData { counter: 0 });
    }
);
```

### Option 2: No State Parameter
```rust
#[op2]
fn op_hello_world() -> String {
    "Hello, World!".to_string()
}
```

Pure functions that don't access runtime state work fine.

### Option 3: Return Serializable Types
```rust
#[op2]
fn op_create_data() -> Result<serde_json::Value, AnyError> {
    Ok(serde_json::json!({
        "name": "test",
        "value": 42
    }))
}
```

---

## 8. FastString and ascii_str! for Code Strings

### FastString Type
Optimized enum for V8 string creation:
```rust
pub enum FastString {
    // Owned UTF-8 string
    Owned(String),
    // Static 7-bit ASCII string
    Static(&'static str),
    // Lazy string builder
    // ... other variants
}
```

### ascii_str! Macro
```rust
use deno_core::ascii_str;

let code = ascii_str!("console.log('hello'); 42");
runtime.execute_script("[main]", code)?;
```

**Guarantees at compile-time:**
- String contains only 7-bit ASCII
- Stored as static data in binary
- Zero runtime validation overhead
- Fastest possible V8 string creation

### When to Use Each

| Type | Use Case | Performance |
|------|----------|-------------|
| `ascii_str!()` | Static code literals | Best |
| `include_str!()` wrapped in ascii_str! | Static file content | Best |
| `&'static str` | Already-static strings | Good |
| `String` from `format!()` | Dynamic code generation | Acceptable |
| `FastString::Owned()` | Manual optimization | Manual |

### Real Example
```rust
// Best: compile-time validated ASCII
runtime.execute_script("[init]", ascii_str!(
    "globalThis.appName = 'MyApp'; \
     globalThis.version = 1;"
))?;

// Good: static file
runtime.execute_script(
    "[runtime]",
    include_str!("./runtime.js")
)?;

// Acceptable: dynamic generation
let config = format!("const CONFIG = {};", json_config);
runtime.execute_script("[config]", config)?;
```

---

## 9. Complete Working Example

### Extension Definition with Multiple Op Types
```rust
use deno_core::{ascii_str, extension, op2};
use std::rc::Rc;
use std::cell::RefCell;

#[op2]
fn op_add(#[number] a: f64, #[number] b: f64) -> f64 {
    a + b
}

#[op2]
fn op_get_data(state: &mut OpState) -> String {
    state.get::<MyData>().value.clone()
}

#[op2(async)]
async fn op_fetch(#[string] url: String) -> Result<String, AnyError> {
    let response = reqwest::get(&url).await?;
    Ok(response.text().await?)
}

deno_core::extension!(
    my_extension,
    ops = [op_add, op_get_data, op_fetch],
    state = |state, _| {
        state.put::<MyData>(MyData {
            value: "initialized".to_string(),
        });
    }
);

#[derive(Clone)]
struct MyData {
    value: String,
}

// Usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![my_extension::init_ops()],
        ..Default::default()
    });

    // Execute script
    runtime.execute_script(
        "[main]",
        ascii_str!(
            "const sum = Deno.core.ops.op_add(10, 20); \
             console.log('Sum:', sum);"
        ),
    )?;

    runtime.run_event_loop(PollEventLoopOptions::default()).await?;
    Ok(())
}
```

---

## 10. Op2 Macro Features Summary

### Attribute Variants
- `#[op2]` - Synchronous op
- `#[op2(async)]` - Asynchronous op with eager polling
- `#[op2(async(lazy))]` - Lazy async (deferred scheduling)
- `#[op2(async(deferred))]` - Deferred async scheduling
- `#[op2(fast)]` - Fastcall optimization enabled

### Parameter Attributes
- `#[smi]` - Small integer (i32/u32, optimized)
- `#[string]` - String type
- `#[buffer]` - JavaScript buffer
- `#[state]` - Custom state extraction
- `#[number]` - Numeric type (f64, i32, etc.)
- None - Default serialization via serde_v8

### Error Handling
All ops can return `Result<T, E>` where `E: Into<anyhow::Error>`:
```rust
#[op2]
fn op_risky() -> Result<String, std::io::Error> {
    std::fs::read_to_string("missing.txt")
        .map(|_| "OK".to_string())
}
```

Errors automatically convert to JavaScript exceptions.

---

## Summary of Key Changes (Old → New)

| Aspect | Old Pattern | New Pattern |
|--------|------------|------------|
| Extension Definition | `Extension::builder().ops().build()` | `extension!` macro |
| Op Declaration | `op_name::decl()` manual | `#[op2]` attribute |
| Async Ops | Callback-based | `async fn` with `#[op2(async)]` |
| State Access | Direct mutable ref | `Rc<RefCell<OpState>>` in async, `&mut` in sync |
| Code Execution | Various patterns | `execute_script(name, impl IntoModuleCodeString)` |
| Promise Awaiting | `resolve_value()` | `resolve()`, `call_and_await()`, etc. |
| String Optimization | Manual | `ascii_str!()` macro for compile-time validation |

---

## Sources

- [deno_core 0.300.0 Documentation](https://docs.rs/deno_core/0.300.0/deno_core/)
- [deno_core extension! Macro](https://docs.rs/deno_core/0.300.0/deno_core/macro.extension.html)
- [deno_core #[op2] Macro](https://docs.rs/deno_core/0.300.0/deno_core/attr.op2.html)
- [deno_core JsRuntime Methods](https://docs.rs/deno_core/0.300.0/deno_core/struct.JsRuntime.html)
- [deno_core FastString](https://docs.rs/deno_core/latest/x86_64-apple-darwin/deno_core/struct.FastString.html)
- [Deno HTTP Extension Example](https://github.com/denoland/deno/blob/main/ext/http/lib.rs)
- [deno_core Hello World Example](https://github.com/denoland/deno_core/blob/main/core/examples/hello_world.rs)
- [The Internals of Deno - Op Registration](https://choubey.gitbook.io/internals-of-deno/import-and-ops/5.6-registration-of-ops)
- [The Internals of Deno - JsRuntime](https://choubey.gitbook.io/internals-of-deno/foundations/jsruntime)
- [Roll Your Own JavaScript Runtime (Deno Blog)](https://deno.com/blog/roll-your-own-javascript-runtime)
- [Roll Your Own JavaScript Runtime, pt. 2](https://deno.com/blog/roll-your-own-javascript-runtime-pt2)

---

## Unresolved Questions

1. **Fastcall optimization details**: The `#[op2(fast)]` variant and fastcall compatibility requirements are mentioned but not fully documented in available sources.

2. **Exact optimization thresholds**: When does the macro switch between slow and fastcall implementations? What are the performance crossover points?

3. **V8 handle lifecycle**: What is the exact lifetime contract for `Global<Value>` returned from `execute_script()` and how it interacts with GC?

4. **Version 0.300 specific changelog**: Could not locate specific release notes for deno_core 0.300.0 changelog—current versions are in 0.38x range. Version 0.300 may be very old or not yet released at documentation time.

5. **Middleware system details**: The `middleware` parameter in `extension!` macro is mentioned but lacks comprehensive documentation.
