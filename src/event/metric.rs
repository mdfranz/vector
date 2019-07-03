#[derive(Debug, Clone, PartialEq)]
pub enum Metric {
    Counter {
        name: String,
        val: f32,
    },
    Timer {
        name: String,
        val: f32,
    },
    Gauge {
        name: String,
        val: f32,
        direction: Option<Direction>,
    },
    Set {
        name: String,
        val: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricPack {
    metric: Metric,
    count: u32,
}

impl MetricPack {
    pub fn unpack(self) -> Vec<Metric> {
        vec![self.metric; self.count as usize]
    }
}

impl Metric {
    pub fn pack(self, count: u32) -> MetricPack {
        MetricPack {
            metric: self,
            count,
        }
    }
}
