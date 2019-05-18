// t = current time
// b = starting value to interpolate
// c = the total change in value of b that needs to occur
// d = total time it should take to complete (duration basically)


const PI: f32 = 3.14159265358979323846;

// Linear easing functions
pub fn linear_none(t: f32, b: f32, c: f32, d: f32) -> f32 { (c * t / d + b) }
pub fn linear_in(t: f32, b: f32, c: f32, d: f32) -> f32 { (c * t / d + b) }
pub fn linear_out(t: f32, b: f32, c: f32, d: f32) -> f32 { (c * t / d + b) }
pub fn linear_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 { (c * t / d + b) }

// Sine easing functions
pub fn sine_in(t: f32, b: f32, c: f32, d: f32) -> f32 { (-c * (t / d * (PI / 2.0)).cos() + c + b) }
pub fn sine_out(t: f32, b: f32, c: f32, d: f32) -> f32 { (c * (t / d * (PI / 2.0)).sin() + b) }
pub fn sine_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 { (-c / 2.0 * ((PI * t / d).cos() - 1.0) + b) }

// Circular easing functions
pub fn circ_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let td = t / d;
    (-c * ((1.0 - td * td).sqrt() - 1.0) + b)
}

pub fn circ_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let td = t / d - 1.0;
    (c * (1.0 - td * td).sqrt() + b)
}

pub fn circ_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut td = t / (d / 2.0);
    if td < 1.0 {
        (-c / 2.0 * ((1.0 - td * td).sqrt() - 1.0) + b)
    }
    else {
        td -= 2.0;
        (c / 2.0 * ((1.0 - td * td).sqrt() + 1.0) + b)
    }
}

// Cubic easing functions
pub fn cubic_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let td = t / d;
    (c * td * td * td + b)
}

pub fn cubic_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let td = t / d - 1.0;
    (c * (td * td * td + 1.0) + b)
}

pub fn cubic_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut td = t / (d / 2.0);
    if td < 1.0 {
        (c / 2.0 * td * td * td + b)
    }
    else {
        td -= 2.0;
        (c / 2.0 * (td * td * td + 2.0) + b)
    }
}

// Quadratic easing functions
pub fn quad_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let td = t / d;
    (c * td * td + b)
}

pub fn quad_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let td = t / d;
    (-c * td * (td - 2.0) + b)
}

pub fn quad_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let td = t / (d / 2.0);
    if td < 1.0 {
        (((c / 2.0) * (td * td)) + b)
    }
    else {
        (-c / 2.0 * (((td - 2.0) * (td - 1.0)) - 1.0) + b)
    }
}

// Exponential easing functions
pub fn expo_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == 0.0 {
        b
    }
    else {
        (c * (2.0f32).powf(10.0 * (t / d - 1.0)) + b)
    }
}

pub fn expo_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == d {
        (b + c)
    }
    else {
        (c * (-(2.0f32.powf(-10.0 * t / d)) + 1.0) + b)
    }
}

pub fn expo_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == 0.0 { return b; }
    else if t == d { return b + c; }

    let td = t / (d / 2.0);
    if td < 1.0 {
        return c / 2.0 * 2.0f32.powf(10.0 * (t - 1.0)) + b;
    }
    else {
        return c / 2.0 * (-(2.0f32.powf(-10.0 * td - 1.0)) + 2.0) + b;
    }
}

// Back easing functions
pub fn back_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let s = 1.70158f32;
    let postfix = t / d;
    (c * postfix * postfix * ((s + 1.0) * postfix - s) + b)
}

pub fn back_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let s = 1.70158f32;
    let td = t / d - 1.0;
    (c * (td * td * ((s + 1.0) * td + s) + 1.0) + b)
}

pub fn back_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut s = 1.70158f32;
    let td = t / (d / 2.0);
    if td < 1.0 {
        s *= 1.525;
        (c / 2.0 * (td * td* ((s + 1.0) * td - s)) + b)
    }
    else {
        let postfix = t - 2.0;
        s *= 1.525;
        (c / 2.0 * ((postfix) * postfix * ((s + 1.0) * t + s) + 2.0) + b)
    }
}

// Bounce easing functions
pub fn bounce_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut td = t / d;
    if td < (1.0 / 2.75) {
        (c * (7.5625 * td * td) + b)
    }
    else if td < (2.0 / 2.75) {
        td -= 1.5 / 2.75;
        (c * (7.5625 * td * td + 0.75) + b)
    }
    else if td < (2.5 / 2.75) {
        td -= 2.25 / 2.75;
        (c * (7.5625 * td * td + 0.9375) + b)
    }
    else {
        td -= 2.625 / 2.75;
        (c * (7.5625 * td * td + 0.984375) + b)
    }
}

pub fn bounce_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    (c - bounce_out(d - t, 0.0, c, d) + b)
}

pub fn bounce_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t < (d / 2.0) {
        (bounce_in(t * 2.0, 0.0, c, d) * 0.5) + b
    }
    else {
        (bounce_out(t * 2.0 - d, 0.0, c, d) * 0.5) + (c * 0.5) + b
    }
}

// Elastic easing functions
pub fn elastic_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut td = t / d;
    
    if t == 0.0 { b }
    else if td == 1.0 { b + c }
    else {
        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        td -= 1.0;
        let postfix = a * 2.0f32.powf(10.0 * td);
        (-(postfix * ((td * d - s) * (2.0 * PI) / p).sin()) + b)
    }
}

pub fn elastic_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let td = t / d;

    if t == 0.0 { b }
    else if td == 1.0 { b + c }
    else {
        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        (a * 2.0f32.powf(-10.0 * td) * ((td * d - s) * (2.0 * PI) / p).sin() + c + b)
    }
}

pub fn elastic_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut td = t / (d / 2.0);
    
    if t == 0.0 { b }
    else if td == 2.0 { b + c }
    else {
        let p = d * (0.3 * 1.5);
        let a = c;
        let s = p / 4.0;
        if td < 1.0 {
            td -= 1.0;
            let postfix = a * 2.0f32.powf(10.0 * td);
            -0.5 * (postfix*((td*d-s)*(2.0 * PI)/p).sin()) + b
        }
        else {
            td -= 1.0;
            let postfix = a * 2.0f32.powf(-10.0 * td);
            (postfix * ((td * d - s) * (2.0 * PI) / p).sin() * 0.5 + c + b)
        }
    }
}
