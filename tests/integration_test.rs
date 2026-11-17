//! Integration tests for SigmaOS-gnome-system-monitor.

use sigmaos_gnome_system_monitor_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
