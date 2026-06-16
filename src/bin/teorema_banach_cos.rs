use plotters::prelude::*;

fn contraccion(x: f64) -> f64 {
    x.cos()   // f(x) = cos(x), contracción en [0,1]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("output")?;
    let nombre_archivo = "output/teorema_banach_cos.png";

    let root = BitMapBackend::new(nombre_archivo, (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let dominio = 0.0_f64..1.2;

    let mut chart = ChartBuilder::on(&root)
        .caption("Teorema del punto fijo de Banach — f(x) = cos(x)", ("sans-serif", 24))
        .margin(15)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(dominio.clone(), dominio.clone())?;

    chart.configure_mesh()
        .x_desc("x_n")
        .y_desc("x_{n+1}")
        .draw()?;

    // Curva f(x)=cos(x) (verde)
    let f_curve = (0..120).map(|i| {
        let x = i as f64 * 0.01; // 0..1.2
        (x, contraccion(x))
    });
    chart.draw_series(LineSeries::new(f_curve, &GREEN))?;

    // Recta identidad (gris punteada)
    let ident = [(0.0, 0.0), (1.2, 1.2)];
    chart.draw_series(LineSeries::new(ident, &BLACK.mix(0.3)))?
        .label("y = x");

    // --- Iteración de punto fijo (telaraña) ---
    let x0 = 0.5;  // punto inicial cualquiera en [0,1]
    let iteraciones = 15;

    let mut x_actual = x0;
    let mut puntos_telaraña: Vec<(f64, f64)> = Vec::new();
    puntos_telaraña.push((x_actual, x_actual)); // inicio en la diagonal

    for _ in 0..iteraciones {
        let x_siguiente = contraccion(x_actual);
        puntos_telaraña.push((x_actual, x_siguiente)); // horizontal
        puntos_telaraña.push((x_siguiente, x_siguiente)); // vertical
        x_actual = x_siguiente;
    }

    // Telaraña en azul
    chart.draw_series(LineSeries::new(puntos_telaraña, &BLUE))?
        .label("Iteración x_{n+1} = cos(x_n)");

    // Punto fijo (número de Dottie)
    let punto_fijo = (0.739085, 0.739085);
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