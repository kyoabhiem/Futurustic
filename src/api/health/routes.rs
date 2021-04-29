use crate::api::Respond;
use crate::errors::ApiError;
use actix_web::{
    get,
    web::{scope, Json, ServiceConfig},
};
use sys_info::*;

extern crate sys_info;

#[get("")]
pub async fn get_health() -> Result<Json<Respond<HealthResponse, ()>>, ApiError> {
    let load = loadavg().unwrap();
    let mem = mem_info().unwrap();
    let disk = disk_info().unwrap();

    Ok(Json(Respond {
        data: HealthResponse {
            status: "ok".into(),
            hostname: hostname().unwrap(),
            app_version: env!("CARGO_PKG_VERSION").into(),
            os: format!("{} {}", os_type().unwrap(), os_release().unwrap()),
            cpu: Cpu {
                core: cpu_num().unwrap(),
                speed: cpu_speed().unwrap(),
            },
            cpu_total: proc_total().unwrap(),
            load: Load {
                one: load.one,
                five: load.five,
                fifteen: load.fifteen,
            },
            memory: Memory {
                total: mem.total,
                free: mem.free,
                avail: mem.avail,
                buffers: mem.buffers,
                cached: mem.cached,
                swap: Swap {
                    total: mem.swap_total,
                    free: mem.swap_free,
                },
            },
            disk: Disk {
                total: disk.total,
                free: disk.free,
            },
        },
        meta: None,
    }))
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub hostname: String,
    pub status: String,
    pub app_version: String,
    pub os: String,
    pub cpu: Cpu,
    pub cpu_total: u64,
    pub load: Load,
    pub memory: Memory,
    pub disk: Disk,
}

#[derive(Serialize)]
pub struct Cpu {
    pub core: u32,
    pub speed: u64,
}

#[derive(Serialize)]
pub struct Load {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(Serialize)]
pub struct Memory {
    pub total: u64,
    pub free: u64,
    pub avail: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap: Swap,
}

#[derive(Serialize)]
pub struct Swap {
    pub total: u64,
    pub free: u64,
}

#[derive(Serialize)]
pub struct Disk {
    pub total: u64,
    pub free: u64,
}


pub fn init_rotes(cfg: &mut ServiceConfig) {
    cfg.service(scope("health").service(get_health));
}
