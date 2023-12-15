use super::Artifact;
use std::{io::prelude::BufRead, io::BufReader};

#[derive(Debug, serde::Serialize)]
pub struct ExtractResult {
    pub artifact: Artifact,
    pub matches: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct UrlListExtractResult {
    pub url: String,
    pub results: Vec<ExtractResult>,
}

fn extract_artifact_from_text(text: &str, artifact: Artifact) -> Vec<String> {
    let mut matches = <Artifact as Into<&regex::Regex>>::into(artifact)
        .find_iter(&text)
        .map(|r_match| r_match.as_str().to_string())
        .collect::<Vec<String>>();
    matches.sort();
    matches.dedup();

    matches
}

pub async fn extract_artifacts_from_url(
    url: &str,
    artifacts: Vec<Artifact>,
    req_client: &impl crate::WebFetch,
) -> Result<Vec<ExtractResult>, crate::error::Error> {
    let string_response: String = req_client.fetch(url).await?;
    let document = select::document::Document::from(string_response.as_str());
    let selectors =
        select::predicate::Or(
            select::predicate::Or(
                select::predicate::Name("p"),
                select::predicate::Name("a")
            ),
            select::predicate::Or(
                select::predicate::Name("li"),
                select::predicate::Name("ol")
            )
        );

    let scraped_html = document
        .find(selectors)
        .into_iter()
        .map(|elem| elem.text())
        .filter(|elem_text| !elem_text.is_empty())
        .collect::<Vec<String>>()
        .join("\n");

    Ok(artifacts
        .into_iter()
        .map(|artifact| ExtractResult {
            artifact,
            matches: extract_artifact_from_text(&scraped_html, artifact),
        })
        .collect::<Vec<ExtractResult>>())
}

pub async fn extract_artifacts_from_url_list_file(
    file_name: &str,
    artifacts: Vec<Artifact>,
    req_client: &impl crate::WebFetch,
) -> Result<Vec<UrlListExtractResult>, crate::error::Error> {
    let file_path = std::path::Path::new(file_name);

    if !file_path.exists() {
        return Err(crate::error::Error::NotFound(format!(
            "File {} not found.",
            file_name
        )));
    }

    let urls = BufReader::new(std::fs::File::open(file_path)?)
        .lines()
        .into_iter()
        .filter_map(|res| res.ok())
        .filter(|line| !line.is_empty());

    let mut artifacts_extracted_from_file: Vec<UrlListExtractResult> = Vec::new();

    for url in urls {
        let extracted_artifacts = extract_artifacts_from_url(url.as_str(), artifacts.clone(), req_client)
            .await?
            .into_iter()
            .filter(|extract_result| !extract_result.matches.is_empty())
            .collect::<Vec<ExtractResult>>();

        artifacts_extracted_from_file.push(UrlListExtractResult {
            url,
            results: extracted_artifacts,
        });
    }

    Ok(artifacts_extracted_from_file)
}

#[cfg(test)]
mod tests {

    use std::iter::zip;

    use super::*;
    const TEST_URL: &str = "http://local.host";

    struct TestCase<'a> {
        test_name: &'a str,
        regex_enum: Artifact,
        expected: usize,
    }

    #[tokio::test]
    async fn test_extract_hashes_from_url() {
        let fake_client = crate::fakers::FakeHttpReqwest::default().set_success_response(
            include_str!("./test_static_files/iceid_proofpoint.html").to_string(),
        );

        let test_cases = vec![
            TestCase {
                test_name: "extract md5 from url test",
                regex_enum: Artifact::MD5,
                expected: 0,
            },
            TestCase {
                test_name: "extract sha1 from url test",
                regex_enum: Artifact::SHA1,
                expected: 0,
            },
            TestCase {
                test_name: "extract sha256 from url test",
                regex_enum: Artifact::SHA256,
                expected: 6,
            },
            TestCase {
                test_name: "extract sha512 from url test",
                regex_enum: Artifact::SHA512,
                expected: 0,
            },
        ];

        for test_case in test_cases {
            if let Some(result) =
                extract_artifacts_from_url(TEST_URL, vec![test_case.regex_enum], &fake_client)
                    .await
                    .unwrap()
                    .first()
            {
                assert_eq!(
                    result.matches.len(),
                    test_case.expected,
                    "{0} error. Got: {1}. Expected: {2}",
                    test_case.test_name,
                    result.matches.len(),
                    test_case.expected
                );
            } else {
                assert!(false, "{0}. Got nothing", test_case.test_name);
            }
        }
    }

    #[tokio::test]
    async fn test_extract_network_artifacts_from_url() {
        let fake_client = crate::fakers::FakeHttpReqwest::default().set_success_response(
            include_str!("./test_static_files/iceid_proofpoint.html").to_string(),
        );

        let test_cases = vec![
            TestCase {
                test_name: "extract urls from url test",
                regex_enum: Artifact::URL,
                expected: 7,
            },
            TestCase {
                test_name: "extract ipv6 from url test",
                regex_enum: Artifact::IPv6,
                expected: 0,
            },
            TestCase {
                test_name: "extract ipv4 from url test",
                regex_enum: Artifact::IPv4,
                expected: 4,
            },
            TestCase {
                test_name: "extract domains from url test",
                regex_enum: Artifact::DOMAIN,
                expected: 20,
            },
            TestCase {
                test_name: "extract emails from url test",
                regex_enum: Artifact::EMAIL,
                expected: 0,
            },
        ];

        for test_case in test_cases {
            if let Some(result) =
                extract_artifacts_from_url(TEST_URL, vec![test_case.regex_enum], &fake_client)
                    .await
                    .unwrap()
                    .first()
            {
                assert_eq!(
                    result.matches.len(),
                    test_case.expected,
                    "{0} error. Got: {1}. Expected: {2}",
                    test_case.test_name,
                    result.matches.len(),
                    test_case.expected
                );
            } else {
                assert!(false, "{0}. Got nothing", test_case.test_name);
            }
        }
    }

    #[tokio::test]
    async fn test_extract_filenames_from_url() {
        let fake_client = crate::fakers::FakeHttpReqwest::default().set_success_response(
            include_str!("./test_static_files/iceid_proofpoint.html").to_string(),
        );

        let test_cases = vec![
            TestCase {
                test_name: "extract web filenames from url test",
                regex_enum: Artifact::WEB,
                expected: 3,
            },
            TestCase {
                test_name: "extract zip filenames from url test",
                regex_enum: Artifact::ZIP,
                expected: 0,
            },
            TestCase {
                test_name: "extract doc filenames from url test",
                regex_enum: Artifact::DOC,
                expected: 0,
            },
            TestCase {
                test_name: "extract windows filenames from url test",
                regex_enum: Artifact::WIN,
                expected: 10,
            },
            TestCase {
                test_name: "extract mac filenames from url test",
                regex_enum: Artifact::MAC,
                expected: 0,
            },
        ];

        for test_case in test_cases {
            if let Some(result) =
                extract_artifacts_from_url(TEST_URL, vec![test_case.regex_enum], &fake_client)
                    .await
                    .unwrap()
                    .first()
            {
                assert_eq!(
                    result.matches.len(),
                    test_case.expected,
                    "{0} error. Got: {1}. Expected: {2}",
                    test_case.test_name,
                    result.matches.len(),
                    test_case.expected
                );
            } else {
                assert!(false, "{0}. Got nothing", test_case.test_name);
            }
        }
    }

    #[tokio::test]
    async fn test_extract_artifacts_from_url() {
        let fake_client = crate::fakers::FakeHttpReqwest::default().set_success_response(
            include_str!("./test_static_files/iceid_proofpoint.html").to_string(),
        );

        let test_cases = vec![
            TestCase {
                test_name: "extract web filenames from url test",
                regex_enum: Artifact::WEB,
                expected: 3,
            },
            TestCase {
                test_name: "extract doc filenames from url test",
                regex_enum: Artifact::DOC,
                expected: 0,
            },
            TestCase {
                test_name: "extract windows filenames from url test",
                regex_enum: Artifact::WIN,
                expected: 10,
            },
            TestCase {
                test_name: "extract urls from url test",
                regex_enum: Artifact::URL,
                expected: 7,
            },
            TestCase {
                test_name: "extract ipv4 from url test",
                regex_enum: Artifact::IPv4,
                expected: 4,
            },
            TestCase {
                test_name: "extract domains from url test",
                regex_enum: Artifact::DOMAIN,
                expected: 20,
            },
            TestCase {
                test_name: "extract sha256 from url test",
                regex_enum: Artifact::SHA256,
                expected: 6,
            },
        ];

        let matches_result = extract_artifacts_from_url(
            TEST_URL,
            test_cases
                .iter()
                .map(|test_case| test_case.regex_enum)
                .collect::<Vec<Artifact>>(),
            &fake_client,
        )
        .await
        .unwrap();

        for (test_case, match_result) in zip(test_cases, matches_result) {
            assert_eq!(
                match_result.matches.len(),
                test_case.expected,
                "{0} error. Got: {1}. Expected: {2}",
                test_case.test_name,
                match_result.matches.len(),
                test_case.expected
            );
        }
    }

    #[tokio::test]
    async fn test_extract_artifacts_from_file_with_list_of_url() {
        let fake_client = crate::fakers::FakeHttpReqwest::default().set_success_response(
            include_str!("./test_static_files/iceid_proofpoint.html").to_string(),
        );

        let response = extract_artifacts_from_url_list_file(
            "./src/commands/test_static_files/test_file",
            vec![Artifact::SHA256],
            &fake_client,
        )
        .await
        .unwrap();

        assert!(!response.is_empty(), "Empty response");
        assert!(!response[0].results.is_empty(), "No results");
        let matches = response[0].results[0].matches.len();
        let expected = 6;

        assert_eq!(
            matches, expected,
            "expected {} sha256 matches. Got: {}",
            expected, matches
        )
    }
}
