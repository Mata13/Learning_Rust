use plotters::prelude::*;

// Función original a aproximar
fn f(x: f64) -> f64 {
    (x - 0.5).abs()
}

// Coeficiente binomial
fn binomial(n: u32, k: u32) -> f64 {
    let mut res = 1.0;
    for i in 1..=k {
        res *= (n - k + i) as f64 / i as f64;
    }
    res
}

// Polinomio de Bernstein de grado n evaluado en x
fn bernstein(n: u32, x: f64) -> f64 {
    let mut suma = 0.0;
    for k in 0..=n {
        let binom = binomial(n, k);
        let termino = binom * f(k as f64 / n as f64) * x.powi(k as i32) * (1.0 - x).powi((n - k) as i32);
        suma += termino;
    }
    suma
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("output")?;
    let nombre_archivo = "output/weierstrass_bernstein.png";

    let root = BitMapBackend::new(nombre_archivo, (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let dominio = -0.05_f64..1.05;

    let mut chart = ChartBuilder::on(&root)
        .caption("Aproximación de f(x)=|x-0.5| con polinomios de Bernstein", ("sans-serif", 18))
        .margin(10)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(dominio.clone(), -0.1_f64..0.6)?;

    chart.configure_mesh()
        .x_desc("x")
        .y_desc("y")
        .draw()?;

    // Graficar la función original (negra, gruesa)
    let original: Vec<(f64, f64)> = (0..200).map(|i| {
        let x = i as f64 * 0.005; // 0..1
        (x, f(x))
    }).collect();
    // Corrección: quitar & antes de BLACK.stroke_width(2)
    chart.draw_series(LineSeries::new(original, BLACK.stroke_width(2)))?
        .label("f(x) = |x-0.5|");

    // Colores para los polinomios
    let colores = [&RED, &BLUE, &GREEN, &MAGENTA];
    let grados = [2, 4, 8, 16];

    for (i, &n) in grados.iter().enumerate() {
        let puntos: Vec<(f64, f64)> = (0..200).map(|j| {
            let x = j as f64 * 0.005;
            (x, bernstein(n, x))
        }).collect();
        // colores[i] es &RGBColor; .stroke_width(1) devuelve ShapeStyle por valor, sin &
        chart.draw_series(LineSeries::new(puntos, colores[i].stroke_width(1)))?
            .label(format!("n = {}", n));
    }

    // Leyenda
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("✅ Imagen guardada como '{}'", nombre_archivo);
    Ok(())
}