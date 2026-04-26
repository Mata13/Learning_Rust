use plotters::prelude::*;
use std::f64::consts::FRAC_PI_2;

// ---------- MATRIZ DEL SISTEMA (nodo inestable) ----------
const A11: f64 = 2.0;
const A12: f64 = 0.0;
const A21: f64 = 0.0;
const A22: f64 = 1.0;

fn ode_system(_t: f64, x: f64, y: f64) -> (f64, f64) {
    (A11 * x + A12 * y, A21 * x + A22 * y)
}

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

fn valores_propios() -> String {
    let traza = A11 + A22;
    let determinante = A11 * A22 - A12 * A21;
    let discriminante = traza * traza - 4.0 * determinante;
    if discriminante >= 0.0 {
        let raiz = discriminante.sqrt();
        let lambda1 = (traza + raiz) / 2.0;
        let lambda2 = (traza - raiz) / 2.0;
        format!("λ₁ = {:.3}, λ₂ = {:.3}", lambda1, lambda2)
    } else {
        let parte_real = traza / 2.0;
        let parte_imag = (-discriminante).sqrt() / 2.0;
        format!("λ = {:.3} ± {:.3} i", parte_real, parte_imag)
    }
}

fn tipo_sistema() -> &'static str {
    let traza = A11 + A22;
    let det = A11 * A22 - A12 * A21;
    let disc = traza * traza - 4.0 * det;
    if disc > 0.0 {
        if traza < 0.0 { "Nodo Estable" }
        else if traza > 0.0 { "Nodo Inestable" }
        else { "Punto Silla (Saddle)" }
    } else if disc < 0.0 {
        if traza < 0.0 { "Espiral Estable" }
        else if traza > 0.0 { "Espiral Inestable" }
        else { "Centro (órbitas periódicas)" }
    } else {
        if traza < 0.0 { "Nodo Degenerado Estable" }
        else if traza > 0.0 { "Nodo Degenerado Inestable" }
        else { "Caso degenerado" }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nombre_archivo = "nodo_inestable.png";

    let root = BitMapBackend::new(nombre_archivo, (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Retrato de Fase — Nodo Inestable", ("sans-serif", 30))
        .margin(15)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(-3.0_f64..3.0, -3.0_f64..3.0)?;

    chart.configure_mesh().draw()?;

    // ---------- CAMPO VECTORIAL ----------
    let paso = 0.3;
    let mut x = -3.0;
    while x <= 3.0 {
        let mut y = -3.0;
        while y <= 3.0 {
            let (dx, dy) = ode_system(0.0, x, y);
            let largo = (dx*dx + dy*dy).sqrt();
            if largo > 0.001 {
                let escala = 0.2 / largo;
                let (x_fin, y_fin) = (x + dx * escala, y + dy * escala);
                // línea roja
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![(x, y), (x_fin, y_fin)],
                    &RED,
                )))?;
                // punta de flecha
                let largo_punta = 0.06;
                let ancho_punta = 0.04;
                let angulo = dy.atan2(dx);
                let bx = x_fin - largo_punta * angulo.cos();
                let by = y_fin - largo_punta * angulo.sin();
                let p1 = (bx + ancho_punta * (angulo + FRAC_PI_2).cos(),
                          by + ancho_punta * (angulo + FRAC_PI_2).sin());
                let p2 = (bx - ancho_punta * (angulo + FRAC_PI_2).cos(),
                          by - ancho_punta * (angulo + FRAC_PI_2).sin());
                chart.draw_series(std::iter::once(Polygon::new(
                    vec![(x_fin, y_fin), p1, p2],
                    BLACK.filled(),   // ✅ sin &
                )))?;
            }
            y += paso;
        }
        x += paso;
    }

    // ---------- TRAYECTORIAS ----------
    let t_final = 10.0;
    let dt = 0.02;
    let pasos = (t_final / dt) as usize;
    for &x0 in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
        for &y0 in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
            if x0 == 0.0 && y0 == 0.0 { continue; }
            let mut cx = x0;
            let mut cy = y0;
            let mut puntos = Vec::with_capacity(pasos + 1);
            puntos.push((cx, cy));
            for _ in 0..pasos {
                let (nx, ny) = rk4_step(ode_system, 0.0, cx, cy, dt);
                cx = nx;
                cy = ny;
                if cx.abs() > 10.0 || cy.abs() > 10.0 {
                    puntos.push((cx.max(-10.0).min(10.0), cy.max(-10.0).min(10.0)));
                break;
                }
                puntos.push((cx, cy));
            }
            // ✅ estilo con stroke_width=2
            let mut estilo_linea = ShapeStyle::from(&BLUE.mix(0.4));
            estilo_linea.stroke_width = 2;
            chart.draw_series(LineSeries::new(puntos, estilo_linea))?;
        }
    }

    // ---------- LEYENDA ----------
    let caja_x0 = -2.95;
    let caja_y0 = 2.7;
    let caja_x1 = 2.95;
    let caja_y1 = 2.95;

    chart.draw_series(std::iter::once(Rectangle::new(
        [(caja_x0, caja_y0), (caja_x1, caja_y1)],
        WHITE.mix(0.8).filled(),   // ✅ sin &
    )))?;

    let estilo_texto = TextStyle::from(("sans-serif", 18).into_font()).color(&BLACK);
    let texto_leyenda = format!("Tipo: {}   |   {}", tipo_sistema(), valores_propios());
    chart.draw_series(std::iter::once(Text::new(
        texto_leyenda,
        (0.0, caja_y1 - 0.12),
        estilo_texto,
    )))?;

    root.present()?;
    println!("✅ Imagen guardada como '{}'", nombre_archivo);
    Ok(())
}