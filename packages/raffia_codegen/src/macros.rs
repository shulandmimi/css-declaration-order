macro_rules! emit {
    ($s:expr, $t:expr) => {{
        use crate::Emit;

        $s.emit(&$t)?;
    }};
}

macro_rules! serialize {
    ($s:expr, $t:expr) => {{
        let res = $s.serialize.translate($t);

        res
    }};
}

macro_rules! write_raw {
    ($s:expr, $t:expr) => {{
        let m = $t;
        if m.is_some() {
            $s.writer.write_raw(m.unwrap())
        }else {
            Ok(())
        }
    }};
}
