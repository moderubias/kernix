//! Learn synthetic stripe orientation, then save and restore weights/Adam state.
//! This is a library training demonstration, not an application/game model.
use kernix::{
    Kahen, Order, Tensor,
    autograd::Tape,
    checkpoint::Checkpoint,
    neural::Conv2dOptions,
    nn::{Conv2d, Linear},
    training::{Adam, SeededRng},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = SeededRng::new(7);
    let mut conv = Conv2d::<f64>::new(1, 4, [2, 2], Conv2dOptions::default(), &mut rng)?;
    let mut dense = Linear::new(4, 2, &mut rng)?;
    let mut adam = Adam::new(0.03)?;
    let mut pixels = Vec::new();
    let mut labels = Vec::new();
    for sample in 0..8 {
        let label = sample % 2;
        labels.push(label);
        for y in 0..4 {
            for x in 0..4 {
                pixels.push(if (label == 0 && x < 2) || (label == 1 && y < 2) {
                    1.0
                } else {
                    0.0
                });
            }
        }
    }
    let input = Tensor::from_vec(pixels, Kahen::new([8, 1, 4, 4])?, Order::C)?;
    let mut first = 0.0;
    let mut last = 0.0;
    for step in 0..100 {
        let mut tape = Tape::new();
        let x = tape.constant(input.clone())?;
        let c = conv.record(&mut tape, &x)?;
        let active = tape.tanh(&c.output)?;
        let pooled = tape.avg_pool2d(&active, [3, 3], [3, 3])?;
        let flat = tape.reshape(&pooled, [8, 4])?;
        let d = dense.record(&mut tape, &flat)?;
        let loss = tape.cross_entropy(&d.output, &labels)?;
        last = tape.value(&loss)?.storage_slice()[0];
        if step == 0 {
            first = last;
        }
        let gradients = tape.backward(&loss)?;
        adam.step(&mut [
            (&mut conv.weight, gradients.get(&c.weight)?.unwrap()),
            (&mut conv.bias, gradients.get(&c.bias)?.unwrap()),
            (&mut dense.weight, gradients.get(&d.weight)?.unwrap()),
            (&mut dense.bias, gradients.get(&d.bias)?.unwrap()),
        ])?;
    }
    assert!(
        last < first * 0.2,
        "loss did not decrease sufficiently: {first} -> {last}"
    );
    let path = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::env::temp_dir().join(format!("kernix-patterns-{}.ckpt", std::process::id()))
        });
    Checkpoint::capture(
        &[
            ("conv.weight", &conv.weight),
            ("conv.bias", &conv.bias),
            ("dense.weight", &dense.weight),
            ("dense.bias", &dense.bias),
        ],
        Some(&adam),
    )?
    .save(&path)?;
    let saved = Checkpoint::<f64>::load(&path)?;
    saved.restore(
        &mut [
            ("conv.weight", &mut conv.weight),
            ("conv.bias", &mut conv.bias),
            ("dense.weight", &mut dense.weight),
            ("dense.bias", &mut dense.bias),
        ],
        Some(&mut adam),
    )?;
    println!(
        "Synthetic stripe classification: loss {first:.6} -> {last:.6}; checkpoint {}",
        path.display()
    );
    Ok(())
}
