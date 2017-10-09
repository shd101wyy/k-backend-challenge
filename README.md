# IMP language backend test  

Times are measured in `ms`.  

|   | sum $10^4$ | sum $10^5$ | sum $10^6$ | sum $10^7$ |
|---|---|---|---|---|
| RV krun (-O3)  | 3217   | 3501   | 7179  | 43757  |
| OCaml | 0 | 24 | 280 | 2872 |
| Rust (direct) | 16 | 207 | 1932 | 19503 |
| Rust (optimized) | 7 | 98 | 899 | 7776 |
| Rust (vector) | 15 | 194 | 1958 | 19695 |
| Rust (arena) | 16 | 198 | 1905 | 19949 |
| JavaScript (node.js) | 59 | 333 | 3061 | 29329 |

OCaml is fast!

---

To compile and run rust code, follow the instructions on http://doc.crates.io/ to install `rust` and `cargo`.  
Then run the following commands:  

```bash
cd rust_imp
cargo run --release
```

---

To run JavaScript code:  

```bash
cd javascript_imp
node imp.js
```

`imp.js` is the JavaScript code compiled from `imp.ts`.   