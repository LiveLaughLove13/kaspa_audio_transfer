mod audio;
mod cli;
mod error;
mod kaspa;

use clap::Parser;
use std::process;

use crate::audio::{read_audio_file, save_audio_file};
use crate::cli::Cli;
use crate::error::Result;
use crate::kaspa::KaspaClient;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Send { 
            input_file, 
            from_private_key,
            rpc_url, 
            resume_from,
            resume_output_index,
            feerate,
            fee_multiplier,
            to_address, 
            amount,
            print_start_block_hash,
            start_block_hash_min_confirmations,
            start_block_hash_timeout_secs,
        } => {
            // Convert Option<String> to Option<&str>
            let rpc_url = Some(rpc_url.as_str());
            send_audio(
                &input_file,
                &from_private_key,
                rpc_url,
                resume_from.as_deref(),
                resume_output_index,
                feerate,
                fee_multiplier,
                &to_address,
                amount,
                print_start_block_hash,
                start_block_hash_min_confirmations,
                start_block_hash_timeout_secs,
            )
            .await
        },
        
        cli::Commands::Receive { 
            tx_id, 
            output, 
            rpc_url,
            start_block_hash,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            receive_audio(&tx_id, &output, rpc_url, start_block_hash.as_deref()).await
        },
        
        cli::Commands::Estimate {
            input_file,
            from_private_key,
            rpc_url,
            amount,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            estimate_audio(&input_file, &from_private_key, rpc_url, amount).await
        },

        cli::Commands::TxAcceptingBlockHash {
            tx_id,
            rpc_url,
            start_block_hash,
            min_confirmations,
            wait_secs,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            let out = resolve_tx_accepting_block_hash(
                &tx_id,
                rpc_url,
                start_block_hash.as_deref(),
                min_confirmations,
                wait_secs,
            )
            .await?;
            if let Some(h) = out {
                println!("{h}");
            }
            Ok(())
        }
    }
}

async fn send_audio(
    input_file: &str, 
    from_private_key: &str,
    rpc_url: Option<&str>,
    resume_from: Option<&str>,
    resume_output_index: u32,
    feerate: Option<f64>,
    fee_multiplier: Option<f64>,
    to_address: &str, 
    amount: f64,
    print_start_block_hash: bool,
    start_block_hash_min_confirmations: u64,
    start_block_hash_timeout_secs: u64,
) -> Result<()> {
    println!("Reading audio file: {}", input_file);
    let audio_data = read_audio_file(input_file)?;
    
    println!("Connecting to Kaspa node...");
    let kaspa = KaspaClient::new(rpc_url).await?;
    
    let network_info = kaspa.get_network_info().await?;
    println!("Connected to network: {}", network_info);
    
    println!("\nSending audio data to {}...", to_address);
    let tx_id = kaspa
        .send_audio_signed(
            &audio_data,
            from_private_key,
            to_address,
            amount,
            resume_from,
            resume_output_index,
            feerate,
            fee_multiplier,
        )
        .await?;
    
    println!("\n✅ File sent successfully!");
    println!("Transaction ID: {}", tx_id);

    if print_start_block_hash {
        let accepting = resolve_tx_accepting_block_hash(
            &tx_id,
            rpc_url,
            None,
            start_block_hash_min_confirmations,
            start_block_hash_timeout_secs,
        )
        .await?;

        if let Some(h) = accepting {
            println!("Start block hash: {}", h);
        } else {
            println!("Start block hash: <not found>");
        }
    }

    println!("\nThe recipient can retrieve the audio file with the following command:");
    println!("kaspa_audio_transfer receive {} --output received_audio.mp3", tx_id);
    
    Ok(())
}

async fn resolve_tx_accepting_block_hash(
    tx_id: &str,
    rpc_url: Option<&str>,
    start_block_hash: Option<&str>,
    min_confirmations: u64,
    wait_secs: u64,
) -> Result<Option<String>> {
    let kaspa = KaspaClient::new(rpc_url).await?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(wait_secs);

    loop {
        let found = kaspa
            .get_tx_accepting_block_hash(tx_id, start_block_hash, min_confirmations)
            .await?;
        if found.is_some() {
            return Ok(found);
        }

        if wait_secs == 0 {
            return Ok(None);
        }
        if std::time::Instant::now() >= deadline {
            return Ok(None);
        }
        tokio::time::sleep(std::time::Duration::from_millis(750)).await;
    }
}

async fn estimate_audio(
    input_file: &str,
    from_private_key: &str,
    rpc_url: Option<&str>,
    amount: f64,
) -> Result<()> {
    println!("Reading audio file: {}", input_file);
    let audio_data = read_audio_file(input_file)?;
    
    println!("Connecting to Kaspa node...");
    let kaspa = KaspaClient::new(rpc_url).await?;
    
    let network_info = kaspa.get_network_info().await?;
    println!("Connected to network: {}", network_info);
    
    println!("\nEstimating fees for {} bytes of data...", audio_data.len());
    kaspa
        .estimate_audio_fees(&audio_data, from_private_key, amount)
        .await?;
    
    Ok(())
}

async fn receive_audio(
    tx_id: &str, 
    output_path: &str, 
    rpc_url: Option<&str>,
    start_block_hash: Option<&str>,
) -> Result<()> {
    println!("Connecting to Kaspa node...");
    let kaspa = KaspaClient::new(rpc_url).await?;
    
    let network_info = kaspa.get_network_info().await?;
    println!("Connected to network: {}", network_info);
    
    println!("\nRetrieving audio data from transaction: {}", tx_id);
    let audio_data = kaspa.receive_audio(tx_id, start_block_hash).await?;
    
    println!("Saving audio file to: {}", output_path);
    save_audio_file(&audio_data, output_path)?;
    
    println!("\n✅ Audio file received and saved successfully!");
    println!("You can now play the audio file at: {}", output_path);
    
    Ok(())
}