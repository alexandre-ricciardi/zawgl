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
use log::info;
use zawgl_core::model::init::InitContext;
use settings::Settings;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("config can't be loaded");
    let log_level = settings.get_log_level();
    SimpleLogger::new().with_level(log_level).init().unwrap();
    let ctx = InitContext::new(&settings.server.database_dir).expect("can't create database context");
    tokio::select! {
        _ = zawgl_server::run_server(&settings.server.address, ctx, || {
            info!("database started");
        }) => 0,
        _ = tokio::signal::ctrl_c() => 0
    };
}
