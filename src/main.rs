use clap::Parser;
use std::process;

use kaspa_audio_transfer::audio::{read_audio_file, save_audio_file};
use kaspa_audio_transfer::cli;
use kaspa_audio_transfer::cli::Cli;
use kaspa_audio_transfer::error::{AudioTransferError, Result};
use kaspa_audio_transfer::{
    connected_wallet_network, receive_bytes, send_bytes, wallet_address, wallet_balance_kas,
    wallet_send_kas,
};

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
        }

        cli::Commands::Receive {
            tx_id,
            output,
            rpc_url,
            start_block_hash,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            receive_audio(&tx_id, &output, rpc_url, start_block_hash.as_deref()).await
        }

        cli::Commands::Estimate {
            input_file,
            from_private_key,
            rpc_url,
            amount,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            estimate_audio(&input_file, &from_private_key, rpc_url, amount).await
        }

        cli::Commands::WalletProfiles => {
            let profiles = kaspa_audio_transfer::wallet_vault::list_profiles()?;
            if profiles.is_empty() {
                println!("No wallet profiles found.");
            } else {
                println!("Wallet profiles:");
                for p in profiles {
                    println!("- {p}");
                }
            }
            Ok(())
        }

        cli::Commands::WalletCreateMnemonic {
            username,
            password,
            word_count,
            mnemonic_password,
        } => {
            let phrase = kaspa_audio_transfer::wallet_vault::create_profile_mnemonic(
                &username,
                &password,
                word_count,
                mnemonic_password.as_deref(),
            )?;
            println!("Created profile: {}", username);
            println!("IMPORTANT: Save this mnemonic securely:");
            println!("{}", phrase);
            Ok(())
        }

        cli::Commands::WalletImportMnemonic {
            username,
            password,
            phrase,
            mnemonic_password,
        } => {
            kaspa_audio_transfer::wallet_vault::import_profile_mnemonic(
                &username,
                &password,
                &phrase,
                mnemonic_password.as_deref(),
            )?;
            println!("Imported profile: {}", username);
            Ok(())
        }

        cli::Commands::WalletImportPrivateKey {
            username,
            password,
            private_key,
        } => {
            kaspa_audio_transfer::wallet_vault::import_profile_private_key(
                &username,
                &password,
                &private_key,
            )?;
            println!("Imported private-key profile: {}", username);
            Ok(())
        }

        cli::Commands::WalletAddress {
            from_private_key,
            profile_username,
            profile_password,
            derivation_path,
            rpc_url,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            show_wallet_address(
                from_private_key.as_deref(),
                profile_username.as_deref(),
                profile_password.as_deref(),
                &derivation_path,
                rpc_url,
            )
            .await
        }

        cli::Commands::WalletBalance {
            from_private_key,
            profile_username,
            profile_password,
            derivation_path,
            rpc_url,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            show_wallet_balance(
                from_private_key.as_deref(),
                profile_username.as_deref(),
                profile_password.as_deref(),
                &derivation_path,
                rpc_url,
            )
            .await
        }

        cli::Commands::WalletSendKas {
            from_private_key,
            profile_username,
            profile_password,
            derivation_path,
            rpc_url,
            to_address,
            amount,
            feerate,
            fee_multiplier,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            wallet_send(
                from_private_key.as_deref(),
                profile_username.as_deref(),
                profile_password.as_deref(),
                &derivation_path,
                rpc_url,
                &to_address,
                amount,
                feerate,
                fee_multiplier,
            )
            .await
        }

        cli::Commands::TxAcceptingBlockHash {
            tx_id,
            rpc_url,
            start_block_hash,
            min_confirmations,
            wait_secs,
        } => {
            let rpc_url = Some(rpc_url.as_str());
            let out = kaspa_audio_transfer::resolve_tx_accepting_block_hash(
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

#[allow(clippy::too_many_arguments)]
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
    println!("Reading file: {}", input_file);
    let audio_data = read_audio_file(input_file)?;

    println!("Connecting to Kaspa node...");
    let network_info = kaspa_audio_transfer::get_network_info(rpc_url).await?;
    println!("Connected to network: {}", network_info);

    println!("\nSending file data to {}...", to_address);
    let tx_id = send_bytes(
        &audio_data,
        from_private_key,
        rpc_url,
        resume_from,
        resume_output_index,
        feerate,
        fee_multiplier,
        to_address,
        amount,
    )
    .await?;

    println!("\n✅ File sent successfully!");
    println!("Transaction ID: {}", tx_id);

    if print_start_block_hash {
        let accepting = kaspa_audio_transfer::resolve_tx_accepting_block_hash(
            &tx_id,
            rpc_url,
            None,
            start_block_hash_min_confirmations,
            start_block_hash_timeout_secs,
        )
        .await?;

        if let Some(h) = accepting {
            println!("Start block hash (scan anchor): {}", h);
        } else {
            println!("Start block hash (scan anchor): <not found>");
        }
    }

    println!("\nThe recipient can retrieve the file with the following command:");
    println!("kaspa_data_transfer receive {} --output output.bin", tx_id);

    Ok(())
}

async fn estimate_audio(
    input_file: &str,
    from_private_key: &str,
    rpc_url: Option<&str>,
    amount: f64,
) -> Result<()> {
    println!("Reading file: {}", input_file);
    let audio_data = read_audio_file(input_file)?;

    println!("Connecting to Kaspa node...");
    let network_info = kaspa_audio_transfer::get_network_info(rpc_url).await?;
    println!("Connected to network: {}", network_info);

    println!(
        "\nEstimating fees for {} bytes of data...",
        audio_data.len()
    );
    let kaspa = kaspa_audio_transfer::kaspa::KaspaClient::new(rpc_url).await?;
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
    let network_info = kaspa_audio_transfer::get_network_info(rpc_url).await?;
    println!("Connected to network: {}", network_info);

    println!("\nRetrieving file data from transaction: {}", tx_id);
    let audio_data = receive_bytes(tx_id, rpc_url, start_block_hash).await?;

    println!("Saving file to: {}", output_path);
    save_audio_file(&audio_data, output_path)?;

    println!("\n✅ File received and saved successfully!");
    println!("You can now open the file at: {}", output_path);

    Ok(())
}

async fn resolve_wallet_private_key(
    from_private_key: Option<&str>,
    profile_username: Option<&str>,
    profile_password: Option<&str>,
    derivation_path: &str,
    rpc_url: Option<&str>,
) -> Result<String> {
    if let Some(k) = from_private_key {
        return Ok(k.to_string());
    }
    let username = profile_username.ok_or_else(|| {
        AudioTransferError::InvalidInput(
            "Provide --from-private-key or (--profile-username and --profile-password)".to_string(),
        )
    })?;
    let password = profile_password.ok_or_else(|| {
        AudioTransferError::InvalidInput(
            "Provide --from-private-key or (--profile-username and --profile-password)".to_string(),
        )
    })?;

    let _ = connected_wallet_network(rpc_url).await?;
    kaspa_audio_transfer::wallet_vault::derive_private_key_hex_for_profile(
        username,
        password,
        derivation_path,
    )
}

async fn show_wallet_address(
    from_private_key: Option<&str>,
    profile_username: Option<&str>,
    profile_password: Option<&str>,
    derivation_path: &str,
    rpc_url: Option<&str>,
) -> Result<()> {
    println!("Connecting to Kaspa node...");
    let network_info = kaspa_audio_transfer::get_network_info(rpc_url).await?;
    println!("Connected to network: {}", network_info);

    let from_private_key = resolve_wallet_private_key(
        from_private_key,
        profile_username,
        profile_password,
        derivation_path,
        rpc_url,
    )
    .await?;
    let address = wallet_address(&from_private_key, rpc_url).await?;
    println!("\nAddress: {}", address);
    Ok(())
}

async fn show_wallet_balance(
    from_private_key: Option<&str>,
    profile_username: Option<&str>,
    profile_password: Option<&str>,
    derivation_path: &str,
    rpc_url: Option<&str>,
) -> Result<()> {
    println!("Connecting to Kaspa node...");
    let network_info = kaspa_audio_transfer::get_network_info(rpc_url).await?;
    println!("Connected to network: {}", network_info);

    let from_private_key = resolve_wallet_private_key(
        from_private_key,
        profile_username,
        profile_password,
        derivation_path,
        rpc_url,
    )
    .await?;
    let address = wallet_address(&from_private_key, rpc_url).await?;
    let balance = wallet_balance_kas(&from_private_key, rpc_url).await?;
    println!("\nAddress: {}", address);
    println!("\nBalance: {:.8} KAS", balance);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn wallet_send(
    from_private_key: Option<&str>,
    profile_username: Option<&str>,
    profile_password: Option<&str>,
    derivation_path: &str,
    rpc_url: Option<&str>,
    to_address: &str,
    amount: f64,
    feerate: Option<f64>,
    fee_multiplier: Option<f64>,
) -> Result<()> {
    println!("Connecting to Kaspa node...");
    let network_info = kaspa_audio_transfer::get_network_info(rpc_url).await?;
    println!("Connected to network: {}", network_info);

    let from_private_key = resolve_wallet_private_key(
        from_private_key,
        profile_username,
        profile_password,
        derivation_path,
        rpc_url,
    )
    .await?;
    println!("\nSending {:.8} KAS to {}...", amount, to_address);
    let tx_id = wallet_send_kas(
        &from_private_key,
        rpc_url,
        to_address,
        amount,
        feerate,
        fee_multiplier,
    )
    .await?;
    println!("\n✅ KAS sent successfully!");
    println!("Transaction ID: {}", tx_id);
    Ok(())
}
