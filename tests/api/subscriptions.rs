use crate::helpers::spawn_app;

#[tokio::test]
async fn subcribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    let body = "name=rick%20de%20boer&email=rick.deboer%40live.nl";
    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "rick.deboer@live.nl");
    assert_eq!(saved.name, "rick de boer");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=rick%20de%boer", "missing email"),
        ("email=rick.deboer%40live.nl", "missing name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=rick.deboer%40live.nl", "empty name"),
        ("name=Rick&email=", "empty email"),
        ("name=Rick&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}
