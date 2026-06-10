//! Complete example from the README and usage documentation.

use tempt::tempt;

struct User {
    name: String,
    age: u8,
    email: String,
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
            Some(User {
                #(
                    #field_name: self.#field_name?,
                )*
            })
        }
    }
}

fn main() {
    let user = UserBuilder::default()
        .name("John Smith")
        .age(30)
        .email("john.smith@example.com")
        .build()
        .expect("failed to build user");

    assert_eq!(user.name, "John Smith");
    assert_eq!(user.age, 30);
    assert_eq!(user.email, "john.smith@example.com");
}
