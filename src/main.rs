use clap::ArgEnum;
use clap::Args;
use clap::FromArgMatches;
use clap::Parser;
pub use cumulus_primitives_core::ParaId;
use hex_literal;
use sp_core::crypto::Ss58AddressFormat;
use sp_core::crypto::Ss58Codec;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::AccountId32;
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    #[allow(missing_docs)]
    #[clap(name = "get-ss58")]
    GetSs58(GetSs58Command),
    #[allow(missing_docs)]
    #[clap(name = "get-account-id")]
    GetAccountId(GetAccountIdCommand),
}

#[derive(Debug, Parser)]
pub struct GetSs58Command {
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub id: IdParams,
    #[allow(missing_docs)]
    #[clap(long, value_name = "Ss58AddressFormat (u16)")]
    pub dest_format: u16,
}

#[derive(Debug, Parser)]
pub struct GetAccountIdCommand {
    #[allow(missing_docs)]
    #[clap(long, value_name = "Ss58Address")]
    pub ss58_address: String,
}

#[derive(Debug, Clone, Args)]
pub struct IdParams {
    #[allow(missing_docs)]
    #[clap(long, value_name = "Parachain Id (u32)")]
    para_id: Option<u32>,
    #[allow(missing_docs)]
    #[clap(long, value_name = "Account ID (hex)")]
    account_id: Option<AccountId32>,
    #[allow(missing_docs)]
    #[clap(long, value_name = "Ss58Address")]
    ss58_address: Option<String>,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

fn main() {
    let cmd = Cli::parse();
    match cmd.subcommand {
        None => {
            println!("nothing");
        }
        Some(x) => match x {
            Subcommand::GetAccountId(x) => {
                let (account_id, format) =
                    AccountId32::from_string_with_version(x.ss58_address.as_str()).unwrap();
                println!("account id: {:?}, Ss58 format:{:?}", account_id, format);
            }
            Subcommand::GetSs58(x) => {
                let dest_format = x.dest_format;

                if x.id.para_id.is_some()
                    && x.id.account_id.is_none()
                    && x.id.ss58_address.is_none()
                {
                    let account_id: AccountId32 =
                        ParaId::from(x.id.para_id.unwrap()).into_account();
                    let ss58 = account_id
                        .to_ss58check_with_version(Ss58AddressFormat::custom(dest_format));
                    println!("ss58 address: {:?}", ss58);
                    return;
                }

                if x.id.account_id.is_some()
                    && x.id.para_id.is_none()
                    && x.id.ss58_address.is_none()
                {
                    let ss58 =
                        x.id.account_id
                            .unwrap()
                            .to_ss58check_with_version(Ss58AddressFormat::custom(dest_format));
                    println!("ss58 address: {:?}", ss58);
                    return;
                }

                if x.id.ss58_address.is_some()
                    && x.id.para_id.is_none()
                    && x.id.account_id.is_none()
                {
                    let (account_id, format) =
                        AccountId32::from_string_with_version(x.id.ss58_address.unwrap().as_str())
                            .unwrap();
                    let ss58 = account_id
                        .to_ss58check_with_version(Ss58AddressFormat::custom(dest_format));
                    println!("ss58 address: {:?}", ss58);
                    return;
                }

                println!("Format error, please check your command line arguments!");
            }
        },
    };
}
