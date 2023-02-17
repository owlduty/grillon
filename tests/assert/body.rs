use crate::HttpMockServer;
use grillon::{
    dsl::{contains, is, is_not, json_path},
    header::{HeaderValue, CONTENT_TYPE},
    json, Grillon, Result,
};

#[tokio::test]
async fn json_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .json_body(is(json!({
            "id": 1,
            "name": "Isaac",
        })));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn raw_string_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .json_body(is(r#"
        {
            "id": 1,
            "name": "Isaac"
        }
        "#));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn string_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];
    let json = r#"
    {
        "id": 1,
        "name": "Isaac"
    }
    "#
    .to_string();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .json_body(is(json));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn it_should_not_be_equals() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_body(is_not(json!({
            "id": 101,
            "name": "Ecbert",
        })));
}

#[tokio::test]
#[should_panic]
async fn it_should_fail_to_compare_bad_body() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_body(is(json!({
            "id": 100,
            "name": "Tom",
        })));
}

#[tokio::test]
#[should_panic]
async fn it_should_fail_to_compare_inexistant_body() {
    let mock_server = HttpMockServer::new();
    mock_server.get_empty_response();

    Grillon::new(mock_server.server.url("/").as_ref())
        .unwrap()
        .get("empty")
        .assert()
        .await
        .json_body(is(json!({
            "id": 1,
            "name": "Isaac",
        })));
}

#[tokio::test]
async fn json_path_should_be_equals() {
    let var_name = HttpMockServer::new();
    let mock_server = var_name;
    mock_server.get_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_body(json_path(
            "$".to_string(),
            is(json!([{
                "id": 1,
                "name": "Isaac",
            }])),
        ));
}

#[tokio::test]
async fn json_path_should_not_be_equal() {
    let var_name = HttpMockServer::new();
    let mock_server = var_name;
    mock_server.get_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_body(json_path(
            "$".to_string(),
            is_not(json!([{
                "id": 2,
                "name": "Max",
            }])),
        ));
}
