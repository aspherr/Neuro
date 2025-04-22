
# Neuro Quick Start Guide

This project is my university dissertation—an AI-driven note-taking app built with Rust and Tauri.


## File Structure

The main 3 directories in this project are: `src/`, `server/src`, and `static/`

- `src/` holds all the code for the frontend.
- `server/src` holds all the code for the backend
- `static/` holds any static assests like fonts and images

The rest of the project files are config files and module plgins


## Environment Variables

To run this project locally instead of deploying, you will need to add the following environment variables to your .env file

`OPENAI_API_KEY` for GPT LLM Model

`DATABASE_URL` for Redis DB


## Running

This project was built in Tauri and so will require Rust, Tauri and npm installed 

Before running the project it is recommended you clean and rebuild the cargo /bin directory

```bash
  cd server/
  cargo clean && cargo build
```

Then return to the root directory and run the following
```bash
  npm run tauri dev
```

## Deployment

To deploy this project run

```bash
  npm run tauri build
```

This will install the executable in the server/target/release/ directory
## Authors

- [@aspherr](https://github.com/aspherr)

