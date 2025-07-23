use clap::{Parser, arg};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::X509NameBuilder;
use openssl::x509::X509ReqBuilder;
mod error;

pub use self::error::{Error, Result};

#[derive(Parser, Debug)]
#[command(name = "gen")]
struct Args {
    #[arg(long, help="Two letter country code (e.g EN)")]
    pub country: String,
    #[arg(long, help="Organization name")]
    pub org: String,
    #[arg(long, help="CNAME record")]
    pub cn: String,
    #[arg(long, help="Path to output the CSR")]
    pub csr_out: String,
    #[arg(long, help="Path to output the CSR private key")]
    pub key_out: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let rsa = Rsa::generate(2048)?;
    let pkey = PKey::from_rsa(rsa)?;


    let mut name_builder = X509NameBuilder::new()?;
    name_builder.append_entry_by_text("C", &args.country)?;
    name_builder.append_entry_by_text("O", &args.org)?;
    name_builder.append_entry_by_text("CN", &args.cn)?;
    let name = name_builder.build();

    let mut req_builder = X509ReqBuilder::new()?;
    req_builder.set_subject_name(&name)?;
    req_builder.set_pubkey(&pkey)?;
    req_builder.sign(&pkey, MessageDigest::sha512())?;
    let csr = req_builder.build();

    let pem = csr.to_pem()?;
    let pem = String::from_utf8(pem)?;

    let pkey_pem = pkey.private_key_to_pem_pkcs8()?;
    let pkey_pem = String::from_utf8(pkey_pem)?;


    std::fs::write( args.csr_out, pem)?;
    std::fs::write( args.key_out, pkey_pem)?;

    println!("Files saved successfully");

    Ok(())
}
