use plotters::prelude::*;

// Proyección oblicua 3D → 2D (tipo caballera)
fn proyectar(x: f64, y: f64, z: f64) -> (f64, f64) {
    let screen_x = x + 0.5 * y;
    let screen_y = z + 0.5 * y;
    (screen_x, screen_y)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("output")?;
    let nombre_archivo = "output/proyeccion_ortogonal_3d.png";

    let root = BitMapBackend::new(nombre_archivo, (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Proyección ortogonal sobre un plano en R³", ("sans-serif", 22))
        .margin(15)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(-2.0_f64..6.0, -2.0_f64..6.0)?;

    chart.configure_mesh()
        .x_desc("x (proyectado)")
        .y_desc("z (proyectado)")
        .draw()?;

    // ---------- Plano M: z = 0 (sombrearlo con paralelogramo) ----------
    let plano_vertices = [
        proyectar(0.0, 0.0, 0.0),
        proyectar(5.0, 0.0, 0.0),
        proyectar(5.0, 5.0, 0.0),
        proyectar(0.0, 5.0, 0.0),
    ];
    chart.draw_series(std::iter::once(Polygon::new(
        plano_vertices.to_vec(),
        ShapeStyle::from(&GREEN.mix(0.2)).filled(),
    )))?;
    chart.draw_series(std::iter::once(Polygon::new(
        plano_vertices.to_vec(),
        &GREEN,
    )))?;
    chart.draw_series(std::iter::once(Text::new(
        "Plano M (z=0)",
        proyectar(2.5, 2.5, -0.5),
        ("sans-serif", 16).into_font(),
    )))?;

    // ---------- Punto P (3D) ----------
    let p_3d = (2.0, 3.0, 4.0);
    let p_screen = proyectar(p_3d.0, p_3d.1, p_3d.2);
    chart.draw_series(std::iter::once(Circle::new(p_screen, 5, RED.filled())))?;
    chart.draw_series(std::iter::once(Text::new(
        "P (2,3,4)",
        (p_screen.0 + 0.15, p_screen.1 - 0.2),
        ("sans-serif", 16).into_font(),
    )))?;

    // ---------- Proyección Q sobre el plano (z=0) ----------
    let q_3d = (2.0, 3.0, 0.0);
    let q_screen = proyectar(q_3d.0, q_3d.1, q_3d.2);
    chart.draw_series(std::iter::once(Circle::new(q_screen, 5, BLUE.filled())))?;
    chart.draw_series(std::iter::once(Text::new(
        "Q (2,3,0)",
        (q_screen.0 + 0.15, q_screen.1 - 0.2),
        ("sans-serif", 16).into_font(),
    )))?;

    // ---------- Segmento perpendicular P-Q ----------
    let segmento = [p_screen, q_screen];
    let mut estilo_segmento = ShapeStyle::from(&RED.mix(0.7));
    estilo_segmento.stroke_width = 2;
    chart.draw_series(LineSeries::new(segmento, estilo_segmento))?;

    // ---------- Símbolo de ángulo recto en Q (en el espacio 3D) ----------
    // Calculamos la dirección del segmento PQ en la pantalla y dibujamos un pequeño cuadrado
    let dx = p_screen.0 - q_screen.0;
    let dy = p_screen.1 - q_screen.1;
    let largo = (dx*dx + dy*dy).sqrt();
    if largo > 0.0 {
        let ux = dx / largo;
        let uy = dy / largo;
        let perp_x = -uy;
        let perp_y = ux;
        let size = 0.15;
        let simbolo = [
            (q_screen.0 - ux * size, q_screen.1 - uy * size),
            (q_screen.0 - ux * size + perp_x * size, q_screen.1 - uy * size + perp_y * size),
            (q_screen.0 + perp_x * size, q_screen.1 + perp_y * size),
        ];
        chart.draw_series(std::iter::once(PathElement::new(simbolo.to_vec(), &BLACK)))?;
    }

    // ---------- Arco que muestra que Q es el punto más cercano ----------
    let radio = ((p_screen.0 - q_screen.0).powi(2) + (p_screen.1 - q_screen.1).powi(2)).sqrt();
    let angulo_q = (q_screen.1 - p_screen.1).atan2(q_screen.0 - p_screen.0);
    let angulo_inicio = angulo_q - 0.5;
    let angulo_fin = angulo_q + 0.5;
    let puntos_arco: Vec<(f64, f64)> = (0..=30)
        .map(|i| {
            let t = angulo_inicio + (angulo_fin - angulo_inicio) * (i as f64) / 30.0;
            (p_screen.0 + radio * t.cos(), p_screen.1 + radio * t.sin())
        })
        .collect();
    chart.draw_series(LineSeries::new(puntos_arco, &BLUE.mix(0.5)))?;

    root.present()?;
    println!("✅ Imagen guardada como '{}'", nombre_archivo);
    Ok(())
}