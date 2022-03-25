use rand::Rng;

struct Juror {
    accuracy: f32,
}

struct Judgement {
    jury: Vec<Juror>,
}

impl Juror {
    pub fn new(accuracy: f32) -> Juror {
        Juror { accuracy: accuracy }
    }

    /// Asks a given juror to pass a judgement
    /// Returns whether or not the judgement was "correct"
    pub fn judge(&self) -> i32 {
        let mut rng = rand::thread_rng();

        // println!("{:?}", rng.gen::<f32>());

        if rng.gen::<f32>() <= self.accuracy {
            return 1;
        }
        0
    }
}

impl Judgement {
    pub fn new_uniform(size: usize, accuracy: f32) -> Judgement {
        let mut jury: Vec<Juror> = Vec::new();

        for i in 0..size {
            jury.push(Juror::new(accuracy))
        }

        Judgement { jury: jury }
    }

    pub fn judge(&self) -> f32 {
        let sum: i32 = self.jury.iter().map(|j| j.judge()).sum();

        if sum as f32 > (self.jury.len() as f32 / 2.0) {
            return 1.0;
        }
        0.0
    }
}

fn main() {

    const ITERATIONS: usize = 10_000;
    const JURY_SIZE: usize = 1000;

    // As the jury size is increased, the propoertion of judgements that are
    // voted correctly increases, despite each juror only being correct 55%
    // of the time.

    let mut scores = [0.0; ITERATIONS];
    for i in 0..ITERATIONS {
        let judge = Judgement::new_uniform(JURY_SIZE, 0.51);

        scores[i] = judge.judge();
    }

    let sum: f32 = scores.iter().sum();
    let ave_score = sum as f32 / ITERATIONS as f32;

    println!("{:?}", ave_score);
}

mod tests {}
