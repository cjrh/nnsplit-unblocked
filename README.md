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
(venv) $ maturin build
```
