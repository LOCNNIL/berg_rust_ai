// Platform with Open-Source ML models avaible: Hugging Face (https://huggingface.co/)

use rust_bert::{resources::RemoteResource, gpt_neo::{GptNeoModelResources, GptNeoConfigResources, GptNeoVocabResources, GptNeoMergesResources}, pipelines::text_generation::{TextGenerationConfig, TextGenerationModel}};

/// The Model used to generate tests contains 4 files:
/// 1 - .ot file
/// 2 - config file
/// 3 - vocab file
/// 4 - merges

fn main() {

    /// Rust Bert has some built-in values for the model that we are going to use with is: GPT-Neo
    let model_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoModelResources::GPT_NEO_2_7B, 
    ));
    let config_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoConfigResources::GPT_NEO_2_7B,
    ));
    let vocab_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoVocabResources::GPT_NEO_2_7B,
    ));
    let merges_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoMergesResources::GPT_NEO_2_7B,
    ));

    let generate_config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        num_beams: 5,
        no_repeat_ngram_size: 2,
        max_length: 100,
        ..Default::default()
    };

    let model = TextGenerationModel::new(generate_config).unwrap();

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("[ERROR] Couldn't read user input!");
        let slipt = line.split('/').collect::<Vec<&str>>();
        let slc = split.as_slice();
        let output = model.generate(&slc[1..], Some(slc[0]));
        for sentence in output {
            println!("{}", sentence);
        }
    }
}

