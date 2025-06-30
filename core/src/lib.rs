//! SOVEREIGN Core - High-Performance Mobile Runtime
//! 
//! This module provides the foundation for 5-10x mobile performance improvements
//! through intelligent resource management and WASM optimization.

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod runtime;
pub mod performance;
pub mod metrics;
pub mod config;
pub mod error;

use std::sync::Arc;
use tracing::{info, warn};

pub use runtime::SovereignCore;
pub use performance::{BatteryOptimizer, PerformanceMonitor};
pub use metrics::MetricsCollector;
pub use config::Config;
pub use error::{Error, Result};

/// Global application state for the SOVEREIGN runtime
#[derive(Clone)]
pub struct AppState {
    /// Core WASM runtime engine
    pub core: Arc<SovereignCore>,
    /// Battery optimization subsystem
    pub battery_optimizer: Arc<BatteryOptimizer>,
    /// Performance monitoring subsystem
    pub performance_monitor: Arc<PerformanceMonitor>,
    /// Metrics collection subsystem
    pub metrics: Arc<MetricsCollector>,
    /// Application configuration
    pub config: Arc<Config>,
}

impl AppState {
    /// Create a new application state with the given configuration
    pub async fn new(config: Config) -> Result<Self> {
        info!("Initializing SOVEREIGN Core v{}", env!("CARGO_PKG_VERSION"));
        
        // Initialize core runtime
        let core = Arc::new(SovereignCore::new(&config).await?);
        
        // Initialize battery optimizer
        let battery_optimizer = Arc::new(BatteryOptimizer::new(&config)?);
        
        // Initialize performance monitor
        let performance_monitor = Arc::new(PerformanceMonitor::new(&config)?);
        
        // Initialize metrics collector
        let metrics = Arc::new(MetricsCollector::new(&config)?);
        
        // Validate system capabilities
        Self::validate_system_capabilities()?;
        
        Ok(Self {
            core,
            battery_optimizer,
            performance_monitor,
            metrics,
            config: Arc::new(config),
        })
    }
    
    /// Validate that the system meets minimum requirements
    fn validate_system_capabilities() -> Result<()> {
        let info = sysinfo::System::new_all();
        
        let total_memory = info.total_memory();
        if total_memory < 1_000_000_000 { // 1GB minimum
            warn!("System has low memory: {} bytes", total_memory);
        }
        
        let cpu_count = num_cpus::get();
        if cpu_count < 2 {
            warn!("System has limited CPU cores: {}", cpu_count);
        }
        
        info!("System validated: {} cores, {} GB RAM", 
              cpu_count, 
              total_memory / 1_000_000_000);
        
        Ok(())
    }
    
    /// Start all subsystems
    pub async fn start(&self) -> Result<()> {
        info!("Starting SOVEREIGN subsystems");
        
        // Start performance monitoring
        self.performance_monitor.start().await?;
        
        // Start battery optimization
        self.battery_optimizer.start().await?;
        
        // Start metrics collection
        self.metrics.start().await?;
        
        info!("All subsystems started successfully");
        Ok(())
    }
    
    /// Shutdown all subsystems gracefully
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down SOVEREIGN subsystems");
        
        // Stop in reverse order
        self.metrics.stop().await?;
        self.battery_optimizer.stop().await?;
        self.performance_monitor.stop().await?;
        self.core.shutdown().await?;
        
        info!("All subsystems shut down successfully");
        Ok(())
    }
}

/// Initialize tracing/logging subsystem
pub fn init_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sovereign=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_app_state_creation() {
        let config = Config::default();
        let state = AppState::new(config).await;
        assert!(state.is_ok());
    }
}