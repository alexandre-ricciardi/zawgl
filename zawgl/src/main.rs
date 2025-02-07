// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate zawgl_server;
extern crate tokio;
extern crate serde;
mod settings;
use log::*;
use zawgl_core::model::init::{DatabaseInitContext, InitContext};
use settings::Settings;
use simple_logger::SimpleLogger;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    database: Option<DatabaseCommands>,
}

#[derive(Subcommand)]
enum DatabaseCommands {
    Create {
        #[arg(short, long)]
        name: String,
    },
}

async fn run_database(ctx: DatabaseInitContext, address: &str) {
    let (tx_run, rx_run) = zawgl_server::keep_commit_loop(500);
    let exit = tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        if let Ok(_) = tx_run.send(false).await {
            info!("Exiting commit loop");
        }
    });
    tokio::select! {
        _ = zawgl_server::run_server(address, ctx, || {
            info!("Database started");
        }, rx_run) => 0,
        _ = exit => 0
    };
    info!("Database stopped");
}


async fn run_all(ctx: InitContext, address: &str) {
    for db_ctx in ctx.dbs_ctx {
        run_database(db_ctx, address).await;
    }
}


#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() {
    let mut settings = Settings::new();
    let log_level = settings.get_log_level();
    SimpleLogger::new().with_level(log_level).init().unwrap();
    let ctx: InitContext = InitContext::new(&settings.server.database_root_dir, settings.get_db_dirs());

    let cli = Cli::parse();
    match &cli.database {
        Some(DatabaseCommands::Create { name }) => {
            settings.server.databases_dirs.push(name.to_string());
        }
        None => {
            run_all(ctx, &settings.server.address).await;
        }
    }
}
