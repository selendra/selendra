// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use sp_keyring::AccountKeyring;
use std::{
    ffi::{OsStr, OsString},
    io::{BufRead, BufReader, Read},
    process,
};
use subxt::{Config, OnlineClient};

/// Spawn a local substrate node for testing subxt.
pub struct TestNodeProcess<R: Config> {
    proc: process::Child,
    client: OnlineClient<R>,
}

impl<R> Drop for TestNodeProcess<R>
where
    R: Config,
{
    fn drop(&mut self) {
        let _ = self.kill();
    }
}

impl<R> TestNodeProcess<R>
where
    R: Config,
{
    /// Construct a builder for spawning a test node process.
    pub fn build<S>(program: S) -> TestNodeProcessBuilder
    where
        S: AsRef<OsStr> + Clone,
    {
        TestNodeProcessBuilder::new(program)
    }

    /// Attempt to kill the running substrate process.
    pub fn kill(&mut self) -> Result<(), String> {
        tracing::info!("Killing node process {}", self.proc.id());
        if let Err(err) = self.proc.kill() {
            let err = format!("Error killing node process {}: {}", self.proc.id(), err);
            tracing::error!("{}", err);
            return Err(err);
        }
        Ok(())
    }

    /// Returns the subxt client connected to the running node.
    pub fn client(&self) -> OnlineClient<R> {
        self.client.clone()
    }
}

/// Construct a test node process.
pub struct TestNodeProcessBuilder {
    node_path: OsString,
    authority: Option<AccountKeyring>,
}

impl TestNodeProcessBuilder {
    pub fn new<P>(node_path: P) -> TestNodeProcessBuilder
    where
        P: AsRef<OsStr>,
    {
        Self {
            node_path: node_path.as_ref().into(),
            authority: None,
        }
    }

    /// Set the authority dev account for a node in validator mode e.g. --alice.
    pub fn with_authority(&mut self, account: AccountKeyring) -> &mut Self {
        self.authority = Some(account);
        self
    }

    /// Spawn the substrate node at the given path, and wait for rpc to be initialized.
    pub async fn spawn<R>(&self) -> Result<TestNodeProcess<R>, String>
    where
        R: Config,
    {
        let mut cmd = process::Command::new(&self.node_path);
        cmd.env("RUST_LOG", "info")
            .arg("--dev")
            .arg("--tmp")
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .arg("--port=0")
            .arg("--rpc-port=0")
            .arg("--ws-port=0");

        if let Some(authority) = self.authority {
            let authority = format!("{authority:?}");
            let arg = format!("--{}", authority.as_str().to_lowercase());
            cmd.arg(arg);
        }

        let mut proc = cmd.spawn().map_err(|e| {
            format!(
                "Error spawning substrate node '{}': {}",
                self.node_path.to_string_lossy(),
                e
            )
        })?;

        // Wait for RPC port to be logged (it's logged to stderr):
        let stderr = proc.stderr.take().unwrap();
        let ws_port = find_substrate_port_from_output(stderr);
        let ws_url = format!("ws://127.0.0.1:{ws_port}");

        // Connect to the node with a subxt client:
        let client = OnlineClient::from_url(ws_url.clone()).await;
        match client {
            Ok(client) => Ok(TestNodeProcess { proc, client }),
            Err(err) => {
                let err = format!("Failed to connect to node rpc at {ws_url}: {err}");
                tracing::error!("{}", err);
                proc.kill().map_err(|e| {
                    format!("Error killing substrate process '{}': {}", proc.id(), e)
                })?;
                Err(err)
            }
        }
    }
}

// Consume a stderr reader from a spawned substrate command and
// locate the port number that is logged out to it.
fn find_substrate_port_from_output(r: impl Read + Send + 'static) -> u16 {
    BufReader::new(r)
        .lines()
        .find_map(|line| {
            let line = line.expect("failed to obtain next line from stdout for port discovery");

            // does the line contain our port (we expect this specific output from substrate).
            let line_end = line
                .rsplit_once("Listening for new connections on 127.0.0.1:")
                .or_else(|| line.rsplit_once("Running JSON-RPC WS server: addr=127.0.0.1:"))
                .map(|(_, port_str)| port_str)?;

            // trim non-numeric chars from the end of the port part of the line.
            let port_str = line_end.trim_end_matches(|b: char| !b.is_ascii_digit());

            // expect to have a number here (the chars after '127.0.0.1:') and parse them into a u16.
            let port_num = port_str
                .parse()
                .unwrap_or_else(|_| panic!("valid port expected for log line, got '{port_str}'"));

            Some(port_num)
        })
        .expect("We should find a port before the reader ends")
}
