// These are attributes of the implicit crate. Really this just needs to parse
// for completeness since .rs files linked from .rc files support this
// notation to specify their module's attributes
#[attr1 = "val"];
#[attr2 = "val"];
#[attr3];
#[attr4(attr5)];

// Special linkage attributes for the crate
#[link(name = "std",
       vers = "0.1",
       uuid = "122bed0b-c19b-4b82-b0b7-7ae8aead7297",
       url = "http://rust-lang.org/src/std")];

// These are are attributes of the following mod
#[attr1 = "val"]
#[attr2 = "val"]
mod test_first_item_in_file_mod { }

mod test_single_attr_outer {

    #[attr = "val"]
    const x: int = 10;

    #[attr = "val"]
    fn f() { }

    #[attr = "val"]
    mod mod1 { }

    #[attr = "val"]
    #[abi = "cdecl"]
    extern mod rustrt { }
}

mod test_multi_attr_outer {

    #[attr1 = "val"]
    #[attr2 = "val"]
    const x: int = 10;

    #[attr1 = "val"]
    #[attr2 = "val"]
    fn f() { }

    #[attr1 = "val"]
    #[attr2 = "val"]
    mod mod1 { }

    #[attr1 = "val"]
    #[attr2 = "val"]
    #[abi = "cdecl"]
    extern mod rustrt { }

    #[attr1 = "val"]
    #[attr2 = "val"]
    type t = {x: int};
}

mod test_stmt_single_attr_outer {

    fn f() {

        #[attr = "val"]
        const x: int = 10;

        #[attr = "val"]
        fn f() { }

        /* FIXME: Issue #493
        #[attr = "val"]
        mod mod1 {
        }

        #[attr = "val"]
        #[abi = "cdecl"]
        extern mod rustrt {
        }
        */
    }
}

mod test_stmt_multi_attr_outer {

    fn f() {

        #[attr1 = "val"]
        #[attr2 = "val"]
        const x: int = 10;

        #[attr1 = "val"]
        #[attr2 = "val"]
        fn f() { }

        /* FIXME: Issue #493
        #[attr1 = "val"]
        #[attr2 = "val"]
        mod mod1 {
        }

        #[attr1 = "val"]
        #[attr2 = "val"]
        #[abi = "cdecl"]
        extern mod rustrt {
        }
        */
    }
}

mod test_attr_inner {

    mod m {
        // This is an attribute of mod m
        #[attr = "val"];
    }
}

mod test_attr_inner_then_outer {

    mod m {
        // This is an attribute of mod m
        #[attr = "val"];
        // This is an attribute of fn f
        #[attr = "val"]
        fn f() { }
    }
}

mod test_attr_inner_then_outer_multi {
    mod m {
        // This is an attribute of mod m
        #[attr1 = "val"];
        #[attr2 = "val"];
        // This is an attribute of fn f
        #[attr1 = "val"]
        #[attr2 = "val"]
        fn f() { }
    }
}

mod test_distinguish_syntax_ext {

    use std;

    fn f() {
        fmt!{"test%s", ~"s"};
        #[attr = "val"]
        fn g() { }
    }
}

mod test_other_forms {
    #[attr]
    #[attr(word)]
    #[attr(attr(word))]
    #[attr(key1 = "val", key2 = "val", attr)]
    fn f() { }
}

mod test_foreign_items {
    #[abi = "cdecl"]
    extern mod rustrt {
        #[attr];

        #[attr]
        fn get_task_id() -> libc::intptr_t;
    }
}

mod test_literals {
    #[str = "s"];
    #[char = 'c'];
    #[int = 100];
    #[uint = 100u];
    #[mach_int = 100u32];
    #[float = 1.0];
    #[mach_float = 1.0f32];
    #[nil = ()];
    #[bool = true];
    mod m { }
}

fn test_fn_inner() {
    #[inner_fn_attr];
}

fn main() { }

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
