<p align="center">
<picture>
    <source media="(prefers-color-scheme: dark)" srcset="img/atlas-pathways-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="img/atlas-pathways-light.svg">
    <img src="img/atlas-pathways-light.svg" style="width: 40%; height: 40%;" alt="Atlas logo">
</picture>
<br>
<a href="https://atlaspathways.github.io/docs/"><img src="https://img.shields.io/badge/📖 docs-atlas.rs-dca282.svg" /></a> &nbsp;
<a href="https://crates.io/crates/atlas-core"><img src="https://img.shields.io/crates/v/atlas-core.svg?color=dca282" /></a>
&nbsp;
<a href="https://crates.io/crates/atlas-core"><img src="https://img.shields.io/crates/d/atlas-core.svg?color=dca282" /></a>
</br>
<a href="https://discord.com/invite/gGqttYU59M"><img src="https://img.shields.io/discord/511303648119226382?color=%236d82cc&label=Discord&logo=discord&logoColor=white" /></a>
&nbsp;
<a href="https://github.com/AtlasPathways/AtlasPathwaysAI"><img src="https://img.shields.io/github/stars/AtlasPathways/AtlasPathwaysAI?style=social" alt="stars - Atlas" /></a>
<br>
<a href=""><img src="https://img.shields.io/badge/built_with-Rust-dca282.svg?logo=rust" /></a>
&nbsp;
<a href="https://x.com/AtlasPathways"><img src="https://img.shields.io/twitter/follow/AtlasPathways"></a> &nbsp

<br>
</p>
&nbsp;

✨ If you would like to help spread the word about Atlas, please consider starring the repo!

> [!WARNING]
> Here be dragons! As we plan to ship a torrent of features in the following months, future updates **will** contain **breaking changes**. With Atlas evolving, we'll annotate changes and highlight migration paths as we encounter them.


## [What is Atlas?](https://atlaspathways.github.io/docs/docs/1_why_atlaspathwaysai)
Atlas is a Rust library for building scalable, modular, and ergonomic **LLM-powered** applications.

More information about this crate can be found in the [official](https://atlaspathways.github.io/docs/) documentations.

Help us improve Atlas by contributing to our Feedback form.

## High-level features
- Full support for LLM completion and embedding workflows
- Simple but powerful common abstractions over LLM providers (e.g. OpenAI, Cohere) and vector stores (e.g. Luna, SQlite, in-memory)
- Integrate LLMs in your app with minimal boilerplate


## [Get Started](https://atlaspathways.github.io/docs/guides/0_text_extraction_classification)
```bash
cargo add atlas-core
```

## [Integrations](https://atlaspathways.github.io/docs/docs/5_integrations)

| Model Providers |                                                                                                                                                                                                                                                                                                               Vector Stores                                                                                                                                                                                                                                                                                                               |
|:--------------:|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
| <br><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/04/ChatGPT_logo.svg/1024px-ChatGPT_logo.svg.png" alt="ChatGPT logo" width="50em"> <picture><source media="(prefers-color-scheme: dark)" srcset="https://www.fahimai.com/wp-content/uploads/2024/06/Untitled-design-7.png"><source media="(prefers-color-scheme: light)" srcset="https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/Claude_Ai.svg/1024px-Claude_Ai.svg.png"><img src="https://www.fahimai.com/wp-content/uploads/2024/06/Untitled-design-7.png" alt="Claude Anthropic logo" width="50em"></picture> <br> <img src="https://cdn.sanity.io/images/rjtqmwfu/production/0adbf394439f4cd0ab8b5b3b6fe1da10c8099024-201x200.svg" alt="Cohere logo" width="50em"> <img src="https://logospng.org/download/google-gemini/google-gemini-1024.png" style="background-color: white; border-radius: 10px; padding: 5px 5px ; width: 3em;" alt="Gemini logo"> <br> <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/57/XAI-Logo.svg/512px-XAI-Logo.svg.png?20240912222841" style="background-color: white; border-radius: 10px; padding: 5px 5px ; width: 3em;" alt="xAI logo"> <img src="https://github.com/user-attachments/assets/4763ae96-ddc9-4f69-ab38-23592e6c4ead" style="background-color: white; border-radius: 10px; padding: 5px 0px ; width: 4em;" alt="perplexity logo">| <br><img src="https://cdn.prod.website-files.com/6640cd28f51f13175e577c05/664e00a400e23f104ed2b6cd_3b3dd6e8-8a73-5879-84a9-a42d5b910c74.svg" alt="Luna DB logo" width="50em"> <img src="https://upload.wikimedia.org/wikipedia/commons/e/e5/Neo4j-logo_color.png" alt="Sunny logo" style="background-color: white; border-radius: 1em; padding: 1em 1em ; width: 4em;"><br><br><img src="https://cdn-images-1.medium.com/max/844/1*Jp6VwF0OcdeyRyW0Ln0RMQ@2x.png" width="100em" alt="Lance DB logo"> <br> <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/38/SQLite370.svg/440px-SQLite370.svg.png" style="width: 6em"> |


Vector stores are available as separate companion-crates:
- Luna vector store: [`atlas-luna`](https://github.com/AtlasPathways/AtlasPathwaysAI/tree/main/atlas-luna)
- LanceDB vector store: [`atlas-lancedb`](https://github.com/AtlasPathways/AtlasPathwaysAI/tree/main/atlas-lancedb)
- Sunny vector store: [`atlas-sunny`](https://github.com/AtlasPathways/AtlasPathwaysAI/tree/main/atlas-sunny)
- Qdrant vector store: [`atlas-qdrant`](https://github.com/AtlasPathways/AtlasPathwaysAI/tree/main/atlas-qdrant)
- SQLite vector store: [`atlas-sqlite`](https://github.com/AtlasPathways/AtlasPathwaysAI/tree/main/atlas-sqlite)


<p align="center">
<br>
<br>
<img src="img/built-by-atlas.svg" alt="Built by Atlas" width="30%">
</p>
