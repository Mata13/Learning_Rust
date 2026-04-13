fn main() {
    // Variables inmutables por defecto (como una constante física)
    let velocidad_luz: f64 = 299_792_458.0; // m/s

    // Variable mutable (puede cambiar, como la velocidad de tu nave)
    let mut velocidad_nave: f64 = 0.0;
    velocidad_nave = 11_200.0; // velocidad de escape Tierra en m/s

    println!("Velocidad de escape: {} m/s", velocidad_nave);
    println!("Velocidad de la luz: {} m/s", velocidad_luz);
}