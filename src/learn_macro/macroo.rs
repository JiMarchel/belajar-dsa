macro_rules! four {
    () => {
        1 + 3
    };
}

macro_rules! times_five {
    ($e:expr) => {
        $e * 5
    };
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {
        $a * ($b + $c)
    };
}

macro_rules! vec_strs {
    ($($element:expr), *) => {
        {
            let mut v = Vec::new();
            $(v.push(format!("{}", $element));)*

            v
        }
    };
}

pub(crate) use four;
pub(crate) use multiply_add;
pub(crate) use times_five;
pub(crate) use vec_strs;
