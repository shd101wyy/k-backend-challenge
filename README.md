I wrote the imp backend code in `rust` and `ocaml` without any optimizations.  

It's so hard to get my code compiled successfully in `rust`, so I give up optimizing it.  

Times are measured in `ms`.  

|   | sum $10^4$ | sum $10^5$ | sum $10^6$ | sum $10^7$ |
|---|---|---|---|---|
| RV K (-O3)  | 3217   | 3501   | 7179  | 43757  |
| OCaml | 0 | 24 | 280 | 2872 |
| Rust | 31 | 313 | 3087 | 30268 |

OCaml is fast!

---

To compile and run rust code, follow the instructions on http://doc.crates.io/ to install `rust` and `cargo`.  
Then run the following commands:  

```bash
cd rust_imp
cargo run --release
```
