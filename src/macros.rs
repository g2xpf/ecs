macro_rules! indexed_tuple {
    ($(pub)? struct $ident:ident($($ty:ty),*)) => {
        $(pub)? struct $ident($($ty),*);

        indexed_tuple!(struct $ident($($ty),*; 0));
    };

    (struct $ident:ident($head:ty); $index:expr) => {
        index
    };

    (struct $ident:ident($head:ty, $($tail:ty),+))

}
