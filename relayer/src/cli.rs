// Copyright (C) Polytope Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tesseract CLI utilities

use crate::{config::Config, logging};
use clap::Parser;

/// Tesseract, the multi-chain ISMP relayer
#[derive(Parser, Debug)]
pub struct Cli {
    /// Path to the relayer config file
    #[arg(short, long)]
    config: String,
}

impl Cli {
    /// Run the relayer
    pub async fn run(self) -> Result<(), anyhow::Error> {
        logging::setup();

        let config = tokio::fs::read_to_string(&self.config).await?;

        let tesseract_config = toml::from_str::<Config>(&config)?;

        let chain_a = tesseract_config.chain_a.into_client().await?;
        let chain_b = tesseract_config.chain_b.into_client().await?;

        messaging::relay(chain_a, chain_b).await?;

        Ok(())
    }
}
