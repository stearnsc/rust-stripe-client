use std::collections::BTreeMap;

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

    pub fn to_map(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        if let Some(gt) = self.gt {
            map.insert("gt".to_string(), gt.to_string());
        }
        if let Some(gte) = self.gte {
            map.insert("gte".to_string(), gte.to_string());
        }
        if let Some(lt) = self.lt {
            map.insert("lt".to_string(), lt.to_string());
        }
        if let Some(lte) = self.lte {
            map.insert("lte".to_string(), lte.to_string());
        }
        map
    }
}

impl Into<BTreeMap<String, String>> for TimeConstraint {
    fn into(self) -> BTreeMap<String, String> {
        (&self).into()
    }
}

impl<'a> Into<BTreeMap<String, String>> for &'a TimeConstraint {
    fn into(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        if let Some(gt) = self.gt {
            map.insert("gt".to_string(), gt.to_string());
        }
        if let Some(gte) = self.gte {
            map.insert("gte".to_string(), gte.to_string());
        }
        if let Some(lt) = self.lt {
            map.insert("lt".to_string(), lt.to_string());
        }
        if let Some(lte) = self.lte {
            map.insert("lte".to_string(), lte.to_string());
        }
        map
    }
}
