macro_rules! emit {
    ($s:expr, $t:expr) => {{
        use crate::Emit;

        $s.emit(&$t)?;
    }};
}

macro_rules! serialize {
    ($s:expr, $t:expr) => {{
        let raw = $s.serialize.translate($t);

        if let Some(val) = raw {
            val
        } else {
            String::from("")
        }
    }};
}
