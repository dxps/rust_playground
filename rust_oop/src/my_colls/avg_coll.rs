// --------------------------------------------------------------------------------------
// `AveragedCollection` shows a some OOP-like features that Rust provides:
// - we have a constructor
// - there are a couple of methods
//       - `add` and `remove` are changing the internal structure of the instance
//       - `get_average` behaves like a getter
//       - `update_average` is just for internal (non-public) usage
// --------------------------------------------------------------------------------------


pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0 as f64,
        }
    }

    pub fn add(&mut self, value: i32) -> &mut AveragedCollection {
        self.list.push(value);
        self.update_average();
        self
    }

    pub fn remove(&mut self) -> Option<i32> {
        let item = self.list.pop();
        if item.is_some() {
            self.update_average();
        }
        item
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }
}
