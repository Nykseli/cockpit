use actix_web::web;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{
    collections::HashMap,
    convert::From,
    env::var,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

// TODO: proper package types
#[derive(Debug, Deserialize, Serialize)]
struct BridgeInitStatePackages {
    playground: Option<String>,
    ssh: Option<String>,
    performance: Option<String>,
    selinux: Option<String>,
    shell: Option<String>,
    kdump: Option<String>,
    network: Option<String>,
    users: Option<String>,
    metrics: Option<String>,
    apps: Option<String>,
    updates: Option<String>,
    storage: Option<String>,
    // TODO: serde key static
    #[serde(rename = "static")]
    static_: Option<String>,
    base1: Option<String>,
    sosreport: Option<String>,
    system: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BridgeInitState {
    command: String,
    version: i32,
    packages: BridgeInitStatePackages,
    #[serde(rename = "os-release")]
    os_release: HashMap<String, Option<String>>,
    capabilities: HashMap<String, bool>,
}

use crate::{cockpit_bridge::CockpitBridge, constants::STATIC_BASE_PATH};

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

fn calculate_branding_roots(os_release: &HashMap<String, String>) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    let base = PathBuf::from(STATIC_BASE_PATH);

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
            roots.push(base.join(format!("branding/{os_id}-{os_variant_id}")));
        }
        roots.push(base.join(format!("branding/{os_id}")));
    }

    if let Some(os_id_like) = os_release.get("ID_LIKE") {
        for word in os_id_like.split(' ') {
            roots.push(base.join(format!("branding/{word}")));
        }
    }

    roots.push(base.join("branding/default"));
    roots.push(base);
    roots
}

// CockpitHandlerData from src/ws/cockpithandlers.h
#[derive(Debug)]
pub struct CockpitState {
    // TODO: Auth
    os_release: HashMap<String, String>,
    branding: Vec<PathBuf>,
    #[allow(dead_code)]
    bridge_state: BridgeInitState,
    bridge: Arc<Mutex<CockpitBridge>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Page {
    connect: bool,
    require_host: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Enviroment {
    is_cockpit_client: bool,
    page: Page,
    hostname: String,
    os_release: HashMap<String, String>,
}

impl From<&CockpitState> for Enviroment {
    fn from(state: &CockpitState) -> Enviroment {
        Enviroment {
            //FIXME don't use constants find out how its detected upstream
            is_cockpit_client: false,
            page: Page {
                connect: true,
                require_host: false,
            },
            //FIXME
            hostname: var("HOST").unwrap(),
            os_release: state.os_release().to_owned(),
        }
    }
}

impl CockpitState {
    pub fn new(bridge: Arc<Mutex<CockpitBridge>>, bridge_msg: &str) -> Self {
        let bridge_state: BridgeInitState =
            serde_json::from_str(bridge_msg).expect("First message from bridge is not valid");
        let os_release = gen_os_release();
        let branding = calculate_branding_roots(&os_release);
        Self {
            os_release,
            branding,
            bridge_state,
            bridge,
        }
    }

    pub fn branding(&self) -> &Vec<PathBuf> {
        &self.branding
    }

    pub fn os_release(&self) -> &HashMap<String, String> {
        &self.os_release
    }

    pub fn bridge(&self) -> &Arc<Mutex<CockpitBridge>> {
        &self.bridge
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
        let environment = Enviroment::from(self);
        //TODO: drop unwrap
        let envstring = to_string(&environment).unwrap();
        format!("{prefix}{envstring}{suffix}")
    }
}

pub type WebCockpitState = web::Data<CockpitState>;
