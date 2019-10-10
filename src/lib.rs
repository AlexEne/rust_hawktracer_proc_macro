extern crate proc_macro;
extern crate rust_hawktracer_sys;
use proc_macro::TokenStream;

use rust_hawktracer_sys::ScopedTracepoint;

#[proc_macro_attribute]
pub fn hawktracer(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.into_iter().next().unwrap_or_else(||
        panic!("\n[rust_hawktracer] Please specify the profile tag: \n #[rust_hawktracer(tag)]\nfn my method() {\n...\n}\n")
    );
    let mut input = input.to_string();

    let string_to_insert = format!(
        r#"
            let __scoped_tracepoint_name_{} = concat!(stringify!({}), "\0");
            ScopedTracepoint::start_trace(__scoped_tracepoint_name_{}.as_ptr() as _);
            let __scoped_tracepoint_{} = ScopedTracepoint {{}};
        "#,
        args, args, args, args
    );

    if let Some(idx) = input.find("{") {
        input.insert_str(idx+1, &string_to_insert);
    }

    // panic!(input);

    input.parse().unwrap()
}
