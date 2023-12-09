use self::extract::{extract_artifacts_from_url, extract_artifacts_from_url_list_file, ExtractResult, };
use crate::{regexes, save_to_json, WebFetch};
use structopt::{clap::arg_enum, StructOpt};

pub mod extract;

arg_enum! {
    #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize)]
    pub enum Artifact {
        MD5,
        SHA1,
        SHA256,
        SHA512,
        URL,
        IPv4,
        IPv6,
        DOMAIN,
        EMAIL,
        DOC,
        WIN,
        MAC,
        ZIP,
        WEB,
    }
}

impl From<Artifact> for &str {
    fn from(value: Artifact) -> Self {
        match value {
            Artifact::MD5 => "md5",
            Artifact::SHA1 => "sha1",
            Artifact::SHA256 => "sha256",
            Artifact::SHA512 => "sha512",
            Artifact::URL => "url",
            Artifact::IPv4 => "ipv4",
            Artifact::IPv6 => "ipv6",
            Artifact::DOMAIN => "domain",
            Artifact::EMAIL => "email",
            Artifact::DOC => "doc",
            Artifact::MAC => "mac",
            Artifact::WEB => "web",
            Artifact::WIN => "win",
            Artifact::ZIP => "zip",
        }
    }
}

impl From<&Artifact> for &str {
    fn from(value: &Artifact) -> Self {
        value.clone().into()
    }
}

impl From<Artifact> for String {
    fn from(value: Artifact) -> Self {
        <Artifact as Into<&str>>::into(value).to_string()
    }
}

impl From<&Artifact> for String {
    fn from(value: &Artifact) -> Self {
        <&Artifact as Into<&str>>::into(value).to_string()
    }
}

impl<'a> Into<&'a regex::Regex> for Artifact {
    fn into(self) -> &'a regex::Regex {
        match self {
            Artifact::MD5 => &regexes::MD5_REGEX,
            Artifact::SHA1 => &regexes::SHA1_REGEX,
            Artifact::SHA256 => &regexes::SHA256_REGEX,
            Artifact::SHA512 => &regexes::SHA512_REGEX,
            Artifact::URL => &regexes::URL_REGEX,
            Artifact::IPv4 => &regexes::IPV4_REGEX,
            Artifact::IPv6 => &regexes::IPV6_REGEX,
            Artifact::DOMAIN => &regexes::DOMAIN_REGEX,
            Artifact::EMAIL => &regexes::EMAIL_REGEX,
            Artifact::DOC => &regexes::DOC_FILE_REGEX,
            Artifact::MAC => &regexes::MAC_FILE_REGEX,
            Artifact::WEB => &regexes::WEB_FILE_REGEX,
            Artifact::WIN => &regexes::WIN_FILE_REGEX,
            Artifact::ZIP => &regexes::ZIP_FILE_REGEX,
        }
    }
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq, Debug, Clone, StructOpt)]
    pub enum ExtractTypeArgument {
        url,
        url_list_file,
    }
}

impl Default for ExtractTypeArgument {
    fn default() -> Self {
        Self::url
    }
}

impl From<ExtractTypeArgument> for &str {
    fn from(value: ExtractTypeArgument) -> Self {
        match value {
            ExtractTypeArgument::url => "url",
            ExtractTypeArgument::url_list_file => "url_list_file",
        }
    }
}

impl From<&ExtractTypeArgument> for &str {
    fn from(value: &ExtractTypeArgument) -> Self {
        value.clone().into()
    }
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    Extract {
        #[
            structopt(
                short = "t",
                long = "type",
                default_value,
                possible_values= &ExtractTypeArgument::variants(),
                case_insensitive = true
            )
        ]
        extract_type: ExtractTypeArgument,

        #[
            structopt(
                short = "a",
                long = "artifacts",
                use_delimiter = true,
                possible_values = &Artifact::variants(),
                case_insensitive = true
            )
        ]
        artifacts: Vec<Artifact>,

        #[structopt(long = "from")]
        extract_from: String,
    },
}

impl SubCommand {
    async fn handle(
        &self,
        req_client: &impl WebFetch,
        output_file: Option<String>,
    ) -> Result<(), crate::error::Error> {
        match self {
            Self::Extract {
                extract_type,
                artifacts,
                extract_from,
            } => match extract_type {
                ExtractTypeArgument::url => {
                    self.handle_extract_from_url(
                        extract_from.as_str(),
                        artifacts.clone(),
                        req_client,
                        output_file,
                    )
                    .await
                }
                ExtractTypeArgument::url_list_file => {
                    self.handle_extract_from_url_list_file(
                        extract_from,
                        artifacts.clone(),
                        req_client,
                        output_file,
                    )
                    .await
                }
            },
        }
    }

    async fn handle_extract_from_url(
        &self,
        url: &str,
        artifacts: Vec<Artifact>,
        req_client: &impl WebFetch,
        output_file: Option<String>,
    ) -> Result<(), crate::error::Error> {
        let extracted_artifacts = extract_artifacts_from_url(&url, artifacts, req_client).await?;

        match output_file {
            Some(filename) => {
                println!("[*] Saving extracted data to {}", filename);
                save_to_json(&filename, extracted_artifacts)?;
            }
            None => self.print_extracted_artifacts_to_stdout(extracted_artifacts),
        };

        Ok(())
    }

    async fn handle_extract_from_url_list_file(
        &self,
        url_filename: &str,
        artifacts: Vec<Artifact>,
        req_client: &impl WebFetch,
        output_file: Option<String>,
    ) -> Result<(), crate::error::Error> {
        let results = extract_artifacts_from_url_list_file(url_filename, artifacts, req_client)
            .await?;

        match output_file {
            Some(filename) => {
                println!("[*] Saving extracted data to {}", filename);
                save_to_json(&filename, results)?;
            }
            None => {
                results
                    .into_iter()
                    .inspect(|result| println!("[+] Results for {}", result.url))
                    .for_each(|result| self.print_extracted_artifacts_to_stdout(result.results));
            }
        };

        Ok(())
    }

    fn print_extracted_artifacts_to_stdout(&self, extracted_artifacts: Vec<ExtractResult>) {
        extracted_artifacts
            .into_iter()
            .filter(|extract_result| !extract_result.matches.is_empty())
            .inspect(|extract_result| {
                println!(
                    "[*] {} results for {}.",
                    extract_result.matches.len(),
                    extract_result.artifact.to_string().to_lowercase()
                )
            })
            .for_each(|extract_result| {
                extract_result
                    .matches
                    .iter()
                    .for_each(|value| println!("\t[-] {}", value))
            });
    }
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long, global = true)]
    output_file: Option<String>,

    #[structopt(subcommand)]
    sub_cmds: SubCommand,
}

impl Args {
    pub async fn handle(self, req_client: impl WebFetch) -> Result<(), crate::error::Error> {
        self.sub_cmds.handle(&req_client, self.output_file).await
    }
}
