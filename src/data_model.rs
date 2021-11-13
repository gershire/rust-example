use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use rdkafka::message::ToBytes;
use serde::de::{self, EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub(crate) struct AppError {
    pub(crate) message: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Vehicle {
    pub(crate) id: String,
    pub(crate) location: Location,
}

#[derive(Debug)]
pub(crate) struct Location {
    lng: f32,
    lat: f32,
    encoded: Vec<u8>,
}

impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        enum Field { Lng, Lat }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str("`lng` or `lat`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: Error
                    {
                        match v {
                            "lng" => Ok(Field::Lng),
                            "lat" => Ok(Field::Lat),
                            _ => Err(de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct LocationVisitor;

        impl<'de> Visitor<'de> for LocationVisitor {
            type Value = Location;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("struct Location")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Location, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut lng = None;
                let mut lat = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Lng => {
                            if lng.is_some() {
                                return Err(de::Error::duplicate_field("lng"));
                            }
                            lng = Some(map.next_value()?);
                        }
                        Field::Lat => {
                            if lat.is_some() {
                                return Err(de::Error::duplicate_field("lat"));
                            }
                            lat = Some(map.next_value()?);
                        }
                    }
                }
                let lng = lng.ok_or_else(|| de::Error::missing_field("lng"))?;
                let lat = lat.ok_or_else(|| de::Error::missing_field("lat"))?;
                Ok(Location::new(lng, lat))
            }
        }

        const FIELDS: &'static [&'static str] = &["lng", "lat"];
        deserializer.deserialize_struct("Location", FIELDS, LocationVisitor)
    }
}

impl AsRef<[u8]> for Location {
    fn as_ref(&self) -> &[u8] {
        &self.encoded
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.lng, self.lat)
    }
}

impl Location {
    pub(crate) fn new(lng: f32, lat: f32) -> Location {
        let lng_bytes = lng.to_le_bytes();
        let lat_bytes = lat.to_le_bytes();
        let encoded = [lng_bytes, lat_bytes].concat();

        Location { lng, lat, encoded }
    }

    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Result<Location, AppError> {
        if bytes.len() < 8 {
            Err(AppError {
                message: "Malformed data".to_string(),
            })
        } else {
            let bytes = bytes.to_bytes();
            let lng = f32::from_le_bytes(bytes[0..4].try_into().expect("slice with incorrect length"));
            let lat = f32::from_le_bytes(bytes[4..].try_into().expect("slice with incorrect length"));
            Ok(Location::new(lng, lat))
        }
    }

    pub(crate) fn set_lng(&mut self, lng: f32) {
        let lng_bytes = lng.to_le_bytes();
        self.encoded.splice(..4, lng_bytes);
        self.lng = lng;
    }

    pub(crate) fn get_lng(&self) -> f32 {
        self.lng
    }

    pub(crate) fn set_lat(&mut self, lat: f32) {
        let lat_bytes = lat.to_le_bytes();
        self.encoded.splice(4.., lat_bytes);
        self.lat = lat;
    }

    pub(crate) fn get_lat(&self) -> f32 {
        self.lat
    }
}
