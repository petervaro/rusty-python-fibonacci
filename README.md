# Rusty Fibonacci

This is a test crate to demonstrate how easy it is to work with Rust from
Python thanks to [`pyo3`][100].   It defines a new Python class called
`Fibonacci` which implements Python's iterator protocol and generates the
fibonacci series up until [`std::u64::MAX`][110] lazily.



## Installation and test

1. Install `rustup` (with your OS's package manager or via [this script][120])
2. Install nightly Rust which is need for `pyo3` (for now)
   ```bash
   $ cd path/to/rusy-python
   $ rustup override set nightly
   ```
3. Compile extension module:
   ```bash
   $ cargo build --release
   ```
4. Create a symlink for `test.py` to work:
   ```bash
   $ ln -s target/release/librusty_fibonacci.so rusty_fibonacci.so
   ```
5. Run `test.py`:
   ```bash
   $ python test.py
   (0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610)
   ```

## License

Copyright &copy; 2019 **Peter Varo**

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program, most likely a file in the root directory, called 'LICENSE'.
If not, see [http://www.gnu.org/licenses][200].

<!-- links -->
[100]: https://crates.io/crates/pyo3
[110]: https://doc.rust-lang.org/std/u64/constant.MAX.html
[120]: https://rustup.rs
[200]: http://www.gnu.org/licenses
