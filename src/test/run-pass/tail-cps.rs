


// -*- rust -*-
fn checktrue(rs: bool) -> bool { assert (rs); return true; }

fn main() { let k = checktrue; evenk(42, k); oddk(45, k); }

fn evenk(n: int, k: extern fn(bool) -> bool) -> bool {
    debug!{"evenk"};
    log(debug, n);
    if n == 0 { return k(true); } else { return oddk(n - 1, k); }
}

fn oddk(n: int, k: extern fn(bool) -> bool) -> bool {
    debug!{"oddk"};
    log(debug, n);
    if n == 0 { return k(false); } else { return evenk(n - 1, k); }
}
