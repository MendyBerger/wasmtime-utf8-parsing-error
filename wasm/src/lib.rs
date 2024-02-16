wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    path: "../",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: MyHost,
    },
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct MyHost;

impl Guest for MyHost {
    fn run() {
        my_func(Some(Options {
            a: "jpjjjjj".to_string(),
            b: vec!["".to_string()],
            c: Foo::new("pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp"),
        }));
    }
}
