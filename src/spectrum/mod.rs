mod data;

use crate::auxiliar;
use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub enum SpectrumType {
    Reflectance,
    Illuminant,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CoefficientSpectrum<const N: usize> {
    coefficients: [f32; N],
}

impl<const N: usize> CoefficientSpectrum<N> {
    /// Crea un nuevo SPD constante.
    #[inline]
    pub const fn new(value: f32) -> CoefficientSpectrum<N> {
        CoefficientSpectrum {
            coefficients: [value; N],
        }
    }

    #[inline]
    pub fn is_black(&self) -> bool {
        self.coefficients.iter().all(|f| *f == 0.0)
    }

    #[inline]
    pub fn sqrt(&self) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        result.iter_mut().for_each(|f| *f = f.sqrt());

        CoefficientSpectrum {
            coefficients: result,
        }
    }

    #[inline]
    pub fn pow(&self, t: f32) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        result.iter_mut().for_each(|f| *f = f.powf(t));

        CoefficientSpectrum {
            coefficients: result,
        }
    }

    #[inline]
    pub fn exp(&self) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        result.iter_mut().for_each(|f| *f = f.exp());

        CoefficientSpectrum {
            coefficients: result,
        }
    }

    #[inline]
    pub fn lerp(
        &self,
        other: &CoefficientSpectrum<N>,
        t: f32,
    ) -> CoefficientSpectrum<N> {
        (1.0 - t) * self + t * other
    }

    #[inline]
    fn has_nan(&self) -> bool {
        self.coefficients.iter().any(|f| f.is_nan())
    }
}

impl<const N: usize> Add for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn add(self, rhs: CoefficientSpectrum<N>) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        for i in 0..N {
            result[i] += rhs.coefficients[i];
        }

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> Add for &CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn add(self, rhs: &CoefficientSpectrum<N>) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        for i in 0..N {
            result[i] += rhs.coefficients[i];
        }

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> AddAssign for CoefficientSpectrum<N> {
    #[inline]
    fn add_assign(&mut self, rhs: CoefficientSpectrum<N>) {
        for i in 0..N {
            self.coefficients[i] += rhs.coefficients[i];
        }
    }
}

impl<const N: usize> Sub for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn sub(self, rhs: CoefficientSpectrum<N>) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        for i in 0..N {
            result[i] -= rhs.coefficients[i];
        }

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> Mul for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn mul(self, rhs: CoefficientSpectrum<N>) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        for i in 0..N {
            result[i] *= rhs.coefficients[i];
        }

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> Mul for &CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn mul(self, rhs: &CoefficientSpectrum<N>) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        for i in 0..N {
            result[i] *= rhs.coefficients[i];
        }

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> Mul<f32> for &CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn mul(self, rhs: f32) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        result.iter_mut().for_each(|f| *f *= rhs);

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> Mul<&CoefficientSpectrum<N>> for f32 {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn mul(self, rhs: &CoefficientSpectrum<N>) -> CoefficientSpectrum<N> {
        rhs * self
    }
}

impl<const N: usize> Div for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn div(self, rhs: CoefficientSpectrum<N>) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        for i in 0..N {
            result[i] /= rhs.coefficients[i];
        }

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> Div for &CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn div(self, rhs: &CoefficientSpectrum<N>) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        for i in 0..N {
            result[i] /= rhs.coefficients[i];
        }

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> Div<f32> for &CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn div(self, rhs: f32) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        result.iter_mut().for_each(|f| *f /= rhs);

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

impl<const N: usize> Neg for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;

    #[inline]
    fn neg(self) -> CoefficientSpectrum<N> {
        let mut result = self.coefficients;

        result.iter_mut().for_each(|f| *f = -*f);

        CoefficientSpectrum {
            coefficients: result,
        }
    }
}

const SAMPLED_LAMBDA_START: f32 = 400.0;
const SAMPLED_LAMBDA_END: f32 = 700.0;
const N_SAMPLES: usize = 60;
pub type SampledSpectrum = CoefficientSpectrum<N_SAMPLES>;

// Funciones XYZ
static mut CIE_X: SampledSpectrum = SampledSpectrum::new(0.0);
static mut CIE_Y: SampledSpectrum = SampledSpectrum::new(0.0);
static mut CIE_Z: SampledSpectrum = SampledSpectrum::new(0.0);

// Funciones para pasar de RGB a SampledSpectrum
static mut RGB_REFL_2_SPECT_WHITE: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_REFL_2_SPECT_CYAN: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_REFL_2_SPECT_MAGEN: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_REFL_2_SPECT_YELLO: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_REFL_2_SPECT_RED: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_REFL_2_SPECT_GREEN: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_REFL_2_SPECT_BLUE: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_ILLUM_2_SPECT_WHITE: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_ILLUM_2_SPECT_CYAN: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_ILLUM_2_SPECT_MAGEN: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_ILLUM_2_SPECT_YELLO: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_ILLUM_2_SPECT_RED: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_ILLUM_2_SPECT_GREEN: SampledSpectrum = SampledSpectrum::new(0.0);
static mut RGB_ILLUM_2_SPECT_BLUE: SampledSpectrum = SampledSpectrum::new(0.0);

impl SampledSpectrum {
    /// Crea un SPD usando una lista de (frecuencia, muestra)
    pub fn from_sampled(samples: &[(f32, f32)]) -> SampledSpectrum {
        let mut samples = Vec::from(samples);
        samples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap()); // ver que pasa con los NaN

        let lambdas = samples.iter().map(|(l, _)| *l).collect::<Vec<_>>();
        let samples = samples.iter().map(|(_, s)| *s).collect::<Vec<_>>();

        let mut result = SampledSpectrum::new(0.0);

        for i in 0..N_SAMPLES {
            let l_0 = auxiliar::lerp(
                SAMPLED_LAMBDA_START,
                SAMPLED_LAMBDA_END,
                i as f32 / N_SAMPLES as f32,
            );
            let l_1 = auxiliar::lerp(
                SAMPLED_LAMBDA_START,
                SAMPLED_LAMBDA_END,
                (i + 1) as f32 / N_SAMPLES as f32,
            );

            result.coefficients[i] =
                Self::average_spectrum_sample(&lambdas, &samples, l_0, l_1);
        }

        result
    }

    /// Interpolo la muestra y calculo el área bajo la curva en el rango pedido.
    /// Luego lo divido por este rango.
    fn average_spectrum_sample(
        lambdas: &[f32],
        samples: &[f32],
        lambda_start: f32,
        lambda_end: f32,
    ) -> f32 {
        // Casos borde
        if samples.len() == 0 {
            return 0.0;
        } else if samples.len() == 1 {
            return samples[0];
        }

        let lambda_first = *lambdas.first().unwrap();
        let lambda_last = *lambdas.last().unwrap();
        let sample_first = *samples.first().unwrap();
        let sample_last = *samples.last().unwrap();

        if lambda_end <= lambda_first {
            return sample_first;
        } else if lambda_start >= lambda_last {
            return sample_last;
        }

        let mut sum = 0.0;

        // Sumo el area rectangular fuera de la muestra (tomo la curva como
        // constante)
        if lambda_start < lambda_first {
            sum += sample_first * (lambda_first - lambda_start);
        }
        if lambda_end > lambda_last {
            sum += sample_last * (lambda_end - lambda_last);
        }

        // busco i tal que lambda_start < samples[i+1].0
        // se podría hacer búsqueda binaria pero es al pedo
        let mut i = 0;
        while lambda_start > lambdas[i + 1] {
            i += 1;
        }

        // interpolo dos muestras
        let interp = |val: f32, k: usize| -> f32 {
            auxiliar::lerp(
                samples[k],
                samples[k + 1],
                (val - lambdas[k]) / (lambdas[k + 1] - lambdas[k]),
            )
        };

        while (i + 1) < samples.len() && lambda_end >= lambdas[i] {
            let l_start = lambdas[i].max(lambda_start);
            let l_end = lambdas[i + 1].min(lambda_end);

            sum += 0.5
                * (interp(l_start, i) + interp(l_end, i))
                * (l_end - l_start);

            i += 1;
        }

        sum / (lambda_end - lambda_start)
    }

    #[allow(non_snake_case)]
    pub fn init() {
        for i in 0..N_SAMPLES {
            let l_0 = auxiliar::lerp(
                SAMPLED_LAMBDA_START,
                SAMPLED_LAMBDA_END,
                i as f32 / N_SAMPLES as f32,
            );

            let l_1 = auxiliar::lerp(
                SAMPLED_LAMBDA_START,
                SAMPLED_LAMBDA_END,
                (i + 1) as f32 / N_SAMPLES as f32,
            );

            unsafe {
                // XYZ
                CIE_X.coefficients[i] = Self::average_spectrum_sample(
                    &data::CIE_LAMBDA,
                    &data::CIE_X,
                    l_0,
                    l_1,
                );
                CIE_Y.coefficients[i] = Self::average_spectrum_sample(
                    &data::CIE_LAMBDA,
                    &data::CIE_Y,
                    l_0,
                    l_1,
                );
                CIE_Z.coefficients[i] = Self::average_spectrum_sample(
                    &data::CIE_LAMBDA,
                    &data::CIE_Z,
                    l_0,
                    l_1,
                );

                // RGB to Spectrum
                RGB_REFL_2_SPECT_WHITE.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_REFL_2_SPECT_WHITE,
                        l_0,
                        l_1,
                    );
                RGB_REFL_2_SPECT_CYAN.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_REFL_2_SPECT_CYAN,
                        l_0,
                        l_1,
                    );
                RGB_REFL_2_SPECT_MAGEN.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_REFL_2_SPECT_MAGENTA,
                        l_0,
                        l_1,
                    );
                RGB_REFL_2_SPECT_YELLO.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_REFL_2_SPECT_YELLOW,
                        l_0,
                        l_1,
                    );
                RGB_REFL_2_SPECT_RED.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_REFL_2_SPECT_RED,
                        l_0,
                        l_1,
                    );
                RGB_REFL_2_SPECT_GREEN.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_REFL_2_SPECT_GREEN,
                        l_0,
                        l_1,
                    );
                RGB_REFL_2_SPECT_BLUE.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_REFL_2_SPECT_BLUE,
                        l_0,
                        l_1,
                    );
                RGB_ILLUM_2_SPECT_WHITE.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_ILLUM_2_SPECT_WHITE,
                        l_0,
                        l_1,
                    );
                RGB_ILLUM_2_SPECT_CYAN.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_ILLUM_2_SPECT_CYAN,
                        l_0,
                        l_1,
                    );
                RGB_ILLUM_2_SPECT_MAGEN.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_ILLUM_2_SPECT_MAGENTA,
                        l_0,
                        l_1,
                    );
                RGB_ILLUM_2_SPECT_YELLO.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_ILLUM_2_SPECT_YELLOW,
                        l_0,
                        l_1,
                    );
                RGB_ILLUM_2_SPECT_RED.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_ILLUM_2_SPECT_RED,
                        l_0,
                        l_1,
                    );
                RGB_ILLUM_2_SPECT_GREEN.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_ILLUM_2_SPECT_GREEN,
                        l_0,
                        l_1,
                    );
                RGB_ILLUM_2_SPECT_BLUE.coefficients[i] =
                    Self::average_spectrum_sample(
                        &data::RGB_2_SPECT_LAMBDA,
                        &data::RGB_ILLUM_2_SPECT_BLUE,
                        l_0,
                        l_1,
                    );
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn from_RGB((r, g, b): (f32, f32, f32), type_: SpectrumType) -> Self {
        let (white, cyan, magenta, yellow, red, green, blue) = unsafe {
            match type_ {
                SpectrumType::Reflectance => (
                    &RGB_REFL_2_SPECT_WHITE, &RGB_REFL_2_SPECT_CYAN,
                    &RGB_REFL_2_SPECT_MAGEN, &RGB_REFL_2_SPECT_YELLO,
                    &RGB_REFL_2_SPECT_RED, &RGB_REFL_2_SPECT_GREEN,
                    &RGB_REFL_2_SPECT_BLUE,
                ),
                SpectrumType::Illuminant => (
                    &RGB_ILLUM_2_SPECT_WHITE, &RGB_ILLUM_2_SPECT_CYAN,
                    &RGB_ILLUM_2_SPECT_MAGEN, &RGB_ILLUM_2_SPECT_YELLO,
                    &RGB_ILLUM_2_SPECT_RED, &RGB_ILLUM_2_SPECT_GREEN,
                    &RGB_ILLUM_2_SPECT_BLUE,
                ),
            }
        };

        let mut result = CoefficientSpectrum::new(0.0);

        // agarro el color más chico de los tres, lo sumo como blanco y lo resto
        // de los otros dos colores. repito sumando cyan, magenta o yellow, y me
        // queda un solo color RGB para sumar
        if r < g && r < b {
            result += white * r;

            if g < b {
                result += cyan * (g - r);
                result += blue * (b - g);
            } else {
                result += cyan * (b - r);
                result += green * (g - b);
            }
        } else if g < r && g < b {
            result += white * g;

            if r < b {
                result += magenta * (r - g);
                result += blue * (b - r);
            } else {
                result += magenta * (b - g);
                result += red * (r - b);
            }
        } else {
            result += white * b;

            if r < g {
                result += yellow * (r - b);
                result += green * (g - r);
            } else {
                result += yellow * (g - b);
                result += red * (r - g);
            }
        }

        result
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn from_XYZ((x, y, z): (f32, f32, f32), type_: SpectrumType) -> Self {
        Self::from_RGB(XYZ_to_RGB((x, y, z)), type_)
    }

    #[allow(non_snake_case)]
    pub fn to_XYZ(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);

        for i in 0..N_SAMPLES {
            unsafe {
                x += CIE_X.coefficients[i] * self.coefficients[i];
                y += CIE_Y.coefficients[i] * self.coefficients[i];
                z += CIE_Z.coefficients[i] * self.coefficients[i];
            }
        }

        let scale = (SAMPLED_LAMBDA_END - SAMPLED_LAMBDA_START)
            / (data::CIE_Y_INTEGRAL * N_SAMPLES as f32);

        x *= scale;
        y *= scale;
        z *= scale;

        (x, y, z)
    }

    pub fn y(&self) -> f32 {
        let mut y = 0.0;

        for i in 0..N_SAMPLES {
            y += unsafe { CIE_Y.coefficients[i] * self.coefficients[i] };
        }

        y * (SAMPLED_LAMBDA_END - SAMPLED_LAMBDA_START)
            / (data::CIE_Y_INTEGRAL * N_SAMPLES as f32)
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn to_RGB(&self) -> (f32, f32, f32) {
        XYZ_to_RGB(self.to_XYZ())
    }
}

#[allow(non_snake_case)]
fn XYZ_to_RGB((x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
    let r = 3.2404790 * x - 1.537150 * y - 0.498535 * z;
    let g = -0.969256 * x + 1.875991 * y + 0.041556 * z;
    let b = 0.0556480 * x - 0.204043 * y + 1.057311 * z;

    (r, g, b)
}

#[allow(non_snake_case)]
fn RGB_to_XYZ((r, g, b): (f32, f32, f32)) -> (f32, f32, f32) {
    let x = 0.412453 * r + 0.357580 * g + 0.180423 * b;
    let y = 0.212671 * r + 0.715160 * g + 0.072169 * b;
    let z = 0.019334 * r + 0.119193 * g + 0.950227 * b;

    (x, y, z)
}
