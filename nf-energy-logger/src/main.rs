use std::time::Duration;

use nf_energy_monitor_parser::*;

use clap::{Parser, Subcommand};
use futures::prelude::*;
use influxdb2::{
    models::{data_point::FieldValue, DataPoint},
    Client,
};
use tokio::time;
use tracing::{event, span, Level};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    DryRun {
        #[clap(long, env)]
        nf_host: String,
    },

    Run {
        #[clap(long, env)]
        nf_host: String,

        #[command(flatten)]
        influx: InfluxInfo,
    },
}

#[derive(Debug, Parser)]
struct InfluxInfo {
    #[clap(long, env)]
    infl_host: String,

    #[clap(long, env)]
    infl_token: String,

    #[clap(long, env)]
    infl_org: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt().init();

    let (host, influx_client): (String, Option<Client>) = match cli.command {
        Commands::DryRun { nf_host } => (nf_host, None),
        Commands::Run { nf_host, influx } => (
            nf_host,
            Some(Client::new(
                influx.infl_host,
                influx.infl_org,
                influx.infl_token,
            )),
        ),
    };

    let http_client = reqwest::Client::builder()
        .user_agent("nf-energy-monitor-logger/0.0.0")
        .build()
        .unwrap();

    let mut interval = time::interval(Duration::from_millis(300));

    loop {
        interval.tick().await;

        let host = host.clone();
        let http_client = http_client.clone();
        let influx_client = influx_client.clone();

        tokio::spawn(async move {
            let gui = async {
                let html = http_client
                    .get(format!("http://{host}/OutputGetGui.cgi"))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                Battery::from_partial_html(&html).unwrap()
            };

            let val = async {
                let html = http_client
                    .get(format!("http://{host}/OutputGetVal.cgi"))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                ValueMap::from_partial_html(&html)
            };

            let time = std::time::SystemTime::now();


            let unix_nanos = time
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64;

            let (battery, val) = tokio::join!(gui, val);

            tracing::info!("Battery {}%", battery.remaining);

            use itertools::Itertools;
            for (k, val) in val.0.iter().sorted_by_key(|v| v.0) {
                tracing::info!("  {k}: {val:?}");
            }

            if let Some(influx_client) = influx_client {
                let mut data_builder = DataPoint::builder("battery")
                    .tag("host", &host)
                    .timestamp(unix_nanos)
                    .field("remaining", battery.remaining as i64);

                for (k, val) in val.0 {
                    let Ok(val) = val else {
                        continue;
                    };

                    data_builder = data_builder.field(
                        k,
                        match val {
                            Value::Wattage(n) => FieldValue::I64(n as i64),
                            Value::TimeInMinutes(n) => FieldValue::I64(n as i64),
                            Value::Mode(s) => FieldValue::String(s.to_string()),
                            Value::Boolean(b) => FieldValue::Bool(b),
                        },
                    );
                }

                let data = data_builder.build().unwrap();

                influx_client
                    .write("nf-energy-monitor", stream::iter([data]))
                    .await
                    .unwrap();
            };
        });
    }
}
