pub fn arange(steps: usize, delta: f64) -> impl Iterator<Item=f64> + 'static {
    return (0usize..steps).map(move |i| (i as f64)*delta);
}