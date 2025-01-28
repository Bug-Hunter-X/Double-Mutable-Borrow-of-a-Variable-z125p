This repository demonstrates a subtle bug in Rust related to mutable borrows. The `bug.rs` file contains code that compiles but results in undefined behavior because it violates Rust's borrowing rules.  The `bugSolution.rs` file shows how to correctly handle the situation.