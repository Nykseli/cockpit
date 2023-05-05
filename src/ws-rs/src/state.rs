use actix_web::web;
use std::{collections::HashMap, fs, path::Path};

use crate::constants::STATIC_BASE_PATH;

fn gen_os_release() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut path = Path::new("/etc/os-release");
    if !path.exists() {
        path = Path::new("/usr/lib/os-release");
        if !path.exists() {
            eprintln!("Error: Cannot load contents of os-release");
            return map;
        }
    }

    let content = fs::read_to_string(path).expect("Unexpected error reading os_release");
    for line in content.lines() {
        if line.starts_with('#') {
            continue;
        }

        let mut key_val = line.split('=');
        let key = key_val.next().unwrap();
        let value = key_val.next().unwrap().replace('"', "");
        map.insert(key.into(), value);
    }

    map
}

fn calculate_branding_roots(os_release: &HashMap<String, String>) -> Vec<String> {
    let mut roots: Vec<String> = Vec::new();

    // TODO: add_system_dirs from src/ws/cockpitbranding.c

    // TODO: properly follow the original paths like this
    /*   if let Some(os_id) = os_release.get("ID") {
        if let Some(os_variant_id) = os_release.get("VARIANT_ID") {
            roots.push(format!("{STATIC_BASE_PATH}cockpit/branding/{os_id}-{os_variant_id}"));
        }
        roots.push(format!("{STATIC_BASE_PATH}cockpit/branding/{os_id}"));
    }

    if let Some(os_id_like) = os_release.get("ID_LIKE") {
        for word in os_id_like.split(" ") {
            roots.push(format!("{STATIC_BASE_PATH}cockpit/branding/{word}"));
        }
    }

    roots.push(format!("{STATIC_BASE_PATH}cockpit/branding/default"));
    roots.push(format!("{STATIC_BASE_PATH}cockpit/static")); */
    // ln -s `pwd`/src/branding/ dist/static/branding

    // This is for local testing
    if let Some(os_id) = os_release.get("ID") {
        if let Some(os_variant_id) = os_release.get("VARIANT_ID") {
            roots.push(format!(
                "{STATIC_BASE_PATH}branding/{os_id}-{os_variant_id}"
            ));
        }
        roots.push(format!("{STATIC_BASE_PATH}branding/{os_id}"));
    }

    if let Some(os_id_like) = os_release.get("ID_LIKE") {
        for word in os_id_like.split(' ') {
            roots.push(format!("{STATIC_BASE_PATH}branding/{word}"));
        }
    }

    roots.push(format!("{STATIC_BASE_PATH}branding/default"));
    roots.push("{STATIC_BASE_PATH}".to_string());
    roots
}

// CockpitHandlerData from src/ws/cockpithandlers.h
#[derive(Debug)]
pub struct CockpitState {
    // TODO: Auth
    os_release: HashMap<String, String>,
    branding: Vec<String>,
}

impl CockpitState {
    pub fn new() -> Self {
        let os_release = gen_os_release();
        let branding = calculate_branding_roots(&os_release);
        Self {
            os_release,
            branding,
        }
    }

    pub fn branding(&self) -> &Vec<String> {
        &self.branding
    }

    pub fn os_release(&self) -> &HashMap<String, String> {
        &self.os_release
    }

    pub fn build_js_environment(&self) -> String {
        let _release_fields = [
            "NAME",
            "ID",
            "PRETTY_NAME",
            "VARIANT",
            "VARIANT_ID",
            "CPE_NAME",
            "ID_LIKE",
            "DOCUMENTATION_URL",
        ];

        let prefix = "\n    <script>\nvar environment = ";
        let suffix = ";\n    </script>";

        // TODO: actually build the environment src/ws/cockpithandlers.c:build_environment
        let environment = "{\"is_cockpit_client\":false,\"page\":{\"connect\":true,\"require_host\":false},\"hostname\":\"pond\",\"os-release\":{\"NAME\":\"openSUSE Tumbleweed\",\"ID\":\"opensuse-tumbleweed\",\"PRETTY_NAME\":\"openSUSE Tumbleweed\",\"CPE_NAME\":\"cpe:/o:opensuse:tumbleweed:20230426\",\"ID_LIKE\":\"opensuse suse\",\"DOCUMENTATION_URL\":\"https://en.opensuse.org/Portal:Tumbleweed\"}}";
        format!("{prefix}{environment}{suffix}")
    }
}

pub type WebCockpitState = web::Data<CockpitState>;
