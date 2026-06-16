use plotters::prelude::*;

// ---------- SISTEMA NO AUTÓNOMO ----------
fn ode_system(t: f64, x: f64, y: f64) -> (f64, f64) {
    // 1a) x'' + 3x' + 2x = t² - t
    let dx = y;
    let dy = -3.0 * y - 2.0 * x + t * t - t;
    (dx, dy)
}

// ---------- RK4 con dependencia temporal ----------
fn rk4_step(
    f: impl Fn(f64, f64, f64) -> (f64, f64),
    t: f64, x: f64, y: f64, h: f64,
) -> (f64, f64) {
    let (k1x, k1y) = f(t, x, y);
    let (k2x, k2y) = f(t + h/2.0, x + h/2.0*k1x, y + h/2.0*k1y);
    let (k3x, k3y) = f(t + h/2.0, x + h/2.0*k2x, y + h/2.0*k2y);
    let (k4x, k4y) = f(t + h, x + h*k3x, y + h*k3y);
    (
        x + (h/6.0)*(k1x + 2.0*k2x + 2.0*k3x + k4x),
        y + (h/6.0)*(k1y + 2.0*k2y + 2.0*k3y + k4y),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Crear carpeta de salida si no existe
    std::fs::create_dir_all("output")?;
    let nombre_archivo = "output/tarea_ej1a.png";

    let root = BitMapBackend::new(nombre_archivo, (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Plano fase — x'' + 3x' + 2x = t² - t",
            ("sans-serif", 26),
        )
        .margin(15)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(-4.0_f64..4.0, -4.0_f64..4.0)?;

    chart.configure_mesh().draw()?;

    // ---------- TRAYECTORIAS (sin campo vectorial) ----------
    let t_final = 10.0;
    let dt = 0.02;
    let pasos = (t_final / dt) as usize;

    // Malla de condiciones iniciales (x0, y0)
    for &x0 in &[-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0] {
        for &y0 in &[-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0] {
            let mut t = 0.0;
            let mut cx = x0;
            let mut cy = y0;
            let mut puntos = Vec::with_capacity(pasos + 1);
            puntos.push((cx, cy));

            for _ in 0..pasos {
                let (nx, ny) = rk4_step(ode_system, t, cx, cy, dt);
                t += dt;
                cx = nx;
                cy = ny;

                // Evitar que se dispare fuera de la ventana visible
                if cx.abs() > 10.0 || cy.abs() > 10.0 {
                    break;
                }
                puntos.push((cx, cy));
            }

            // Dibujar trayectoria en azul semitransparente
            let mut estilo = ShapeStyle::from(&BLUE.mix(0.4));
            estilo.stroke_width = 2;
            chart.draw_series(LineSeries::new(puntos, estilo))?;
        }
    }

    // ---------- LEYENDA (opcional) ----------
    let estilo_texto = TextStyle::from(("sans-serif", 18).into_font()).color(&BLACK);
    chart.draw_series(std::iter::once(Text::new(
        "x' = y,  y' = -3y - 2x + t² - t",
        (0.0, 3.8),
        estilo_texto,
    )))?;

    root.present()?;
    println!("✅ Imagen guardada como '{}'", nombre_archivo);
    Ok(())
}