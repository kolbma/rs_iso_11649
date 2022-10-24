[![Rust](https://github.com/kolbma/rs_iso_11649/actions/workflows/rust.yml/badge.svg)](https://github.com/kolbma/rs_iso_11649/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/iso_11649)](https://crates.io/crates/iso_11649)
[![docs](https://docs.rs/iso_11649/badge.svg)](https://docs.rs/iso_11649)

## Description

The Creditor Reference (also called the Structured Creditor Reference)
is an international business standard based on ISO 11649, implemented at
the end of 2008.

The Creditor Reference was first implemented within the SEPA rulebook 3.2.

A vendor adds the Creditor Reference to its invoices. When a customer pays
the invoice, the company writes the Creditor Reference instead of the
invoice number in the message section, or places a Creditor Reference
field in its payment ledger.

When the vendor receives the payment, it can automatically match the
remittance information to its Accounts Receivable system.

## Usage

Put the crate to your `Cargo.toml` dependencies and set latest 
available version...

```toml
[dependencies]
iso_11649 = "*"
```

The library documentation is available at [docs.rs](https://docs.rs/iso_11649).

## Licenses

You can choose between __[MIT License](https://opensource.org/licenses/MIT)__ or __[Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0)__.

### MIT License

Copyright (c) 2022 Markus Kolb

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

### Apache License 2.0

Copyright 2022 Markus Kolb

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
