//! # Utils
//!
//! This submodule contains helping functions used by the rest of the
//! library. It is not publicly accessible.

use macroquad::prelude::{Color, Vec3};

pub fn map_color_decay(orig: Color, current: f32, total: f32) -> Color {
    Color::new(orig.r, orig.g, orig.b, orig.a * (1.0 - (current / total)))
}

// find the linearly interpolated value from 'values' given the ratio 'elapsed' / 'total'
pub fn map_float_value(values: &[f32], elapsed: f32, total: f32) -> Result<f32, String> {
    let ratio = elapsed / total;
    let len = values.len() - 1;
    let vratio = len as f32 * ratio;
    let low = (vratio.floor()) as usize;
    let high = (vratio.ceil()) as usize;

    let low = if low > len { len } else { low };
    let high = if high > len { len } else { high };

    let first_value = match values.get(low) {
        Some(val) => val,
        None => {
            return Err(format!(
                "map_float_values indexing error: {} of {}",
                low, len
            ));
        }
    };

    if low == high {
        Ok(*first_value)
    } else {
        match values.get(high) {
            Some(val) => {
                let vratio_norm = high as f32 - vratio;
                Ok((first_value * vratio_norm) + (val * (1.0 - vratio_norm)))
            }
            None => Err(format!(
                "map_float_values indexing error: {} of {}",
                high, len
            )),
        }
    }
}

#[test]
fn map_float_value_test() {
    let values = vec![0.0, 1.0];
    assert_eq!(map_float_value(&values, 0.0, 1.0).unwrap_or(-1.0), 0.0);
    assert_eq!(
        map_float_value(&values, 2.0 / 3.0, 1.0).unwrap_or(-1.0),
        2.0 / 3.0
    );

    let values = vec![1.0, 0.0, 0.5, 0.0];
    assert_eq!(map_float_value(&values, 0.5, 1.0).unwrap_or(-1.0), 0.25);
}

// find the linearly interpolated color from 'colors' given the ratio 'elapsed' / 'total'
pub fn map_color_value(
    colors: &[Color],
    elapsed: f32,
    total: f32,
) -> Result<(f32, f32, f32, f32), String> {
    let ratio = elapsed / total;
    let len = colors.len() - 1;
    let vratio = len as f32 * ratio;
    let low = ((len as f32 * ratio).floor()) as usize;
    let high = ((len as f32 * ratio).ceil()) as usize;

    let low = if low > len { len } else { low };
    let high = if high > len { len } else { high };

    let first_value = match colors.get(low) {
        Some(val) => val,
        None => {
            return Err(format!(
                "map_color_value indexing error: {} of {}",
                low, len
            ));
        }
    };

    if low == high {
        Ok((first_value.r, first_value.g, first_value.b, first_value.a))
    } else {
        match colors.get(high) {
            Some(val) => {
                let vratio_norm = high as f32 - vratio;
                Ok((
                    (first_value.r * vratio_norm) + (val.r * (1.0 - vratio_norm)),
                    (first_value.g * vratio_norm) + (val.g * (1.0 - vratio_norm)),
                    (first_value.b * vratio_norm) + (val.b * (1.0 - vratio_norm)),
                    (first_value.a * vratio_norm) + (val.a * (1.0 - vratio_norm)),
                ))
            }
            None => Err(format!(
                "map_color_value indexing error: {} of {}",
                high, len
            )),
        }
    }
}

// Find the linearly interpolated location from 'start_location' to 'end_location'
// given the 'locations' values and the ratio 'elapsed' / 'period'
pub fn map_location(
    locations: &[f32],
    start_location: Vec3,
    end_location: Vec3,
    elapsed: f32,
    period: f32,
) -> Result<(f32, f32, f32), String> {
    let ratio = map_float_value(locations, elapsed, period)?;
    let vratio = Vec3::new(ratio, ratio, ratio);
    let v = (start_location * vratio) + ((Vec3::ONE - vratio) * end_location);
    Ok(v.into())
}

// check that the period of LinearParticles is valid
pub fn check_period(period: f32) -> Result<(), String> {
    match period {
        p if p >= 0. => Ok(()),
        p => Err(format!(
            "value error: {} period should be positive value",
            p
        )),
    }
}

// check that the decay of LinearParticles is valid
pub fn check_decay(decay: f32) -> Result<(), String> {
    match decay {
        d if d >= 0. => Ok(()),
        d => Err(format!("value error: {} decay should be positive value", d)),
    }
}

// check that the locations interpolation values are valid
pub fn check_locations(locations: &[f32]) -> Result<(), String> {
    if locations.is_empty() {
        return Err(String::from("empty vec: location Vec cannot be empty"));
    }
    for l in locations.iter() {
        if *l > 1. || *l < 0. {
            return Err(format!(
                "value error: {} location interpolation should be between 0 and 1 inclusive",
                *l
            ));
        };
    }
    Ok(())
}

// check that the density chance values are valid
pub fn check_densities(densities: &[f32]) -> Result<(), String> {
    if densities.is_empty() {
        return Err(String::from("empty vec: densities Vec cannot be empty"));
    }
    for d in densities.iter() {
        if *d > 1. || *d < 0. {
            return Err(format!(
                "value error: {} density value should be between 0 and 1 inclusive",
                *d
            ));
        };
    }
    Ok(())
}

// check that the color interpolations are valid
pub fn check_colors(colors: &[Color]) -> Result<(), String> {
    if colors.is_empty() {
        return Err(String::from("empty vec: color Vec cannot be empty"));
    }
    Ok(())
}
