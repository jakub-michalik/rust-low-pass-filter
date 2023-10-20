#[derive(Debug)]
struct LeakyIntegrator {
    a: f64,
    s: f64,
}

impl LeakyIntegrator {
    fn new(a: f64) -> Self {
        LeakyIntegrator { a, s: 0.0 }
    }

    fn update(&mut self, x: f64) {
        self.s = self.a * x + (1.0 - self.a) * self.s;
    }

    fn output(&self) -> f64 {
        self.s
    }

}

#[derive(Debug)]
struct LPF {
    stages: Vec<LeakyIntegrator>,
}

impl LPF {
    fn new(a: f64, stages: usize) -> Self {
        LPF {
            stages: (0..stages).map(|_| LeakyIntegrator::new(a)).collect(),
        }
    }

    fn update(&mut self, x: f64) {
        let mut input = x;
        for stage in self.stages.iter_mut() {
            stage.update(input);
            input = stage.output();
        }
    }

    fn output(&self) -> f64 {
        self.stages.last().unwrap().output()
    }
}

fn main() {
    let mut lpf = LPF::new(0.5, 3);
    let inputs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    
    for input in inputs {
        lpf.update(input);
        println!("Updated LPF: {:?}", lpf);
        println!("Current Output: {}", lpf.output());
    }
}
