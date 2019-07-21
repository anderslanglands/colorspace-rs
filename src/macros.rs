#[macro_export]
macro_rules! vspd {
    ($($nm:expr => $v:expr,)*) => {{
        let mut samples = Vec::new();
        $(samples.push(crate::vspd::Sample::new($nm, $v));)*
        VSPD::new(samples)
    }};
    ($($nm:expr =>$v:expr),*) => {{
        let mut samples = Vec::new();
        $(samples.push(crate::vspd::Sample::new($nm, $v));)*
        VSPD::new(samples)
    }};
}

