use std::env;
use std::fs::File as stdFile;
use std::io::BufReader;

use actix_files as fs;
use actix_web::{App, HttpServer, middleware, web};
use config::{Config, File as cfgFile};
use glob::glob;
use rustls::{NoClientAuth, ServerConfig};
use rustls::internal::pemfile::{certs, rsa_private_keys};

#[macro_use]
mod error;
mod app;

#[allow(non_snake_case)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut settings: Config = Config::default();
    settings.merge(glob("config/*")
        .unwrap()
        .map(|path| cfgFile::from(path.unwrap()))
        .collect::<Vec<_>>())
        .unwrap();
    env::set_var("RUST_LOG", settings.get_str(&"RustLog").unwrap());
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let BindAddress: String = settings.get_str("IpAddress").unwrap() + ":" + settings.get_str(&"HttpPort").unwrap().as_str();
    let BindAddressOpenSsl: String = settings.get_str("IpAddress").unwrap() + ":" + settings.get_str(&"HttpsPort").unwrap().as_str();
    let Worker: usize = settings.get_str("Worker").unwrap().parse().unwrap();

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(stdFile::open(settings.get_str(&"SslCertificate").unwrap()).unwrap());
    let key_file = &mut BufReader::new(stdFile::open(settings.get_str(&"SslPrivateKey").unwrap()).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(web::scope("/api")
                .configure(app::apis::routes)
            )
            .configure(app::webs::routes)
            .service(fs::Files::new("/", "public").disable_content_disposition())
    })
        .workers(Worker)
        .bind(BindAddress)?
        .bind_rustls(BindAddressOpenSsl, config)?
        .run()
        .await
}