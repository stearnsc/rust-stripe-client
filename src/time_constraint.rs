use url_encodable::UrlEncodable;

#[derive(Clone, Debug)]
pub struct TimeConstraint {
    pub gt:  Option<i64>,
    pub gte: Option<i64>,
    pub lt:  Option<i64>,
    pub lte: Option<i64>,
}

impl TimeConstraint {
    pub fn new() -> TimeConstraint {
        TimeConstraint {
            gt:  None,
            gte: None,
            lt:  None,
            lte: None,
        }
    }

    pub fn greater_than(mut self, timestamp: i64) -> TimeConstraint {
        self.gt = Some(timestamp);
        self
    }

    pub fn greater_than_or_equal(mut self, timestamp: i64) -> TimeConstraint {
        self.gte = Some(timestamp);
        self
    }

    pub fn less_than(mut self, timestamp: i64) -> TimeConstraint {
        self.lt = Some(timestamp);
        self
    }

    pub fn less_than_or_equal(mut self, timestamp: i64) -> TimeConstraint {
        self.lte = Some(timestamp);
        self
    }
}

impl UrlEncodable for TimeConstraint {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let mut vec = Vec::new();
        if let Some(gt) = self.gt {
            vec.push(("gt".to_string(), gt.to_string()));
        }
        if let Some(gte) = self.gte {
            vec.push(("gte".to_string(), gte.to_string()));
        }
        if let Some(lt) = self.lt {
            vec.push(("lt".to_string(), lt.to_string()));
        }
        if let Some(lte) = self.lte {
            vec.push(("lte".to_string(), lte.to_string()));
        }
        vec
    }
}
