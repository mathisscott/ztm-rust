macro_rules! vek {
    (
        $( $el:expr ),+
        $(,)?
    ) => {{
        let mut vek = Vec::new();
        $( 
            vek.push($el);
        )+
        vek
    }};
}

fn main() {
    let v = vek![1,2,3,4,];
    dbg!(v);
}
