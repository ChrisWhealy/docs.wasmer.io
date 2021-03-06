---
id: wasmer-js-client-transforming-wasi-modules
title: Wasmer-JS in the Browser
sidebar_label: Transforming WASI Modules
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/client/examples/transforming-wasi-modules)

# Transforming WASI Modules in the Browser

Irrespective of whether your JavaScript code runs on the client or the server, the statement shown below to [transform a WASI module](../../../wasmer-js-module-transformation) will always be needed.

## Setup Instructions

Please repeat the step-by-step instructions given in the [Hello World](../hello-world/wasmer-js-client-hello-world) example, but with the following changes:

1. Call your project `wasmer-js-transforming-wasi`
1. Download the WASM module [`clock_time_get.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/docs/wasmer-js/wasm_lib/clock_time_get.wasm) and store it in the `static` directory

## JavaScript Coding

The coding seen below is very similar to the coding used for the previous Hello World example &mdash; but with one very important difference!

Inside function `startWasiTask`, we fetch the WASM file contents and convert it to a `Uint8Array` as before, but then there is the additional line:

```JavaScript
const loweredWasmBytes = await lowerI64Imports(wasmBytes)
```

The call to function `lowerI64Imports` performs the all-important transformation that allows a JavaScript `BigInt` to be transferred to a WebAssembly `i64`.

Now that the interface has been transformed, we can instantiate the WebAssembly module and invoke it as before.

```JavaScript
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Imports
import { WASI }            from '@wasmer/wasi'
import { WasmFs }          from '@wasmer/wasmfs'
import { lowerI64Imports } from "@wasmer/wasm-transformer"

const wasmFilePath = './clock_time_get.wasm'  // Path to our WASI module

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Instantiate new WASI and WasmFs Instances
// IMPORTANT:
// Instantiating WasmFs is only needed when running in a browser.
// When running on the server, NodeJS's native FS module is assigned by default
const wasmFs = new WasmFs()

let wasi = new WASI({
  // Arguments passed to the WASM Module
  // The first argument is usually the filepath to the executable WASI module
  // we want to run.
  args: [wasmFilePath],

  // Environment variables that are accesible to the WASI module
  env: {},

  // Bindings used by the WASI instance (fs, path, etc...)
  bindings: {
    ...WASI.defaultBindings,
    fs: wasmFs.fs
  }
})

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Preserve the original console.log functionality
const consoleLog = console.log

// Implement our own console.log functionality that also writes to the DOM
console.log = (...args) =>
  (logTxt => {
    consoleLog(logTxt)
    document.body.appendChild(
      document.createTextNode(logTxt)
    )
  })
  (args.join(' '))

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Async Function to run our WASI module/instance
const startWasiTask =
  async () => {
    // Fetch our WASM File
    const response  = await fetch(wasmFilePath)
    const wasmBytes = new Uint8Array(await response.arrayBuffer())

    // IMPORTANT EXTRA STEP!
    // We must transform the WebAssembly module interface!
    const loweredWasmBytes = await lowerI64Imports(wasmBytes)

    // Instantiate the WebAssembly file
    let { instance } = await WebAssembly.instantiate(loweredWasmBytes, {
      wasi_unstable: wasi.wasiImport
    })

    wasi.start(instance)                      // Start the transformed WASI instance
    let stdout = await wasmFs.getStdOut()     // Get the contents of stdout
    console.log(`Standard Output: ${stdout}`) // Write stdout to the DOM
  }

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Everything starts here
startWasiTask()
```

On the both the browser screen and the JavaScript console, you should see the text `Done!`.

> ### Known Limitation
>
> This example is somewhat contrived because the WebAssembly module has been hard-coded to return the text string `Done!` rather than the time value from `clock_time_get`.
>
> Anything written to standard out should be a printable string followed by a carriage return character, not the raw `i32` value returned from `clock_time_get`.  Therefore, before being able to return the actual clock time, this WebAssembly module would additionally need to convert the raw `i32` value to a printable string before then writing it to standard out.

Next, let's look at handling input and output via WASI.
