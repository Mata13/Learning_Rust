use plotters::prelude::*;
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("output")?;
    let nombre_archivo = "output/proyeccion_ortogonal.png";

    let root = BitMapBackend::new(nombre_archivo, (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let dominio_x = -2.0_f64..4.0;
    let dominio_y = -2.0_f64..4.0;

    let mut chart = ChartBuilder::on(&root)
        .caption("Teorema de la proyección ortogonal en R²", ("sans-serif", 24))
        .margin(15)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(dominio_x, dominio_y)?;

    chart.configure_mesh()
        .x_desc("x")
        .y_desc("y")
        .draw()?;

    // ---------- Subespacio M: recta y = x ----------
    let recta = [(-2.0, -2.0), (4.0, 4.0)];
    chart.draw_series(LineSeries::new(recta, GREEN.stroke_width(2)))?
        .label("Subespacio M: y = x");

    // ---------- Punto P y su proyección Q ----------
    let p = (3.0, 1.0);
    // Proyección sobre y = x
    let q = ((p.0 + p.1) / 2.0, (p.0 + p.1) / 2.0);

    // Dibujar P (rojo)
    chart.draw_series(std::iter::once(Circle::new(p, 5, RED.filled())))?;
    chart.draw_series(std::iter::once(Text::new(
        "P",
        (p.0 + 0.15, p.1 - 0.2),
        ("sans-serif", 16).into_font(),
    )))?;

    // Dibujar Q (azul)
    chart.draw_series(std::iter::once(Circle::new(q, 5, BLUE.filled())))?;
    chart.draw_series(std::iter::once(Text::new(
        "Q",
        (q.0 + 0.15, q.1 - 0.2),
        ("sans-serif", 16).into_font(),
    )))?;

    // ---------- Segmento perpendicular P-Q ----------
    let segmento = [p, q];
    chart.draw_series(LineSeries::new(segmento, RED.mix(0.7).stroke_width(2)))?;

    // ---------- Símbolo de ángulo recto en Q ----------
    let perp_dir = (-0.2, 0.2);   // perpendicular a (1,1)
    let along_dir = (0.2, 0.2);
    let simbolo = [
        (q.0 - along_dir.0, q.1 - along_dir.1),
        (q.0 - along_dir.0 + perp_dir.0, q.1 - along_dir.1 + perp_dir.1),
        (q.0 + perp_dir.0, q.1 + perp_dir.1),
    ];
    chart.draw_series(std::iter::once(PathElement::new(simbolo.to_vec(), &BLACK)))?;

    // ---------- Arco de circunferencia: muestra que Q es el punto más cercano ----------
    let radio = ((p.0 - q.0).powi(2) + (p.1 - q.1).powi(2)).sqrt();
    let angulo_q = (q.1 - p.1).atan2(q.0 - p.0);
    let angulo_inicio = angulo_q - PI / 6.0;
    let angulo_fin = angulo_q + PI / 6.0;
    let puntos_arco: Vec<(f64, f64)> = (0..=40)
        .map(|i| {
            let t = angulo_inicio + (angulo_fin - angulo_inicio) * (i as f64) / 40.0;
            (p.0 + radio * t.cos(), p.1 + radio * t.sin())
        })
        .collect();
    chart.draw_series(LineSeries::new(puntos_arco, BLUE.mix(0.5)))?;

    // ---------- Leyenda ----------
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("✅ Imagen guardada como '{}'", nombre_archivo);
    Ok(())
}