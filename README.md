<h1 align="center">Tempt</h1>
<h3 align="center">Templating for tokens</h3>
<div align="center">

[![docs.rs](https://img.shields.io/badge/docs.rs-tempt-58a78a?style=for-the-badge&logo=Docs.rs)](https://docs.rs/tempt)
&nbsp;&nbsp;&nbsp;
[![crates.io](https://img.shields.io/crates/v/tempt?style=for-the-badge&logo=Rust)](https://crates.io/crates/tempt)
&nbsp;&nbsp;&nbsp;
[![github](https://img.shields.io/badge/github-tempt-gray?style=for-the-badge&logo=GitHub&color=669bbc)](https://github.com/michaelni678/tempt)

</div>

**Tempt** adds templating for tokens with the [`tempt!`] macro.

# Setup

Add this to your `Cargo.toml`:

```toml
[dependencies]
tempt = "0.1.0"
```

# Usage

In the example below, Tempt is used to generate builder methods for `UserBuilder`.

```rust
use tempt::tempt;

struct User {
    // ...
}

#[derive(Default)]
struct UserBuilder {
    name: Option<String>,
    age: Option<u8>,
    email: Option<String>,
}

tempt! {
      field_name    field_type
    [ name       ][ String     ]
    [ age        ][ u8         ]
    [ email      ][ String     ];

    impl UserBuilder {
        #(
            fn #field_name(mut self, value: impl Into<#field_type>) -> Self {
                self.#field_name = Some(value.into());
                self
            }
        )*

        fn build(self) -> Option<User> {
            // ...
        }
    }
}
```

The [`tempt!`] macro expands to the following code:

```rust
impl UserBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn age(mut self, value: impl Into<u8>) -> Self {
        self.age = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn build(self) -> Option<User> {
        // ...
    }
}
```

The full example is available at [examples/usage.rs].

See the [documentation] for additional information.

[`tempt!`]: https://docs.rs/tempt/latest/tempt/macro.tempt.html
[examples/usage.rs]: https://github.com/michaelni678/tempt/blob/main/examples/usage.rs
[documentation]: https://docs.rs/tempt
