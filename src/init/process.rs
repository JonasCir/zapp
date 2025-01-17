use super::actions_yml::*;
use crate::style_print::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str;

fn regex(re_str: &str) -> Regex {
    Regex::new(re_str).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GcpConfig {
    pub project_id: String,
    pub service_name: String,
    pub region: String,
    pub network: String,
}

pub fn process_init_gcp_config() {
    let msg1 = "Please Input Your GCP Project ID:";
    log_input(msg1);
    let mut project_id = String::new();
    io::stdin()
        .read_line(&mut project_id)
        .expect("Failed to read line");
    let project_id: String = project_id
        .trim()
        .parse()
        .expect("Please Input Your GCP Project ID:");

    let msg2 = "Please Input Your GCP Service Name:";
    log_input(msg2);

    let mut service_name = String::new();
    io::stdin()
        .read_line(&mut service_name)
        .expect("Failed to read line");
    let service_name: String = service_name
        .trim()
        .parse()
        .expect("Please input your GCP service_name:");

    let msg3 = "Please Input Your GCP Region:";
    log_input(msg3);
    let mut region = String::new();
    io::stdin()
        .read_line(&mut region)
        .expect("Failed to read line");
    let region: String = region
        .trim()
        .parse()
        .expect("Please Input Your GCP Region:");

    let msg4 = "Please Input Your GCP Network:";
    log_input(msg4);
    let mut network = String::new();
    io::stdin()
        .read_line(&mut network)
        .expect("Failed to read line");
    let network: String = network
        .trim()
        .parse()
        .expect("Please input your GCP Network:");

    let json_struct = build_gcp_config(project_id, service_name, region, network);
    let result = write_gcp_config(json_struct);
    match result {
        Ok(..) => {
            log_success("Successfully Generated!");
            log_create_file("File Path: ./gcp_config.json");
        }
        Err(err) => {
            log_error(&format!("Failed to Write: {}", err));
        }
    }
}

fn write_gcp_config(json_struct: GcpConfig) -> std::io::Result<()> {
    let serialized: String = serde_json::to_string_pretty(&json_struct).unwrap();
    let mut file = File::create("gcp_config.json")?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn build_gcp_config(
    project_id: String,
    service_name: String,
    region: String,
    network: String,
) -> GcpConfig {
    GcpConfig {
        project_id,
        service_name,
        region,
        network,
    }
}

pub fn build_api_workflow(gcr_region: &str) {
    let workflow_dir = ".github/workflows";
    fs::create_dir_all(workflow_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let workflow_yml = ".github/workflows/zapp_service.yml";
    let file_exist = Path::new(workflow_yml).exists();
    match file_exist {
        true => {
            log_error("Error: File already exist!");
        }
        false => {
            let mut file = fs::File::create(workflow_yml).unwrap();
            file.write_all(action_yml(gcr_region).as_bytes()).unwrap();
            log_success("Successfully created workflow!");
        }
    }
}

pub fn dl_zapp(app_name: &str) {
    let version_range = "v0.7";
    let zapp_dl_url = format!(
        "https://storage.googleapis.com/zapp-bucket/zapp-api-template/{}/zapp-api.tar.gz",
        version_range
    );
    let output = Command::new("curl").args(&["-OL", &zapp_dl_url]).output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let rt = regex("Received");
            match rt.is_match(err.unwrap()) {
                true => {
                    let _ = fs::create_dir(app_name);
                    unzip_zapp(app_name);
                }
                false => {
                    panic!("{:?}", err.unwrap())
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub fn unzip_zapp(app_name: &str) {
    let filename = "zapp-api.tar.gz";
    let output = Command::new("tar").args(&["-zxvf", &filename]).output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let rt = regex("could not");
            match rt.is_match(err.unwrap()) {
                true => {
                    panic!("{:?}", err.unwrap())
                }
                false => {
                    let _ = fs::rename("zapp-api", app_name);
                    let _ = fs::remove_file(&filename);
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub fn git_init(app_name: &str) {
    let output = Command::new("cd")
        .args(&[&app_name, "&&", "git", "init", "--initial-branch=main"])
        .output();

    match &output {
        Ok(_val) => {
            // println!("{:?}", val);
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub fn underscore(s: &str) -> String {
    s.replace("-", "_")
}

pub fn create_dockerfile(app_name: &str) {
    let filename = format!("{}/Dockerfile", app_name);
    let underscore_app_name = underscore(app_name);
    let file_content = format!(
        "FROM rust:1.61 as build
RUN USER=root cargo new --bin {}
WORKDIR /{}
COPY entity entity
COPY migration migration
COPY Cargo.toml Cargo.toml
RUN cargo build --release
COPY ./src ./src
COPY entity/src entity/src
COPY migration/src migration/src
RUN rm -f target/release/deps/{}*
RUN cargo build --release

FROM debian:11.3
COPY --from=build /{}/target/release/{} .

CMD [\"./{}\"]",
        app_name, app_name, &underscore_app_name, app_name, app_name, app_name
    );
    let mut file = fs::File::create(&filename).unwrap();
    file.write_all(file_content.as_bytes()).unwrap();
}

pub fn create_env(app_name: &str) {
    let filename = format!("{}/.env", app_name);
    let file_content = format!(
        "DATABASE_URL1=postgres://postgres:postgres@localhost:5432/{}_db
    PORT=3000
    SECRET=xxxxxxxxxxxxxx
    ZAPP_ENV=development",
        app_name
    );
    let mut file = fs::File::create(&filename).unwrap();
    file.write_all(file_content.as_bytes()).unwrap();
}

pub fn endroll(app_name: &str) {
    let text1 = "  ███████╗ █████╗ ██████╗ ██████╗ ";
    let text2 = "  ╚══███╔╝██╔══██╗██╔══██╗██╔══██╗";
    let text3 = "    ███╔╝ ███████║██████╔╝██████╔╝";
    let text4 = "   ███╔╝  ██╔══██║██╔═══╝ ██╔═══╝ ";
    let text5 = "  ███████╗██║  ██║██║     ██║     ";
    let text6 = "  ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝     ";
    log_white(text1);
    log_white(text2);
    log_white(text3);
    log_white(text4);
    log_white(text5);
    log_white(text6);
    log_new(&format!("\nRust Serverless Framework\n$ cd {}\n$ zapp docker psql\n$ cargo run\n\nGo to : http://localhost:3000/api/graphql", app_name));
}
