//! Explicit recurrent-state composition; reset and detach boundaries remain caller choices.
use kernix::{Kahen, Order, Tensor, add, nn::Linear, tanh, training::SeededRng};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = SeededRng::new(11);
    let observation = Linear::<f32>::new(3, 4, &mut rng)?;
    let transition = Linear::<f32>::new(4, 4, &mut rng)?;
    let mut state = Tensor::filled(Kahen::new([1, 4])?, Order::C, 4, 0.0)?;
    for frame in 0..5 {
        let features = Tensor::from_vec(
            vec![frame as f32 / 5.0, 0.25, 0.5],
            Kahen::new([1, 3])?,
            Order::C,
        )?;
        state = tanh(&add(
            &observation.forward(&features)?,
            &transition.forward(&state)?,
        )?)?;
    }
    assert_eq!(state.shape().dims(), &[1, 4]);
    println!(
        "State after five synthetic observations: {:?}",
        state.storage_slice()
    );
    Ok(())
}
