use crate::homewizard::HomeWizardWaterData;
use anyhow::Result;
use prometheus::{Counter, Encoder, Gauge, GaugeVec, Opts, Registry, TextEncoder};

pub struct Metrics {
    // Water consumption metrics
    total_water: Counter,
    active_flow: Gauge,
    water_offset: Gauge,

    // Network metrics
    wifi_strength: Gauge,

    // Info metric
    meter_info: GaugeVec,

    registry: Registry,
}

impl Metrics {
    pub fn new() -> Result<Self> {
        let registry = Registry::new();

        // Water consumption metrics
        let total_water = Counter::with_opts(Opts::new(
            "homewizard_water_total_m3",
            "Total water consumption in m³",
        ))?;
        registry.register(Box::new(total_water.clone()))?;

        let active_flow = Gauge::with_opts(Opts::new(
            "homewizard_water_active_flow_lpm",
            "Current water flow in liters per minute",
        ))?;
        registry.register(Box::new(active_flow.clone()))?;

        let water_offset = Gauge::with_opts(Opts::new(
            "homewizard_water_offset_m3",
            "Water meter offset in m³",
        ))?;
        registry.register(Box::new(water_offset.clone()))?;

        // Network metrics
        let wifi_strength = Gauge::with_opts(Opts::new(
            "homewizard_water_wifi_strength_percent",
            "WiFi signal strength percentage",
        ))?;
        registry.register(Box::new(wifi_strength.clone()))?;

        // Info metric
        let meter_info = GaugeVec::new(
            Opts::new("homewizard_water_meter_info", "Water meter information"),
            &["wifi_ssid"],
        )?;
        registry.register(Box::new(meter_info.clone()))?;

        Ok(Self {
            total_water,
            active_flow,
            water_offset,
            wifi_strength,
            meter_info,
            registry,
        })
    }

    pub fn update(&self, data: &HomeWizardWaterData) -> Result<()> {
        // Update water metrics
        self.total_water.reset();
        self.total_water.inc_by(data.total_liter_m3);

        self.active_flow.set(data.active_liter_lpm);
        self.water_offset.set(data.total_liter_offset_m3);

        // Update network metrics
        self.wifi_strength.set(data.wifi_strength);

        // Update info metric
        self.meter_info.reset();
        self.meter_info
            .with_label_values(&[&data.wifi_ssid])
            .set(1.0);

        Ok(())
    }

    pub fn gather(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}
