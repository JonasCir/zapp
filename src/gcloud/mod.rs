pub mod process;

use crate::cli::GcpConfig;
use crate::compute::{
    process_create_connector, process_create_external_ip, process_create_firewall_ssh,
    process_create_firewall_tcp, process_create_nat, process_create_network, process_create_router,
    process_create_subnet,
};
use crate::iam::{
    process_add_roles, process_create_service_account, process_create_service_account_key,
    process_enable_permissions,
};
use crate::init::build_api_workflow;
use crate::sql::{
    process_assign_network, process_connect_vpc_connector, process_create_ip_range,
    process_create_sql,
};
use crate::style_print::log_success;
pub use process::*;
use std::fs::File;
use std::io::BufReader;

pub fn get_gcp() -> GcpConfig {
    let file_name = "gcp_config.json";
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let gcp: GcpConfig = serde_json::from_reader(reader).unwrap();
    gcp
}

pub fn setup_deployment(gcp: GcpConfig) {
    // 1. Create IAM
    process_create_service_account(&gcp.project_id, &gcp.service_name);
    process_create_service_account_key(&gcp.project_id, &gcp.service_name);
    process_add_roles(&gcp.project_id, &gcp.service_name);
    process_enable_permissions(&gcp.project_id);
    let log = "Your IAM is all set!";
    log_success(log);
    // 2. Create NAT
    process_create_network(&gcp.project_id, &gcp.service_name);
    process_create_firewall_tcp(&gcp.project_id, &gcp.service_name);
    process_create_firewall_ssh(&gcp.project_id, &gcp.service_name);
    process_create_subnet(&gcp.project_id, &gcp.service_name, &gcp.region);
    process_create_connector(&gcp.project_id, &gcp.service_name, &gcp.region);
    process_create_router(&gcp.project_id, &gcp.service_name, &gcp.region);
    process_create_external_ip(&gcp.project_id, &gcp.service_name, &gcp.region);
    process_create_nat(&gcp.project_id, &gcp.service_name, &gcp.region);
    // 3. Create Cloud SQL
    process_create_sql(
        &gcp.project_id,
        &gcp.service_name,
        &gcp.region,
        &gcp.network,
    );
    // 4. Create Cloud SQL Private Network
    process_create_ip_range(&gcp.project_id, &gcp.service_name);
    process_connect_vpc_connector(&gcp.project_id, &gcp.service_name);
    process_assign_network(&gcp.project_id, &gcp.service_name);
    // 5. Create Github Actions Workflow
    build_api_workflow(&gcp.gcr_region());
}
