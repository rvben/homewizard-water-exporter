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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::homewizard::HomeWizardWaterData;

    fn create_test_data() -> HomeWizardWaterData {
        HomeWizardWaterData {
            wifi_ssid: "TestNetwork".to_string(),
            wifi_strength: 75.5,
            total_liter_m3: 1234.567,
            active_liter_lpm: 15.5,
            total_liter_offset_m3: 100.0,
        }
    }

    #[test]
    fn test_metrics_creation() {
        let metrics = Metrics::new();
        assert!(metrics.is_ok());
    }

    #[test]
    fn test_metrics_update() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        let result = metrics.update(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_metrics_gather() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let result = metrics.gather();
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("homewizard_water_total_m3"));
        assert!(output.contains("homewizard_water_active_flow_lpm"));
        assert!(output.contains("homewizard_water_offset_m3"));
        assert!(output.contains("homewizard_water_wifi_strength_percent"));
        assert!(output.contains("homewizard_water_meter_info"));
    }

    #[test]
    fn test_metrics_water_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_total_m3 1234.567"));
        assert!(output.contains("homewizard_water_active_flow_lpm 15.5"));
        assert!(output.contains("homewizard_water_offset_m3 100"));
    }

    #[test]
    fn test_metrics_network_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_wifi_strength_percent 75.5"));
    }

    #[test]
    fn test_metrics_meter_info_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_meter_info{wifi_ssid=\"TestNetwork\"} 1"));
    }

    #[test]
    fn test_metrics_with_zero_values() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.total_liter_m3 = 0.0;
        data.active_liter_lpm = 0.0;
        data.total_liter_offset_m3 = 0.0;
        data.wifi_strength = 0.0;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_total_m3 0"));
        assert!(output.contains("homewizard_water_active_flow_lpm 0"));
        assert!(output.contains("homewizard_water_offset_m3 0"));
        assert!(output.contains("homewizard_water_wifi_strength_percent 0"));
    }

    #[test]
    fn test_metrics_update_multiple_times() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();

        // First update
        metrics.update(&data).unwrap();
        let output1 = metrics.gather().unwrap();
        assert!(output1.contains("homewizard_water_active_flow_lpm 15.5"));

        // Second update with different values
        data.active_liter_lpm = 25.0;
        metrics.update(&data).unwrap();
        let output2 = metrics.gather().unwrap();
        assert!(output2.contains("homewizard_water_active_flow_lpm 25"));
    }

    #[test]
    fn test_metrics_large_values() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.total_liter_m3 = 999999.999;
        data.active_liter_lpm = 999.0;
        data.total_liter_offset_m3 = 500.0;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_total_m3 999999.999"));
        assert!(output.contains("homewizard_water_active_flow_lpm 999"));
        assert!(output.contains("homewizard_water_offset_m3 500"));
    }

    #[test]
    fn test_metrics_with_different_wifi_network() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.wifi_ssid = "DifferentNetwork".to_string();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_meter_info{wifi_ssid=\"DifferentNetwork\"} 1"));
    }

    #[test]
    fn test_metrics_with_high_flow_rate() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.active_liter_lpm = 1000.0;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_active_flow_lpm 1000"));
    }

    #[test]
    fn test_metrics_with_negative_offset() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.total_liter_offset_m3 = -50.0;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_offset_m3 -50"));
    }

    #[test]
    fn test_metrics_with_weak_wifi() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.wifi_strength = 10.0;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_wifi_strength_percent 10"));
    }

    #[test]
    fn test_metrics_with_decimal_values() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.total_liter_m3 = 123.456;
        data.active_liter_lpm = 7.89;
        data.total_liter_offset_m3 = 12.34;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_water_total_m3 123.456"));
        assert!(output.contains("homewizard_water_active_flow_lpm 7.89"));
        assert!(output.contains("homewizard_water_offset_m3 12.34"));
    }
}
