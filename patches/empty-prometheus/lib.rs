// patches/empty-prometheus/lib.rs
pub mod prometheus {
    pub mod core {
        pub struct Collector;
    }

    pub struct HistogramTimer;

    pub struct CounterVec;
    pub struct GaugeVec;
    pub struct HistogramOpts;
    pub struct HistogramVec;
    pub struct Opts;
    pub struct PrometheusError;
    pub struct Registry;
    pub struct U64;

    pub fn exponential_buckets(_: f64, _: f64, _: usize) -> Vec<f64> {
        vec![]
    }
}