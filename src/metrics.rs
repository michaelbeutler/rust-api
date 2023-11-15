use prometheus::{
    core::{AtomicF64, GenericCounter},
    opts, Counter, Encoder, HistogramVec, Opts, Registry, TextEncoder,
};
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    http::Method,
    time::Instant,
    Build, Data, Request, Response, Rocket, State,
};

/// Value stored in request-local state to measure response time.
#[derive(Copy, Clone)]
struct TimerStart(Option<Instant>);

#[derive(Clone, Debug)]
pub struct PrometheusMetrics {
    http_requests_total: GenericCounter<AtomicF64>,
    http_get_requests_total: GenericCounter<AtomicF64>,
    http_post_requests_total: GenericCounter<AtomicF64>,
    http_put_requests_total: GenericCounter<AtomicF64>,
    http_delete_requests_total: GenericCounter<AtomicF64>,
    http_patch_requests_total: GenericCounter<AtomicF64>,
    http_head_requests_total: GenericCounter<AtomicF64>,
    http_options_requests_total: GenericCounter<AtomicF64>,
    http_2xx_responses_total: GenericCounter<AtomicF64>,
    http_3xx_responses_total: GenericCounter<AtomicF64>,
    http_4xx_responses_total: GenericCounter<AtomicF64>,
    http_401_responses_total: GenericCounter<AtomicF64>,
    http_403_responses_total: GenericCounter<AtomicF64>,
    http_5xx_responses_total: GenericCounter<AtomicF64>,
    http_requests_duration_seconds: HistogramVec,
}

macro_rules! counter {
    ($name:tt,$help:tt) => {
        Counter::with_opts(Opts::new($name, $help)).unwrap()
    };
}

impl Default for PrometheusMetrics {
    fn default() -> Self {
        Self {
            http_requests_total: counter!(
                "http_requests_total",
                "Total number of HTTP requests made."
            ),
            http_get_requests_total: counter!(
                "http_get_requests_total",
                "Total number of HTTP GET requests made."
            ),
            http_post_requests_total: counter!(
                "http_post_requests_total",
                "Total number of HTTP POST requests made."
            ),
            http_put_requests_total: counter!(
                "http_put_requests_total",
                "Total number of HTTP PUT requests made."
            ),
            http_delete_requests_total: counter!(
                "http_delete_requests_total",
                "Total number of HTTP DELETE requests made."
            ),
            http_patch_requests_total: counter!(
                "http_patch_requests_total",
                "Total number of HTTP PATCH requests made."
            ),
            http_head_requests_total: counter!(
                "http_head_requests_total",
                "Total number of HTTP HEAD requests made."
            ),
            http_options_requests_total: counter!(
                "http_options_requests_total",
                "Total number of HTTP OPTIONS requests made."
            ),
            http_2xx_responses_total: counter!(
                "http_2xx_responses_total",
                "Total number of HTTP 2xx responses made."
            ),
            http_3xx_responses_total: counter!(
                "http_3xx_responses_total",
                "Total number of HTTP 3xx responses made."
            ),
            http_4xx_responses_total: counter!(
                "http_4xx_responses_total",
                "Total number of HTTP 4xx responses made."
            ),
            http_401_responses_total: counter!(
                "http_401_responses_total",
                "Total number of HTTP 401 responses made."
            ),
            http_403_responses_total: counter!(
                "http_403_responses_total",
                "Total number of HTTP 403 responses made."
            ),
            http_5xx_responses_total: counter!(
                "http_5xx_responses_total",
                "Total number of HTTP 5xx responses made."
            ),
            http_requests_duration_seconds: HistogramVec::new(
                opts!(
                    "http_requests_duration_seconds",
                    "HTTP request duration in seconds for all requests"
                )
                .into(),
                &["endpoint", "method", "status"],
            )
            .unwrap(),
        }
    }
}

#[rocket::async_trait]
impl Fairing for PrometheusMetrics {
    fn info(&self) -> Info {
        Info {
            name: "Prometheus Metrics",
            kind: Kind::Ignite | Kind::Request | Kind::Response,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        #[get("/metrics")]
        fn metrics(metrics: &State<PrometheusMetrics>) -> String {
            // Create a Registry and register Counter.
            let r = Registry::new();

            r.register(Box::new(metrics.http_requests_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_get_requests_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_post_requests_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_put_requests_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_delete_requests_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_patch_requests_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_head_requests_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_options_requests_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_2xx_responses_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_3xx_responses_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_4xx_responses_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_401_responses_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_403_responses_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_5xx_responses_total.clone()))
                .unwrap();
            r.register(Box::new(metrics.http_requests_duration_seconds.clone()))
                .unwrap();

            // Gather the metrics.
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = r.gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();

            // Output to the standard output.
            format!("{}", String::from_utf8(buffer).unwrap())
        }

        Ok(rocket.manage(self.clone()).mount("/", routes![metrics]))
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        request.local_cache(|| TimerStart(Some(Instant::now())));
        self.http_requests_total.inc();

        match request.method() {
            Method::Get => self.http_get_requests_total.inc(),
            Method::Post => self.http_post_requests_total.inc(),
            Method::Put => self.http_put_requests_total.inc(),
            Method::Delete => self.http_delete_requests_total.inc(),
            Method::Patch => self.http_patch_requests_total.inc(),
            Method::Head => self.http_head_requests_total.inc(),
            Method::Options => self.http_options_requests_total.inc(),
            _ => {}
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let endpoint = request.route().unwrap().uri.as_str();
        let method = request.method().as_str();
        let status = response.status().code.to_string();

        let start_time = request.local_cache(|| TimerStart(None));
        if let Some(duration) = start_time.0.map(|st| st.elapsed()) {
            let duration_secs = duration.as_seconds_f64();
            self.http_requests_duration_seconds
                .with_label_values(&[endpoint, method, status.as_str()])
                .observe(duration_secs);
        }

        match response.status().code {
            200..=299 => self.http_2xx_responses_total.inc(),
            300..=399 => self.http_3xx_responses_total.inc(),
            400..=499 => {
                self.http_4xx_responses_total.inc();
                match response.status().code {
                    401 => self.http_401_responses_total.inc(),
                    403 => self.http_403_responses_total.inc(),
                    _ => {}
                }
            }
            500..=599 => self.http_5xx_responses_total.inc(),
            _ => {}
        }
    }
}
