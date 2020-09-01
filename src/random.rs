use rand::random;

pub fn is_true(chance: f64) -> bool {
    random::<f64>() < chance
}

pub fn dice_u32(sides: u32) -> u32 {
    let result = (random::<f64>() * sides as f64).floor() as u32;

    if result == sides {
        result - 1
    } else {
        result
    }
}

pub fn dice_usize(sides: usize) -> usize {
    let result = (random::<f64>() * sides as f64).floor() as usize;

    if result == sides {
        result - 1
    } else {
        result
    }
}
