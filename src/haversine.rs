use std::fmt;

#[derive(Default)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(FromFormField, PartialEq)]
pub enum Units {
    Miles,
    Kilometers,
    NauticalMiles,
    Mi,
    Km,
    Nm,
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Units::Kilometers | Units::Km => write!(f, "km"),
            Units::Miles | Units::Mi => write!(f, "mi"),
            Units::NauticalMiles | Units::Nm => write!(f, "NM"),
        }
    }
}

pub fn distance(start: Location, end: Location, units: &Units) -> f64 {
    let r = match units {
        Units::Miles | Units::Km => 3958.8,
        Units::Kilometers | Units::Mi => 6371.0,
        Units::NauticalMiles | Units::Nm => 3440.1,
    };

    let d_lat: f64 = (end.latitude - start.latitude).to_radians();
    let d_lon: f64 = (end.longitude - start.longitude).to_radians();
    let lat1: f64 = (start.latitude).to_radians();
    let lat2: f64 = (end.latitude).to_radians();

    let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

    return r * c;
}
