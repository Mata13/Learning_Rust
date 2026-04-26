# Atlas de Retratos de Fase para Sistemas Lineales 2D

**Autor:** [Tu nombre]  
**Fecha:** 2026-04-25  
**Materia:** [Nombre de la materia / curso]

---

## 1. Introducción

Los sistemas de ecuaciones diferenciales lineales de la forma  

\[
\mathbf{x}'(t) = A\,\mathbf{x}(t),\qquad \mathbf{x}(t) \in \mathbb{R}^2,\; A \in \mathbb{R}^{2\times 2}
\]

modelan una gran variedad de fenómenos físicos y biológicos (mecánica orbital, circuitos eléctricos, dinámica de poblaciones, etc.).  
El comportamiento de las soluciones cerca del punto fijo \(\mathbf{x}^* = (0,0)\) está determinado completamente por los **valores propios** de la matriz \(A\).

En este informe se presentan los **seis retratos de fase cualitativamente distintos** para sistemas lineales bidimensionales, generados computacionalmente con el lenguaje **Rust** y la librería gráfica \plotters\.

---

## 2. Marco teórico

Dado el sistema \(\mathbf{x}' = A\mathbf{x}\), los valores propios \(\lambda\) se obtienen resolviendo la ecuación característica  

\[
\det(A - \lambda I) = \lambda^2 - \operatorname{tr}(A)\,\lambda + \det(A) = 0 .
\]

La clasificación depende del discriminante \(\Delta = \operatorname{tr}(A)^2 - 4\det(A)\) y de los signos de la traza y el determinante.

### Clasificación mediante valores propios

| Discriminante | Valores propios | Condición adicional | Tipo de punto fijo |
|---------------|----------------|---------------------|---------------------|
| \(\Delta > 0\) | Reales distintos | Ambos negativos | Nodo estable (atractor) |
| \(\Delta > 0\) | Reales distintos | Ambos positivos | Nodo inestable (repulsor) |
| \(\Delta > 0\) | Reales distintos | Signos opuestos | Punto silla (saddle) |
| \(\Delta < 0\) | Complejos conjugados | Parte real < 0 | Espiral estable (foco atractor) |
| \(\Delta < 0\) | Complejos conjugados | Parte real > 0 | Espiral inestable (foco repulsor) |
| \(\Delta < 0\) | Imaginarios puros | Parte real = 0 | Centro (órbitas periódicas) |

---

## 3. Metodología computacional

Los retratos de fase fueron generados mediante un **script en Rust** (\src/bin/\ de este repositorio) que:

1. Define la matriz \(A\) con constantes \A11..A22\.
2. Dibuja el campo vectorial con flechas rojas que indican la dirección \((dx/dt, dy/dt)\).
3. Integra numéricamente trayectorias con el método de **Runge-Kutta 4** desde varias condiciones iniciales.
4. Superpone una leyenda con el tipo de sistema detectado automáticamente y los valores propios calculados.

---

## 4. Atlas de casos

Para cada caso se muestra el retrato de fase, los valores propios correspondientes y una breve interpretación.

### 4.1 Nodo Estable (Atractor)

![Nodo Estable](./nodo_estable_lujo.png)

**Matriz:**  
\[
A = \begin{pmatrix} -2 & 0 \\ 0 & -1 \end{pmatrix}
\]

**Valores propios:** \(\lambda_1 = -2,\; \lambda_2 = -1\)

**Comportamiento:**  
Ambos valores propios son reales y negativos. Todas las trayectorias convergen al origen. El origen es un **atractor**; cualquier perturbación pequeña decae con el tiempo. Físicamente, corresponde a un sistema con fricción dominante.

---

### 4.2 Nodo Inestable (Repulsor)

![Nodo Inestable](./nodo_inestable_lujo.png)

**Matriz:**  
\[
A = \begin{pmatrix} 2 & 0 \\ 0 & 1 \end{pmatrix}
\]

**Valores propios:** \(\lambda_1 = 2,\; \lambda_2 = 1\)

**Comportamiento:**  
Ambos valores propios son reales y positivos. Todas las trayectorias se alejan del origen. El origen es un **repulsor**. Representa, por ejemplo, el equilibrio inestable de una montaña o un cohete descontrolado.

---

### 4.3 Punto Silla (Saddle)

![Punto Silla](./silla_lujo.png)

**Matriz:**  
\[
A = \begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix}
\]

**Valores propios:** \(\lambda_1 = 1,\; \lambda_2 = -1\)

**Comportamiento:**  
Un valor propio positivo y otro negativo. Existen dos direcciones que atraen hacia el origen y dos que repelen. La mayoría de las trayectorias se acercan siguiendo la dirección estable y luego se alejan por la inestable, formando hipérbolas. Es el prototipo de **silla de montar**.

---

### 4.4 Espiral Estable (Foco Atractor)

![Espiral Estable](./espiral_estable_lujo.png)

**Matriz:**  
\[
A = \begin{pmatrix} -0.2 & 1 \\ -1 & -0.2 \end{pmatrix}
\]

**Valores propios:** \(\lambda = -0.2 \pm 1.0\,i\)

**Comportamiento:**  
Valores propios complejos con parte real negativa. Las trayectorias giran en espiral hacia el origen, como el agua en un desagüe. Corresponde a un oscilador subamortiguado.

---

### 4.5 Espiral Inestable (Foco Repulsor)

![Espiral Inestable](./espiral_inestable_lujo.png)

**Matriz:**  
\[
A = \begin{pmatrix} 0.2 & 1 \\ -1 & 0.2 \end{pmatrix}
\]

**Valores propios:** \(\lambda = 0.2 \pm 1.0\,i\)

**Comportamiento:**  
Complejos con parte real positiva. Las soluciones se alejan del origen girando. Describe un sistema que amplifica oscilaciones, como un puente con retroalimentación positiva.

---

### 4.6 Centro (Órbitas Periódicas)

![Centro](./centro_lujo.png)

**Matriz:**  
\[
A = \begin{pmatrix} 0 & 1 \\ -1 & 0 \end{pmatrix}
\]

**Valores propios:** \(\lambda = \pm 1.0\,i\)

**Comportamiento:**  
Valores propios imaginarios puros. Las trayectorias son curvas cerradas (elipses o círculos) y no se acercan ni se alejan del origen. Representa un oscilador armónico simple sin amortiguamiento.

---

## 5. Conclusiones

La simple inspección de los valores propios permite predecir el comportamiento cualitativo del sistema sin necesidad de resolver las ecuaciones.  
Los retratos de fase generados confirman la clasificación teórica y facilitan la comprensión intuitiva de conceptos como estabilidad, cuencas de atracción y sensibilidad a condiciones iniciales.

Además, el uso de Rust y \plotters\ demuestra que es posible realizar visualizaciones científicas de alta calidad con herramientas de código abierto, combinando eficiencia computacional y control preciso sobre el resultado gráfico.

---

## 6. Anexo: Código fuente

Los scripts utilizados se encuentran en el directorio \src/bin/\ del repositorio \Learning_Rust\. Para cada caso se empleó la misma plantilla, modificando únicamente las constantes de la matriz y los títulos.  
El código integra automáticamente el método de Runge-Kutta 4 y dibuja tanto el campo vectorial como las trayectorias.

---

## 7. Referencias

- Hirsch, M. W., Smale, S., & Devaney, R. L. (2004). *Differential Equations, Dynamical Systems, and an Introduction to Chaos*. Academic Press.
- Strogatz, S. H. (2015). *Nonlinear Dynamics and Chaos*. Westview Press.
- Documentación oficial de Rust: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- Crate \plotters\: [https://crates.io/crates/plotters](https://crates.io/crates/plotters)
