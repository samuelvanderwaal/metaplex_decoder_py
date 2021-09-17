# Simple Metaplex Metadata Decoder

Install the correct Python wheel for your Python version with `pip`:

```bash
pip install metaplex_decoder-0.1.0-cp39-cp39-manylinux_2_5_x86_64.manylinux1_x86_64.whl
```

Example Python usage:

```python
from metaplex_decoder import *
account_info = "..." # Base58 string
metadata = deserialize_metadata(account_info)
```

## Build With Rust 

[Install Rust](https://www.rust-lang.org/tools/install).

Use `virtualenv` to create a Python virtualenv environment and then activate it:

```bash
virtualenv env
source env/bin/activate
```

Install `maturin`:

```bash
pip install maturin
```

For Linux build in docker with the ManyLinux image:

```bash
docker run --rm -v $(pwd):/io konstin2/maturin build --release
```

For MacOSX:

```bash
maturin build
```

