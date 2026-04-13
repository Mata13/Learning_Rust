# 🚀 Learning Rust — Space Science & No Man's Sky Edition

> *"El universo es un sistema de sistemas. Rust es el lenguaje para modelarlos."*

Repositorio de autoaprendizaje del lenguaje de programación **Rust**, documentado progresivamente por un **Space Scientist y Matemático**. Todos los ejercicios y ejemplos están contextualizados en temas de **astrofísica, mecánica orbital, cosmología** y el universo procedural de **No Man's Sky** — porque aprender es más poderoso cuando el tema te apasiona.

**Meta final:** construir un solver de ecuaciones diferenciales ordinarias (ODE) en Rust, conectado a Python vía PyO3, para acelerar simulaciones espaciales.

---

## 👨‍🚀 Sobre este repositorio

| Campo | Detalle |
|---|---|
| Lenguaje | Rust 1.94+ |
| Enfoque | Space Science · Astrofísica · No Man's Sky |
| Nivel inicial | Conocimiento previo en Python, algo de JS y C# |
| Meta técnica | ODE solver en Rust + bridge con Python (PyO3) |
| Entorno | VSCode + rust-analyzer + Cargo |
| Sistema | Windows 11 / PowerShell |

---

## 🗺️ Mapa de aprendizaje

```
Fase 1  → Fundamentos: variables, tipos, funciones, control de flujo
Fase 2  → Ownership & borrowing (el corazón de Rust)
Fase 3  → Structs, enums y pattern matching
Fase 4  → Iterators, closures y error handling
Fase 5  → Traits y generics
Fase 6  → Rust + Python con PyO3 (meta final)
```

---

## 📁 Estructura del proyecto

```
Learning_Rust/
│
├── README.md                        ← este archivo
├── Cargo.toml                       ← configuración del proyecto
├── .gitignore                       ← excluye /target y archivos temporales
│
└── src/
    ├── main.rs                      ← punto de entrada principal
    └── bin/                         ← ejercicios independientes por tema
        │
        ├── fase_1_fundamentos/
        │   ├── tipos_variables_datos.rs
        │   ├── energia_cinetica.rs
        │   ├── clasificador_planetas.rs
        │   ├── kepler_tercera_ley.rs
        │   ├── telemetria_nave.rs
        │   ├── stefan_boltzmann.rs
        │   ├── schwarzschild.rs
        │   ├── gravedad_superficial.rs
        │   └── tsiolkovsky.rs
        │
        ├── fase_2_ownership/        ← próximamente
        ├── fase_3_structs/          ← próximamente
        ├── fase_4_iterators/        ← próximamente
        ├── fase_5_traits/           ← próximamente
        └── fase_6_python_bridge/    ← meta final
```

---

## ⚙️ Requisitos e instalación

### 1. Instalar Rust

```bash
# En Windows (PowerShell)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificar instalación
rustc --version
cargo --version
```

### 2. Clonar este repositorio

```bash
git clone https://github.com/TU_USUARIO/Learning_Rust.git
cd Learning_Rust
```

### 3. Extensión recomendada en VSCode

- **rust-analyzer** — autocompletado, errores en tiempo real, documentación inline

---

## ▶️ Cómo ejecutar los ejercicios

```powershell
# Ejecutar un ejercicio específico
cargo run --bin schwarzschild

# Solo verificar errores (más rápido, sin ejecutar)
cargo check --bin schwarzschild

# Compilar todo el proyecto
cargo build
```

> **Nota:** Siempre ejecuta los comandos desde la raíz del proyecto (`Learning_Rust/`), nunca desde dentro de `src/`.

---

## 🌌 Fase 1 — Fundamentos de Rust

Conceptos cubiertos: `let`, `mut`, tipos primitivos (`f64`, `i32`, `bool`, `&str`, `String`), funciones (`fn`), constantes (`const`), `if/else`, arrays, tuplas, bucles `for`, formato de output.

### Ejercicios

| Archivo | Física | Conceptos Rust |
|---|---|---|
| `tipos_variables_datos.rs` | Velocidad de la luz, escape terrestre | `let`, `mut`, tipos `f64` |
| `energia_cinetica.rs` | E = ½mv² — propulsión NMS | `fn`, `.powi()`, formato `{:.3e}` |
| `clasificador_planetas.rs` | Zona habitable, temperatura superficial | `if/else`, arrays, `for`, tuples |
| `kepler_tercera_ley.rs` | T² ∝ a³ — Sistema Solar completo | `const`, `use`, arrays tipados |
| `telemetria_nave.rs` | g, v_escape, radio de Hill, Hawking | múltiples `fn`, `String` vs `&str` |
| `stefan_boltzmann.rs` | L = 4πR²σT⁴ — luminosidad estelar | `const SIGMA`, clasificación espectral |
| `schwarzschild.rs` | r_s = 2GM/c² — horizonte de eventos | temperatura de Hawking, densidad |
| `gravedad_superficial.rs` | g = GM/R² en planetas NMS | altura de salto, `format!()` |
| `tsiolkovsky.rs` | Δv = v_e·ln(m₀/mf) — cohetes | `.ln()`, `.exp()`, viabilidad de misión |

---

## ⚫ Ejemplo destacado — Agujero Negro (Schwarzschild)

```rust
const G: f64 = 6.674e-11;
const C: f64 = 2.998e8;

fn radio_schwarzschild(masa_kg: f64) -> f64 {
    (2.0 * G * masa_kg) / C.powi(2)
}

fn main() {
    let masa_sol = 1.989e30;
    let rs = radio_schwarzschild(21.2 * masa_sol); // Cygnus X-1
    println!("Radio de Schwarzschild: {:.3} km", rs / 1000.0);
    // Output: Radio de Schwarzschild: 62.481 km
}
```

---

## 🪐 Analogía central de este repositorio

```
Python  →  traje de exploración: cómodo, flexible, ideal para prototipar
Rust    →  motor de pulso: rápido, seguro, sin garbage collector
PyO3    →  interfaz nave-traje: conecta ambos mundos
Tú      →  el Comandante que decide cuándo usar cada sistema
```

---

## 📐 Física implementada

| Ley / Ecuación | Área | Ejercicio |
|---|---|---|
| E = ½mv² | Mecánica clásica | `energia_cinetica.rs` |
| T² = 4π²a³/GM | Mecánica orbital (Kepler) | `kepler_tercera_ley.rs` |
| L = 4πR²σT⁴ | Astrofísica estelar | `stefan_boltzmann.rs` |
| r_s = 2GM/c² | Relatividad general | `schwarzschild.rs` |
| g = GM/R² | Gravitación universal | `gravedad_superficial.rs` |
| Δv = v_e·ln(m₀/mf) | Astronáutica | `tsiolkovsky.rs` |
| v_esc = √(2GM/R) | Mecánica orbital | `telemetria_nave.rs` |

---

## 🛠️ Herramientas y recursos

### Recursos usados en este aprendizaje

- [The Rust Book](https://doc.rust-lang.org/book/) — documentación oficial
- [Rust Playground](https://play.rust-lang.org/) — probar código sin instalar nada
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — ejemplos prácticos
- [PyO3 Guide](https://pyo3.rs/) — bridge Rust ↔ Python (fase final)

### Stack de desarrollo

```
Rust 1.94       — compilador y toolchain
Cargo 1.94      — gestor de paquetes y build system
VSCode          — editor principal
rust-analyzer   — extensión LSP para VSCode
PowerShell      — terminal en Windows
Git             — control de versiones
```

---

## 📈 Progreso

- [x] Fase 1 — Fundamentos (variables, tipos, funciones, control de flujo)
- [ ] Fase 2 — Ownership & borrowing
- [ ] Fase 3 — Structs, enums, pattern matching
- [ ] Fase 4 — Iterators, closures, error handling
- [ ] Fase 5 — Traits y generics
- [ ] Fase 6 — ODE solver + bridge Python/Rust con PyO3

---

## 📝 Bitácora de aprendizaje

### Fase 1 — Completada ✅

**Lo más importante aprendido:**
- Las variables en Rust son **inmutables por defecto** — debes declarar `mut` explícitamente si necesitas cambiarlas. Esto previene bugs que en Python pasarían silenciosos.
- Rust siempre necesita saber el **tipo de dato en tiempo de compilación**. Para física y matemáticas, siempre usar `f64` (equivalente a `float64` de NumPy).
- El compilador de Rust es tu mejor mentor — sus `warning` y `error` son detallados, apuntan exactamente al problema y sugieren cómo resolverlo.
- `cargo run --bin nombre` compila y ejecuta. `cargo check --bin nombre` solo verifica errores (más rápido mientras escribes).

---

## 🤝 Contribuciones

Este es un repositorio de aprendizaje personal, pero si encuentras un error físico en alguna ecuación o una mejor forma de escribir algún ejercicio en Rust, un PR o Issue es bienvenido.

---

## 📄 Licencia

MIT — libre para usar, modificar y compartir con atribución.

---

<div align="center">

*Construido con Rust 🦀 · Inspirado en el universo 🌌 · Documentado para aprender en público*

</div>
