//! Complete example from the nested invocations section of the documentation.

use tempt::tempt;

trait IsNonnegative {
    fn is_nonnegative(self) -> bool;
}

tempt! {
    signed   [ self >= 0 ]
    unsigned [ true      ];

    #(
        tempt! {
              number_type    implementation
            [ i8          ][ #signed        ]
            [ i16         ][ #signed        ]
            [ u8          ][ #unsigned      ]
            [ u16         ][ #unsigned      ];

            #(
                impl IsNonnegative for #number_type {
                    fn is_nonnegative(self) -> bool {
                        #implementation
                    }
                }
            )*
        }
    )*
}

fn main() {
    assert!(!(-25i8).is_nonnegative());
    assert!(0i16.is_nonnegative());
    assert!(0u8.is_nonnegative());
}
