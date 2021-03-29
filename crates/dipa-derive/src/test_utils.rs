use syn::__private::TokenStream2;

pub fn assert_tokens_eq(left: &TokenStream2, right: &TokenStream2) {
    assert_eq!(
        token_stream_to_vec(&left),
        token_stream_to_vec(&right),
        r#"
Left Tokens:
{}

Right Tokens:
{}
"#,
        left.to_string(),
        right.to_string()
    )
}

pub fn assert_starts_with_tokens(all_tokens: &TokenStream2, expected_start: &TokenStream2) {
    let all_token_strings = token_stream_to_vec(&all_tokens);
    let expected_start_strings = token_stream_to_vec(&expected_start);

    assert_eq!(
        &all_token_strings[0..expected_start_strings.len()],
        &expected_start_strings,
        r#"
All Tokens:
{}

Expected Start:
{}
"#,
        all_tokens.to_string(),
        expected_start.to_string()
    );
}

fn token_stream_to_vec(tokens: &TokenStream2) -> Vec<String> {
    tokens
        .clone()
        .into_iter()
        .map(|t| t.to_string().trim().to_string())
        .collect()
}
