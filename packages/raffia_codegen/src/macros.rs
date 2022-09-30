macro_rules! emit {
    ($s:expr, $t:expr) => {{
        use crate::Emit;

        $s.emit(&$t)?;
    }};
}
