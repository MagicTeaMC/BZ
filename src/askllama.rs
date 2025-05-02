pub async fn ask(question: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use async_openai::{
        Client,
        config::OpenAIConfig,
        types::{CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, ChatCompletionRequestAssistantMessageArgs},
    };

    // Configure the client with the GROQ API URL
    let config = OpenAIConfig::new()
        .with_api_base("https://api.groq.com/openai/v1")
        .with_api_key(std::env::var("GROQ_API_KEY").expect("Missing GROQ_API_KEY environment variable"));

    // Create a client with the custom configuration
    let client = Client::with_config(config);

    // Create the chat completion request
    let request = CreateChatCompletionRequestArgs::default()
        .model("llama-3.3-70b-versatile")
        .messages(vec![
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Who won the world series in 2020?")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("The Los Angeles Dodgers won the World Series in 2020.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(question)
                .build()?
                .into(),
        ])
        .build()
        .unwrap();

    // Execute the request and get the response
    let response = client.chat().create(request).await?;

    // Extract the answer from the response
    let answer = response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_else(|| String::from("No answer provided"));

    Ok(answer)
}
