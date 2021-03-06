---
id: introduction
title: Introduction
sidebar_label: Introduction
---

![Wasmer Logo](/img/wasmer-logo.svg)

Hello! Welcome to the Wasmer Documentation!

[Wasmer](https://wasmer.io/) is an open-source runtime for executing WebAssembly on the Server. Our mission is make all software universally available. 

For an overview of WebAssembly, and what WebAssembly is, [take a look here](https://webassembly.org/).

## Background

By design, the environment within which a WebAssembly module runs is completely isolated (or "sand-boxed") from the native functionality of the underlying host system. This means that by default, WASM modules are designed to perform nothing more than pure computation. Consequently, access to OS-level resources such as file descriptors, network sockets, the system clock, and random numbers is not normally possible from WASM.

However, there are many cases in which WebAssembly modules need to do more than perform pure computation; they must also interact with native "OS" functionality.

Wasmer therefore provides two key services:

1. It allows you to run WASM modules either standalone, or (by means of our language integration tools) embedded within other language runtimes such as C/C++, Python, and Rust. This enables extremely portable binaries that can run unmodified in any Wasmer-supported OS (Linux, macOS, Windows and FreeBSD).
1. By means of an Application Binary Interface (ABI) such as [`WASI`](https://github.com/webassembly/wasi) or [`Emscripten`](https://github.com/emscripten-core/emscripten), it acts as a secure bridge for WASM modules to interact with native "OS" functionality

> ## IMPORTANT  
> The term "OS" used above is in quotes to indicate that the native function being called might not actually be provided by the host's operating system.
>
> In reality, native functions always belong to the host environment within which the WebAssembly module is being run, and this could be either a host language's runtime environment (for example, JavaScript, Python or Ruby), or it might be the actual operating system.
>
> Either way though, from a WebAssembly point of view, we don't need to care too much about this detail. All we need to know is that:
> 
> * The host environment provides "imported" functions for the WebAssembly module
> * Via Wasmer's included ABIs, WebAssembly modules can have access to a set of operating-system-like functions with varying levels of sandboxing

## Projects

We also have other projects such as:

1. The [WAPM (WebAssembly Package Manager)](https://wapm.io/)
1. [Wasmer-JS](https://github.com/wasmerio/wasmer-js)
1. The [WebAssembly Shell](https://webassembly.sh/)

to name but a few...

Also, for the latest blogs on Wasmer features and developments, check out our [Medium site](https://medium.com/wasmer).

## Tutorials

If you would like to see tutorials, examples, or reference API documentation about a specific Wasmer project, please use the sidebar to the left, or the search bar at the top of this page in the header, or take a look at the list below:

* [Wasmer Runtime Documentation](/runtime/runtime)
* [Wasmer JS Documentation](/wasmer-js/wasmer-js)
* [WAPM Documentation](/wapm/wapm)
