use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::dec::date_format;
use super::RootEntry;

// Main Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SystemWithCoordinates {
    pub id: u64,
    // Attributes
    pub coords: Coords,
    pub id64: Option<u64>,
    pub name: String,
    // Metadata
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

impl RootEntry for SystemWithCoordinates {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SystemWithoutCoordinates {
    pub id: u64,
    // Attributes
    pub estimated_coordinates: Option<EstimatedCoords>,
    pub id64: Option<u64>,
    pub name: String,
    // Metadata
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

impl RootEntry for SystemWithoutCoordinates {}

// Field Type

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Coords {
    pub fn abs(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dist(self, other: Coords) -> f32 {
        (self - other).abs()
    }
}

impl Add for Coords {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Coords {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Div<f32> for Coords {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Coords {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f32> for Coords {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Mul<f32> for Coords {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Coords {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f32> for Coords {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Neg for Coords {
    type Output = Self;
    fn neg(self) -> Self {
        Coords {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Coords {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Coords {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Coords {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EstimatedCoords {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub precision: f32,
}
