use plotters::prelude::*; // todas las herramientas de dibujo

/// Matriz del sistema A (nodo estable)
const A11: f64 = -2.0;
const A12: f64 = 0.0;
const A21: f64 = 0.0;
const A22: f64 = -1.0;

/// La función del sistema: dado el estado (x, y) devuelve (dx/dt, dy/dt)
fn ode_system(_t: f64, x: f64, y: f64) -> (f64, f64) {
    (
        A11 * x + A12 * y,
        A21 * x + A22 * y,
    )
}

/// Un paso del método de Runge-Kutta 4 (RK4)
fn rk4_step(
    f: impl Fn(f64, f64, f64) -> (f64, f64),
    t: f64,
    x: f64,
    y: f64,
    h: f64,
) -> (f64, f64) {
    let (k1x, k1y) = f(t, x, y);
    let (k2x, k2y) = f(t + h / 2.0, x + h / 2.0 * k1x, y + h / 2.0 * k1y);
    let (k3x, k3y) = f(t + h / 2.0, x + h / 2.0 * k2x, y + h / 2.0 * k2y);
    let (k4x, k4y) = f(t + h, x + h * k3x, y + h * k3y);

    let nx = x + (h / 6.0) * (k1x + 2.0 * k2x + 2.0 * k3x + k4x);
    let ny = y + (h / 6.0) * (k1y + 2.0 * k2y + 2.0 * k3y + k4y);
    (nx, ny)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Preparar el lienzo ---
    let nombre_archivo = "nodo_estable.png";
    let root = BitMapBackend::new(nombre_archivo, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Configurar el sistema de coordenadas (de -3 a 3 en ambos ejes)
    let mut chart = ChartBuilder::on(&root)
        .caption("Nodo Estable (Atractor)", ("sans-serif", 28))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-3.0_f64..3.0, -3.0_f64..3.0)?;

    chart.configure_mesh().draw()?;

    // --- Dibujar campo vectorial como flechitas ---
    let paso = 0.3;        // cada cuánto dibujamos una flecha
    let mut x = -3.0;
    while x <= 3.0 {
        let mut y = -3.0;
        while y <= 3.0 {
            let (dx, dy) = ode_system(0.0, x, y);
            // Escalamos la flecha para que sea visible pero no gigante
            let largo = (dx * dx + dy * dy).sqrt();
            if largo > 0.001 {
                let escala = 0.2 / largo;
                let (x_fin, y_fin) = (x + dx * escala, y + dy * escala);
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![(x, y), (x_fin, y_fin)],
                    &RED,
                )))?;
            }
            y += paso;
        }
        x += paso;
    }

    // --- Calcular y dibujar trayectorias con RK4 ---
    let t_final = 5.0;
    let dt = 0.02;
    let pasos = (t_final / dt) as usize;
    for &x0 in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
        for &y0 in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
            if x0 == 0.0 && y0 == 0.0 {
                continue; // el origen ya es punto fijo
            }
            let mut cx = x0;
            let mut cy = y0;
            let mut puntos: Vec<(f64, f64)> = Vec::with_capacity(pasos + 1);
            puntos.push((cx, cy));
            for _ in 0..pasos {
                let (nx, ny) = rk4_step(ode_system, 0.0, cx, cy, dt);
                puntos.push((nx, ny));
                cx = nx;
                cy = ny;
            }
            // Dibujar la trayectoria como una línea azul
            chart.draw_series(LineSeries::new(puntos, &BLUE.mix(0.6)))?
                 .label("trayectoria")
                 .legend(move |(x, y)| {
                     PathElement::new(vec![(x, y), (x + 20, y)], &BLUE.mix(0.6))
                 });
        }
    }

    // Guardar la imagen en disco
    root.present()?;
    println!("✅ Imagen guardada como '{}'", nombre_archivo);
    Ok(())
}