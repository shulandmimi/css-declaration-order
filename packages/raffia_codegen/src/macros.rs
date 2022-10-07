macro_rules! emit {
    ($s:expr, $t:expr) => {{
        use crate::Emit;

        $s.emit(&$t)?;
    }};
}

macro_rules! translate {
    ($s:expr, $t:expr) => {{
        let res = $s.serialize.translate($t);

        res
    }};
}

macro_rules! write_last {
    ($s:expr, $t:expr) => {{
        let res = $s.serialize.write_last($t);

        res
    }};
}

macro_rules! new_line {
    ($s:expr, $t:expr) => {{
        write_raw!($s, $s.serialize.new_line($t))?;
    }};
}

macro_rules! space {
    ($s:expr) => {{
        write_str!($s, " ")?
    }};

    ($s:expr, $c:expr) => {{
        $s.writer.write_raw(" ".repeat($c))
    }};
}

macro_rules! write_raw {
    ($s:expr, $t:expr) => {{
        let m = $t;
        if m.is_some() {
            $s.writer.write_raw(m.unwrap())
        } else {
            Ok(())
        }
    }};
}


macro_rules! write_str {
    ($s:expr, $t:expr) => {{
        write_raw!($s, Some($t.into()))
    }};
}