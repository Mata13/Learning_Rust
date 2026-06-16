use plotters::prelude::*;

fn contraccion(x: f64) -> f64 {
    0.5 * x + 1.0   // f(x) = 0.5*x + 1, punto fijo en 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("output")?;
    let nombre_archivo = "output/teorema_banach.png";

    let root = BitMapBackend::new(nombre_archivo, (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Teorema del punto fijo de Banach — f(x) = 0.5x + 1", ("sans-serif", 24))
        .margin(15)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(0.0_f64..2.5, 0.0_f64..2.5)?;

    chart.configure_mesh()
        .x_desc("x_n")
        .y_desc("x_{n+1}")
        .draw()?;

    // Dibujar la función f(x) (verde)
    let f_curve = (0..250).map(|i| {
        let x = i as f64 * 0.01; // 0..2.5
        (x, contraccion(x))
    });
    chart.draw_series(LineSeries::new(f_curve, &GREEN))?;

    // Dibujar la recta identidad y = x (gris punteada)
    let ident = [(0.0, 0.0), (2.5, 2.5)];
    chart.draw_series(LineSeries::new(ident, &BLACK.mix(0.3)))?
        .label("y = x");

    // ---------- Iteración de punto fijo (telaraña) ----------
    let x0 = 0.2;  // punto inicial (cualquiera en [0,2] funciona)
    let iteraciones = 12;

    let mut x_actual = x0;
    let mut puntos_telaraña: Vec<(f64, f64)> = Vec::new();
    puntos_telaraña.push((x_actual, x_actual)); // inicio en la diagonal

    for _ in 0..iteraciones {
        let x_siguiente = contraccion(x_actual);
        // paso horizontal: (x_actual, x_actual) -> (x_actual, x_siguiente)
        puntos_telaraña.push((x_actual, x_siguiente));
        // paso vertical: (x_actual, x_siguiente) -> (x_siguiente, x_siguiente)
        puntos_telaraña.push((x_siguiente, x_siguiente));
        x_actual = x_siguiente;
    }

    // Dibujar telaraña en azul
    chart.draw_series(LineSeries::new(puntos_telaraña, &BLUE))?
        .label("Iteración x_{n+1} = f(x_n)");

    // Marcar el punto fijo
    let punto_fijo = (2.0, 2.0);
    chart.draw_series(std::iter::once(Cross::new(punto_fijo, 8, &RED)))?;

    // Leyenda
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("✅ Imagen guardada como '{}'", nombre_archivo);
    Ok(())
}