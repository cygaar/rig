use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GenerationConfig},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the Google Gemini client
    // Create OpenAI client
    let client = gemini::Client::from_env();

    // Create agent with a single context prompt
    let agent = client
        .agent(gemini::completion::GEMINI_1_5_PRO)
        .preamble("Be precise and concise.")
        .temperature(0.5)
        .max_tokens(8192)
        .additional_params(
            serde_json::to_value(GenerationConfig {
                top_k: Some(1),
                top_p: Some(0.95),
                candidate_count: Some(1),
                ..Default::default()
            })
            .unwrap(),
        ) // Unwrap the Result to get the Value
        .build();

    // Prompt the agent and print the response
    let response = agent
        .prompt("How much wood would a woodchuck chuck if a woodchuck could chuck wood?")
        .await?;
    println!("{}", response);

    Ok(())
}
