use std::{collections::HashMap, fs, io::Error, str::FromStr};
use tinyjson::JsonValue;

use crate::template::Day;

static TIMINGS_FILE_PATH: &str = "./data/timings.json";

/// Represents benchmark times for a single day.
#[derive(Clone, Debug)]
pub struct Timing {
    pub day: Day,
    pub part_1: Option<String>,
    pub part_2: Option<String>,
    pub total_nanos: f64,
}

/// Represents benchmark times for a set of days.
/// Can be serialized from / to JSON.
#[derive(Clone, Debug, Default)]
pub struct Timings {
    pub data: Vec<Timing>,
}

impl Timings {
    /// Dehydrate timings to a JSON file.
    pub fn store_file(&self) -> Result<(), Error> {
        let json = JsonValue::from(self.clone());
        let mut file = fs::File::create(TIMINGS_FILE_PATH)?;
        json.format_to(&mut file)
    }

    /// Rehydrate timings from a JSON file. If not present, returns empty timings.
    pub fn read_from_file() -> Self {
        fs::read_to_string(TIMINGS_FILE_PATH)
            .map_err(|x| x.to_string())
            .and_then(Timings::try_from)
            .unwrap_or_default()
    }

    /// Merge two sets of timings, overwriting `self` with `other` if present.
    pub fn merge(&self, new: &Self) -> Self {
        let mut data: Vec<Timing> = vec![];

        for timing in &new.data {
            data.push(timing.clone());
        }

        for timing in &self.data {
            if !data.iter().any(|t| t.day == timing.day) {
                data.push(timing.clone());
            }
        }

        data.sort_unstable_by(|a, b| a.day.cmp(&b.day));
        Timings { data }
    }

    /// Sum up total duration of timings as millis.
    pub fn total_millis(&self) -> f64 {
        self.data.iter().map(|x| x.total_nanos).sum::<f64>() / 1_000_000_f64
    }

    pub fn is_day_complete(&self, day: Day) -> bool {
        self.data
            .iter()
            .any(|t| t.day == day && t.part_1.is_some() && t.part_2.is_some())
    }
}

/* -------------------------------------------------------------------------- */

impl From<Timings> for JsonValue {
    fn from(value: Timings) -> Self {
        let mut map: HashMap<String, JsonValue> = HashMap::new();

        map.insert(
            "data".into(),
            JsonValue::Array(value.data.iter().map(JsonValue::from).collect()),
        );

        JsonValue::Object(map)
    }
}

impl TryFrom<String> for Timings {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let json = JsonValue::from_str(&value).or(Err("not valid JSON file."))?;

        let json_data = json
            .get::<HashMap<String, JsonValue>>()
            .ok_or("expected JSON document to be an object.")?
            .get("data")
            .ok_or("expected JSON document to have key `data`.")?
            .get::<Vec<JsonValue>>()
            .ok_or("expected `json.data` to be an array.")?;

        Ok(Timings {
            data: json_data
                .iter()
                .map(Timing::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

/* -------------------------------------------------------------------------- */

impl From<&Timing> for JsonValue {
    fn from(value: &Timing) -> Self {
        let mut map: HashMap<String, JsonValue> = HashMap::new();

        map.insert("day".into(), JsonValue::String(value.day.to_string()));
        map.insert("total_nanos".into(), JsonValue::Number(value.total_nanos));

        let part_1 = value.part_1.clone().map(JsonValue::String);
        let part_2 = value.part_2.clone().map(JsonValue::String);

        map.insert(
            "part_1".into(),
            match part_1 {
                Some(x) => x,
                None => JsonValue::Null,
            },
        );

        map.insert(
            "part_2".into(),
            match part_2 {
                Some(x) => x,
                None => JsonValue::Null,
            },
        );

        JsonValue::Object(map)
    }
}

impl TryFrom<&JsonValue> for Timing {
    type Error = String;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let json = value
            .get::<HashMap<String, JsonValue>>()
            .ok_or("Expected timing to be a JSON object.")?;

        let day = json
            .get("day")
            .and_then(|v| v.get::<String>())
            .and_then(|day| Day::from_str(day).ok())
            .ok_or("Expected timing.day to be a Day struct.")?;

        let part_1 = json
            .get("part_1")
            .map(|v| if v.is_null() { None } else { v.get::<String>() })
            .ok_or("Expected timing.part_1 to be null or string.")?;

        let part_2 = json
            .get("part_2")
            .map(|v| if v.is_null() { None } else { v.get::<String>() })
            .ok_or("Expected timing.part_2 to be null or string.")?;

        let total_nanos = json
            .get("total_nanos")
            .and_then(|v| v.get::<f64>().copied())
            .ok_or("Expected timing.total_nanos to be a number.")?;

        Ok(Timing {
            day,
            part_1: part_1.cloned(),
            part_2: part_2.cloned(),
            total_nanos,
        })
    }
}

/* -------------------------------------------------------------------------- */

#[cfg(feature = "test_lib")]
mod tests {
    use crate::day;

    use super::{Timing, Timings};

    fn get_mock_timings() -> Timings {
        Timings {
            data: vec![
                Timing {
                    day: day!(1),
                    part_1: Some("10ms".into()),
                    part_2: Some("20ms".into()),
                    total_nanos: 3e+10,
                },
                Timing {
                    day: day!(2),
                    part_1: Some("30ms".into()),
                    part_2: Some("40ms".into()),
                    total_nanos: 7e+10,
                },
                Timing {
                    day: day!(4),
                    part_1: Some("40ms".into()),
                    part_2: None,
                    total_nanos: 4e+10,
                },
            ],
        }
    }

    mod deserialization {
        use crate::{day, template::timings::Timings};

        #[test]
        fn handles_json_timings() {
            let json = r#"{ "data": [{ "day": "01", "part_1": "1ms", "part_2": null, "total_nanos": 1000000000 }] }"#.to_string();
            let timings = Timings::try_from(json).unwrap();
            assert_eq!(timings.data.len(), 1);
            let timing = timings.data.first().unwrap();
            assert_eq!(timing.day, day!(1));
            assert_eq!(timing.part_1, Some("1ms".to_string()));
            assert_eq!(timing.part_2, None);
            assert_eq!(timing.total_nanos, 1_000_000_000_f64);
        }

        #[test]
        fn handles_empty_timings() {
            let json = r#"{ "data": [] }"#.to_string();
            let timings = Timings::try_from(json).unwrap();
            assert_eq!(timings.data.len(), 0);
        }

        #[test]
        #[should_panic]
        fn panics_for_invalid_json() {
            let json = r#"{}"#.to_string();
            Timings::try_from(json).unwrap();
        }

        #[test]
        #[should_panic]
        fn panics_for_malformed_timings() {
            let json = r#"{ "data": [{ "day": "01" }, { "day": "26" }, { "day": "02", "part_2": null, "total_nanos": 0 }] }"#.to_string();
            Timings::try_from(json).unwrap();
        }
    }

    mod serialization {
        use super::get_mock_timings;
        use std::collections::HashMap;
        use tinyjson::JsonValue;

        #[test]
        fn serializes_timings() {
            let timings = get_mock_timings();
            let value = JsonValue::try_from(timings).unwrap();
            assert_eq!(
                value
                    .get::<HashMap<String, JsonValue>>()
                    .unwrap()
                    .get("data")
                    .unwrap()
                    .get::<Vec<JsonValue>>()
                    .unwrap()
                    .len(),
                3
            );
        }
    }

    mod is_day_complete {
        use crate::{
            day,
            template::timings::{Timing, Timings},
        };

        #[test]
        fn handles_completed_days() {
            let timings = Timings {
                data: vec![Timing {
                    day: day!(1),
                    part_1: Some("1ms".into()),
                    part_2: Some("2ms".into()),
                    total_nanos: 3_000_000_000_f64,
                }],
            };

            assert_eq!(timings.is_day_complete(&day!(1)), true);
        }

        #[test]
        fn handles_partial_days() {
            let timings = Timings {
                data: vec![Timing {
                    day: day!(1),
                    part_1: Some("1ms".into()),
                    part_2: None,
                    total_nanos: 1_000_000_000_f64,
                }],
            };

            assert_eq!(timings.is_day_complete(&day!(1)), false);
        }

        #[test]
        fn handles_uncompleted_days() {
            let timings = Timings {
                data: vec![Timing {
                    day: day!(1),
                    part_1: None,
                    part_2: None,
                    total_nanos: 0.0,
                }],
            };

            assert_eq!(timings.is_day_complete(&day!(1)), false);
        }
    }

    mod merge {
        use crate::{
            day,
            template::timings::{Timing, Timings},
        };

        use super::get_mock_timings;

        #[test]
        fn handles_disjunct_timings() {
            let timings = get_mock_timings();
            let other = Timings {
                data: vec![Timing {
                    day: day!(3),
                    part_1: None,
                    part_2: None,
                    total_nanos: 0_f64,
                }],
            };
            let merged = timings.merge(&other);
            assert_eq!(merged.data.len(), 4);
            assert_eq!(merged.data[0].day, day!(1));
            assert_eq!(merged.data[1].day, day!(2));
            assert_eq!(merged.data[2].day, day!(3));
            assert_eq!(merged.data[3].day, day!(4));
        }

        #[test]
        fn handles_overlapping_timings() {
            let timings = get_mock_timings();

            let other = Timings {
                data: vec![Timing {
                    day: day!(2),
                    part_1: None,
                    part_2: None,
                    total_nanos: 0_f64,
                }],
            };
            let merged = timings.merge(&other);

            assert_eq!(merged.data.len(), 3);
            assert_eq!(merged.data[0].day, day!(1));
            assert_eq!(merged.data[1].day, day!(2));
            assert_eq!(merged.data[1].total_nanos, 0_f64);
            assert_eq!(merged.data[2].day, day!(4));
        }

        #[test]
        fn handles_empty_timings() {
            let timings = Timings::default();
            let other = get_mock_timings();
            let merged = timings.merge(&other);
            assert_eq!(merged.data.len(), 3);
        }

        #[test]
        fn handles_empty_other_timings() {
            let timings = get_mock_timings();
            let other = Timings::default();
            let merged = timings.merge(&other);
            assert_eq!(merged.data.len(), 3);
        }
    }
}
