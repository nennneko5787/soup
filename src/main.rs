use std::fs::File;
use std::fs::create_dir;
use std::io::Write;

use chrono::Utc;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod hs;
mod hspupd;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(required = true)]
        name: String,
    },
    HSP {
        #[command(subcommand)]
        command: Option<HSPCommands>,
    },
}

#[derive(Subcommand)]
enum HSPCommands {
    List,
    Info {
        #[arg(required = true)]
        id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct HSPProject {
    soup: SoupProject,
}

#[derive(Debug, Serialize, Deserialize)]
struct SoupProject {
    name: String,
    version: String,
    description: String,
    base_system: String,
    dependencies: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        // プロジェクト作成コマンド
        Some(Commands::New { name }) => {
            let timestamp = Utc::now().timestamp();
            match create_dir(name) {
                Ok(_) => {
                    // プロジェクト構造体作成
                    let project = HSPProject {
                        soup: SoupProject {
                            name: name.to_string(),
                            version: "0.1.0".to_string(),
                            description: "ここに説明を追加".to_string(),
                            base_system: "hsp36,hsp37".to_string(),
                            dependencies: Vec::new(),
                        },
                    };

                    // 書き込み
                    let mut file = File::create(format!("{}/hspproject.toml", name))?;
                    let toml = toml::to_string(&project).unwrap();
                    write!(file, "{}", toml)?;
                    file.flush()?;

                    println!(
                        "\"{}\" プロジェクトを作成しました。 [{}s]\n\nプロジェクトを実行するには、以下のコマンドを使用してください。\ncd {}\nsoup run",
                        name,
                        (Utc::now().timestamp() - timestamp),
                        name
                    )
                }
                Err(_) => eprintln!("プロジェクトを作成できませんでした。"),
            }
        }
        Some(Commands::HSP { command }) => match (command) {
            Some(HSPCommands::List) => {
                let platform = hspupd::get_hsp_official_platform().await?;
                for item in platform.items {
                    if !item.group.starts_with("hsp") || !item.id.ends_with("_base") {
                        continue;
                    }

                    println!(
                        "{}: {} by {} (公開日 {})",
                        item.id, item.name, item.author, item.last
                    );
                }

                println!("詳細を確認するには、 soup hsp info <id> コマンドを使用します。");

                println!(
                    "これらをインストールするには、 soup hsp install <id> コマンドを使用します。"
                );
            }
            Some(HSPCommands::Info { id }) => {
                let mut matched = false;
                let platform = hspupd::get_hsp_official_platform().await?;
                for item in platform.items {
                    if !item.group.starts_with("hsp") || !item.id.ends_with("_base") {
                        continue;
                    }

                    if item.id == id.to_string() {
                        matched = true;

                        println!(
                            "{}: {} by {} (公開日 {})",
                            item.id, item.name, item.author, item.last
                        );
                        println!("{}", item.inst);
                        if !item.href.is_empty() {
                            println!("依存関係: {}", item.href);
                        }
                        break;
                    }
                }

                if !matched {
                    eprintln!("id \"{}\" のHSPパッケージが見つかりませんでした。", id)
                }
            }
            None => {}
        },
        None => {}
    }
    Ok(())
}
