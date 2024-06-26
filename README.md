# nnsplit-unblocked
A python wrapper for nnsplit 0.5.9 that runs in newer python versions

## Overview

We have code stuck on nnsplit 0.5.9, the official wheel for which is
only available for Python 3.10. This package is a wrapper around
nnsplit 0.5.9 that runs in newer python versions.

It uses the abi3 stable ABI to load the nnsplit shared library, so
it should work in any python version that has the abi3 stable ABI.

## Setup

You need Rust and a newer Python, like 3.12.

From scratch,

```bash
$ python3.12 -m venv venv
$ source venv/bin/activate
(venv) $ pip install -r requirements.txt
(venv) $ cargo build
```

If all the above completes successfully, move onto the developer
workflow which describes the typical actions.

## Developer workflow

Typically, you first use `maturin develop` to build the package and 
install it in the current environment. Then you can run the tests.

```bash
(venv) $ maturin develop
(venv) $ pytest
```

When you want to build your final wheel, you can use `maturin build`.

```bash
(venv) $ maturin build --release
$ maturin build --release
🔗 Found pyo3 bindings with abi3 support for Python ≥ 3.10
🐍 Not using a specific python interpreter
📡 Using build options features from pyproject.toml
   Compiling proc-macro2 v1.0.86
   Compiling unicode-ident v1.0.12
   Compiling autocfg v1.3.0
   Compiling once_cell v1.19.0

   ...snip...

   Compiling tract-core v0.15.8
   Compiling tract-nnef v0.15.8
   Compiling tract-hir v0.15.8
   Compiling tract-onnx-opl v0.15.8
   Compiling nnsplit v0.5.9
   Compiling nnsplit-unblocked v0.1.0 (/home/caleb/Documents/repos/nnsplit-unblocked)
    Finished `release` profile [optimized] target(s) in 4m 21s
📦 Built wheel for abi3 Python ≥ 3.10 to /home/caleb/Documents/repos/nnsplit-unblocked/target/wheels/nnsplit_unblocked-0.1.0-cp310-abi3-manylinux_2_35_x86_64.whl
```

The wheel, as shown above, will be in the `target/wheels` directory.
