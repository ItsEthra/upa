# Upa
Macro that removes the hassle of creating long pointer chains.

# Installation
```toml
[dependencies]
upa = "0.1.0"
```

# Usage
```rust
struct Foo {
    bar: *mut Bar,
}

struct Bar {
    quz: *mut Quz,
}

struct Quz {
    tau: *mut Tau,
}

struct Tau {
    val: i32,
}

use upa::p;

fn main() {
    let mut t = Tau { val: 1337 };
    let mut q = Quz { tau: &mut t };
    let mut b = Bar { quz: &mut q };
    let f: *mut Foo = &mut Foo { bar: &mut b };

    unsafe {
        let wow = p!(f->bar->quz->tau->val);
        assert_eq!(wow, 1337);
    }
}
```