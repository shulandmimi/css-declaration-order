use std::ops::Div;

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
        $s.writer.write_raw(" ")
    }};

    ($s:expr, $c:expr) => {{
        $s.writer.write_raw(" ".repeat($c))
    }};
}

fn pad_start(res: String, str: &str, max: usize) -> String {
    let cur_len = res.len();
    if max < cur_len {
        return res;
    }

    let str_len = str.len();
    let retain_len = max - cur_len.div(str_len);
    let mut c = 0;

    while c < retain_len {
        c += str_len;
    }

    return res;
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
