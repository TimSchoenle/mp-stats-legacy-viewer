# MP Legacy Stats Viewer

This project provides a comprehensive platform for parsing, converting, and viewing historical statistics for a legacy Minecraft-Server. The platform consists of a backend data server, a frontend web application, and data conversion utilities.

## Architecture

The project is built entirely in Rust and is structured as a Cargo workspace containing the following components:

### Applications

*   **Frontend** (`apps/frontend`): A WebAssembly-based client application built with Yew. It provides a responsive interface for searching players and viewing their statistics.
*   **Server** (`apps/server`): A REST API backend built with Axum. It serves the processed statistical data to the frontend application.
*   **Converter** (`apps/converter`): A command-line utility for processing, parsing, and converting raw binary data dumps into an optimized format suitable for efficient querying.

### Shared Libraries

*   **Core** (`crates/core`): Contains the core data models, parsing logic, and business rules shared across all applications.
*   **Common** (`crates/common`): Provides common utilities and helper functions.

## Technology Stack

*   **Language:** Rust
*   **Backend Server:** Axum
*   **Frontend Framework:** Yew, WebAssembly (wasm-bindgen)
*   **Styling:** Tailwind CSS
*   **Data Processing:** Postcard, Serde, Rayon

## Getting Started

### Prerequisites

To build and run the project locally, you will need the following tools installed:

*   [Rust Toolchain](https://rustup.rs/) (edition 2024 support)
*   [Trunk](https://trunkrs.dev/) (for building the WebAssembly frontend)
*   [Node.js](https://nodejs.org/) (required for Tailwind CSS processing)

### Local Development

1.  **Prepare the Frontend**
    Navigate to the frontend directory and install the required npm dependencies:
    ```sh
    cd apps/frontend
    npm install
    ```

2.  **Start the Backend Server**
    Run the Axum server from the workspace root:
    ```sh
    cargo run -p mp-stats-server
    ```
    *(Note: Depending on configuration, ensure you have the required data files in the expected directories.)*

3.  **Start the Frontend Client**
    In a separate terminal, use Trunk to serve the frontend application:
    ```sh
    cd apps/frontend
    trunk serve
    ```

### Docker Setup

The easiest way to run the entire stack (Frontend, Backend, and Data processing) without needing Rust or Node.js locally is via Docker.

1.  **Place Data Files**
    Ensure your raw data files are placed in a directory (e.g., `data/`) at the workspace root before building or use build arguments to adjust the data directory.

2.  **Build and Run the Container**
    ```sh
    docker build -t mp-stats-viewer .
    docker run -p 8080:8080 mp-stats-viewer
    ```
    The web interface will be accessible at `http://localhost:8080`.

## Data Conversion

To process raw statistical data into the optimized format used by the server, use the converter utility:

```sh
cargo run -p mp-stats-converter -- [options]
```

Please refer to the internal documentation within the `apps/converter` crate for detailed usage instructions and supported data formats.

## Acknowledgements

The historical data used and viewed in this project was originally collected utilizing the [StatsApi](https://github.com/TimSchoenle/StatsApi) utility.
