pub fn mslp(p: f64, h: f64, t: f64) -> f64 {
    if p.is_nan() || h.is_nan() || t.is_nan() {
        return f64::NAN;
    }
    return p * ((1.0 - 0.0065 * h / (t + 0.0065 * h + 273.15)).powf(-5.257));
}

pub fn dewpoint_fast(t: f64, rh: f64) -> f64 {
    if t.is_nan() || rh.is_nan() {
        return f64::NAN;
    }
    let a = 17.271;
    let b = 237.7;
    let alpha = ((a * t) / (b + t)) + (rh / 100.0).ln();
    let dew_point = ((b * alpha) / (a - alpha) * 100.0).round() / 100.0;
    return dew_point;
}
