#[macro_export]
macro_rules! measure {
    ($tag:expr, $expr:expr) => {{
        let start = std::time::Instant::now();
        let ret = $expr;
        let end = start.elapsed();
        println!("{}.{}[s]", end.as_secs(), end.subsec_nanos());
        ret
    }};
}
