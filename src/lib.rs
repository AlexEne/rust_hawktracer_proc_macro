extern crate proc_macro;

#[cfg(feature = "profiling_enabled")]
extern crate rust_hawktracer_sys;
use proc_macro::TokenStream;

#[proc_macro_attribute]
#[cfg(feature = "profiling_enabled")]
pub fn hawktracer(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.into_iter().next().unwrap_or_else(||
        panic!("\n[rust_hawktracer] Please specify the profile tag: \n #[rust_hawktracer(tag)]\nfn my method() {\n...\n}\n")
    );
    let mut input = input.to_string();

    let string_to_insert = format!(
        r#"
            thread_local! {{
                static tracepoint_id_{}: u64 = add_cached_mapping(concat!(stringify!({}), "\0").as_ptr() as _);
            }};
            
            tracepoint_id_{}.with(|id| {{
                ScopedTracepoint::start_trace_id(*id);
            }});
            
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

#[proc_macro_attribute]
#[cfg(not(feature = "profiling_enabled"))]
pub fn hawktracer(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}
