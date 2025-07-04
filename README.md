# Welcome to {{project_name}}!

This beginner-friendly Rust web application template uses the **Actix web framework** and **{{database}}** to help you quickly build APIs or web applications. Whether you’re new to Rust or an experienced developer, this template makes setup easy and lets you focus on building your project.

## Why Use This Template?

- **Fast Web Server**: Powered by Actix web with built-in logging and rate limiting.
- **Database Ready**: Connects to {{database}} using SQLx for reliable database operations.
- **Quick Testing**: Includes a `/health` endpoint to check if your server is running.
- **Flexible**: Easily add new endpoints or database features.

## What You’ll Need

Before you begin, ensure you have the following:

- **Rust**: The latest stable version. Install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  After installation, run `rustc --version` to confirm.
- **cargo-generate**: A tool to generate your project from this template. Install it with:
  ```sh
  cargo install cargo-generate
  ```
- **Database**:
  {% if database == "sqlite" %}
  - **SQLite**: No extra setup needed! SQLite uses a file-based database that’s created automatically.
  {% endif %}
  {% if database == "postgres" %}
  - **PostgreSQL**: A running PostgreSQL server. Download from [postgresql.org](https://www.postgresql.org/download/).
  {% endif %}
  {% if database == "mysql" %}
  - **MySQL**: A running MySQL server. Download from [mysql.com](https://www.mysql.com/downloads/).
  {% endif %}

## Step-by-Step Setup

Follow these simple steps to get your project up and running. We’ll guide you every step of the way!

### 1. Create Your Project

1. Run the following command to generate your project:
   ```sh
   cargo generate --git https://github.com/PRiEsTCSC/rust_webserver_template.git
   ```
2. Answer the prompts:
   - **Project name**: Enter a name for your project (e.g., `myapp`). This will be your project folder and Rust package name.
   - **Database**: Choose `postgres`, `mysql`, or `sqlite` based on your preference.

   This creates a new folder (e.g., `myapp`) containing all project files.

### 2. Set Up Your Database

{% if database == "postgres" %}
- **Install PostgreSQL**: Follow the instructions at [postgresql.org](https://www.postgresql.org/download/) for your operating system.
- **Create a Database**: For example, to create a database named `mydb`, run:
  ```sh
  psql -U postgres -c "CREATE DATABASE mydb;"
  ```
- **Note**: Save your database username, password, and port (default: `5432`).
{% endif %}
{% if database == "mysql" %}
- **Install MySQL**: Follow the instructions at [mysql.com](https://www.mysql.com/downloads/) for your operating system.
- **Create a Database**: For example, to create a database named `mydb`, run:
  ```sh
  mysql -u root -p -e "CREATE DATABASE mydb;"
  ```
- **Note**: Save your database username, password, and port (default: `3306`).
{% endif %}
{% if database == "sqlite" %}
- **No Setup Needed**: SQLite is file-based, so you’ll just specify a file path (e.g., `mydb.sqlite`) in the next step.
{% endif %}

### 3. Configure Your Environment

1. Navigate to your project folder:
   ```sh
   cd {{project_name}}
   ```
2. Create a `.env` file in the project root to store configuration settings:
   ```sh
   echo "HOST=127.0.0.1" > .env
   echo "PORT=8080" >> .env
   echo "DATABASE_URL=<your_database_url>" >> .env
   ```
3. Set the `DATABASE_URL` based on your chosen database:
   {% if database == "postgres" %}
   ```sh
   DATABASE_URL=postgres://<username>:<password>@localhost:5432/mydb
   ```
   **Example**: `postgres://myuser:mypass@localhost:5432/mydb`
   {% endif %}
   {% if database == "mysql" %}
   ```sh
   DATABASE_URL=mysql://<username>:<password>@localhost:3306/mydb
   ```
   **Example**: `mysql://myuser:mypass@localhost:3306/mydb`
   {% endif %}
   {% if database == "sqlite" %}
   ```sh
   DATABASE_URL=sqlite://mydb.sqlite
   ```
   **Example**: `sqlite://mydb.sqlite`
   {% endif %}
   **Tip**: Replace `<username>`, `<password>`, and `mydb` with your actual database credentials and name. For SQLite, use any valid file path (e.g., `./mydb.sqlite`).

### 4. Install Dependencies

Build the project to download and compile dependencies:
```sh
cargo build
```
This may take a few minutes the first time. If you encounter errors, check the **Troubleshooting** section below.

### 5. Run Your Application

Start the web server:
```sh
cargo run
```
- The server will start at `http://127.0.0.1:8080`.
- To verify it’s working, open a new terminal and run:
  ```sh
  curl http://127.0.0.1:8080/health
  ```
- You should see: `Server is healthy`.

**Congratulations!** Your web server is up and running!

## What’s Included in Your Project?

Here’s a breakdown of the key files in your project:

| File/Folder             | Purpose                                                                 |
|-------------------------|-------------------------------------------------------------------------|
| `src/main.rs`           | The entry point of your app, setting up logging and starting the server. |
| `src/init.rs`           | Configures the Actix web server with logging, rate limiting, and routes. |
| `src/configs/env_load.rs` | Loads settings (`HOST`, `PORT`, `DATABASE_URL`) from the `.env` file.  |
| `src/models/database.rs` | Manages the connection pool to your {{database}} database using SQLx.   |
| `src/routes/health.rs`  | Defines the `/health` endpoint to check server status.                   |
| `log4rs.yaml`           | Configures logging output (e.g., to the console or a file).              |

## Customizing Your Project

Want to add your own features? Here are some common ways to extend the template:

### Adding New Endpoints

1. Create a new file in `src/routes/` (e.g., `example.rs`):
   ```rust
   use actix_web::{get, HttpResponse, Responder};

   #[get("/example")]
   async fn example() -> impl Responder {
       HttpResponse::Ok().body("Hello from the example endpoint!")
   }
   ```
2. Update `src/routes/mod.rs` to include the new module:
   ```rust
   pub mod health;
   pub mod example;
   ```
3. Add the endpoint to `src/init.rs` in the `init_routes` function:
   ```rust
   cfg.service(example::example);
   ```
4. Restart the server and test:
   ```sh
   curl http://127.0.0.1:8080/example
   ```
   Expected output: `Hello from the example endpoint!`

### Adding Database Queries

1. In `src/models/database.rs`, add a function to query your database:
   ```rust
   impl DatabaseConnection {
       pub async fn get_users(&self) -> Result<Vec<String>, sqlx::Error> {
           let rows = sqlx::query_scalar("SELECT name FROM users")
               .fetch_all(&self.pool)
               .await?;
           Ok(rows)
       }
   }
   ```
   **Note**: Ensure your database has a `users` table, or modify the query for your schema.
2. Create a route in `src/routes/users.rs`:
   ```rust
   use actix_web::{get, web, HttpResponse, Responder};
   use crate::models::database::DatabaseConnection;

   #[get("/users")]
   async fn get_users(db: web::Data<DatabaseConnection>) -> impl Responder {
       match db.get_users().await {
           Ok(users) => HttpResponse::Ok().json(users),
           Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
       }
   }
   ```
3. Register the route in `src/init.rs` and `src/routes/mod.rs`.
4. Test with:
   ```sh
   curl http://127.0.0.1:8080/users
   ```

### Changing Settings

- **Logging**: Modify `log4rs.yaml` to change log verbosity or output to a file:
  ```yaml
  appenders:
    file:
      kind: file
      path: "app.log"
      encoder:
        pattern: "{d(%Y-%m-%d %H:%M:%S)} [{t}] {l} - {m}{n}"
    stdout:
      kind: console
      encoder:
        pattern: "{d(%Y-%m-%d %H:%M:%S)} [{t}] {l} - {m}{n}"
  root:
    level: debug
    appenders:
      - stdout
      - file
  ```
  This logs to both the console and `app.log` with `debug` level.
- **Rate Limiting**: Adjust settings in `src/init.rs`:
  ```rust
  .seconds_per_request(60) // Allow one request per minute
  .burst_size(10) // Allow bursts of up to 10 requests
  ```

## Troubleshooting

If something goes wrong, try these solutions:

- **“Database connection failed”**
  - **Check**: Ensure `DATABASE_URL` in `.env` is correct.
  - **PostgreSQL/MySQL**: Verify the database server is running and credentials (username, password, port) are valid.
  - **SQLite**: Confirm the file path is accessible and writable.
  - **Test**: For PostgreSQL/MySQL, connect using a client (e.g., `psql` or `mysql`).

- **“Port already in use”**
  - **Fix**: Change `PORT` in `.env` to another value (e.g., `8081`).
  - **Or**: Stop the conflicting process:
    ```sh
    killall cargo
    ```
  - **Check**: Use `netstat -tuln | grep 8080` to see if the port is occupied.

- **“Build fails with dependency errors”**
  - **Update Rust**: Run `rustup update`.
  - **Clear Cache**: Run `cargo clean`.
  - **Inspect**: Read the error messages in the terminal for clues (e.g., missing system libraries).
  - **Dependencies**: Ensure you have required libraries:
    - For PostgreSQL: Install `libpq-dev` (e.g., `sudo apt-get install libpq-dev` on Ubuntu).
    - For MySQL: Install `libmysqlclient-dev`.
    - For SQLite: Install `libsqlite3-dev`.

- **“No logs appear”**
  - **Check**: Ensure `log4rs.yaml` exists in the project root and is valid.
  - **Fix**: Set `level: debug` in `log4rs.yaml` for more detailed output:
    ```yaml
    root:
      level: debug
    ```

- **“Template generation fails”**
  - **Check**: Ensure you’re using the correct repository URL: `https://github.com/PRiEsTCSC/rust_webserver_template.git`.
  - **Clear Cache**: Remove cached template data:
    ```sh
    rm -rf ~/.cargo/git/db/rust_webserver_template-*
    ```
  - **Retry**: Run `cargo generate` again.

## Getting Help

- **Logs**: Check the terminal or `app.log` (if configured) for error details.
- **Repository**: Visit [https://github.com/PRiEsTCSC/rust_webserver_template](https://github.com/PRiEsTCSC/rust_webserver_template) to open an issue or find support.
- **Community**: Ask questions in Rust communities like [r/rust](https://www.reddit.com/r/rust/) or the [Rust Discord](https://discord.gg/rust-lang).

## Contributing

Love this template? Want to make it better?
- Fork the repository at [https://github.com/PRiEsTCSC/rust_webserver_template](https://github.com/PRiEsTCSC/rust_webserver_template).
- Make your changes and submit a pull request.
- Share feedback or report bugs via GitHub issues.

## License

This project is licensed under the **MIT License**. Feel free to use, modify, and share it!

## Next Steps

- **Add an Endpoint**: Create a new route (see “Adding New Endpoints”).
- **Work with Data**: Try database queries (see “Adding Database Queries”).
- **Learn More**: Explore the [Actix web documentation](https://actix.rs/docs/) or [SQLx documentation](https://docs.rs/sqlx) for advanced features.

Happy coding with {{project_name}}! 🚀

---

### Instructions to Save as a Text File

1. **Copy the Content**:
   - Copy the entire text block above (starting from `# Welcome to {{project_name}}!` to `Happy coding with {{project_name}}! 🚀`).

2. **Create the File**:
   - Open a text editor (e.g., VS Code, Notepad, or `nano`).
   - Paste the content into the editor.
   - Save the file as `README.md` in your template repository’s root directory (e.g., `rust-web-template/`).

3. **Using a Terminal** (optional):
   - If you prefer the command line, create the file directly:
     ```sh
     nano README.md
     ```
   - Paste the content, save (Ctrl+O, Enter), and exit (Ctrl+X).
   - Alternatively, redirect the content to a file if you have it in a script or copy-paste:
     ```sh
     cat > README.md
     ```
     Paste the content, then press Ctrl+D to save.

4. **Verify the File**:
   - Ensure the file is named `README.md` (case-sensitive) and is in the root of your template directory.
   - Check that it renders correctly in a Markdown viewer (e.g., GitHub or VS Code).

### Notes
- **Markdown Formatting**: The content uses standard Markdown syntax (e.g., `#` for headers, ``` for code blocks, `|` for tables) to ensure it renders correctly in Markdown viewers like GitHub.
- **Liquid Templating**: The `{{project_name}}` and `{{database}}` placeholders, along with `{% if database == "..." %}` conditionals, ensure the README adapts to user inputs during `cargo generate`.
- **Error Context**: The previous `{{project_name}}` error (`invalid character '{' in package name`) is unrelated to the README but suggests a `cargo-generate` configuration issue. Ensure your `cargo-generate.toml` includes:
  ```toml
  [placeholders.project_name]
  type = "string"
  prompt = "What is your project name?"
  regex = "^[a-zA-Z][a-zA-Z0-9_-]*$"
  description = "The name of the project, used in Cargo.toml."
  ```
  And that `Cargo.toml` uses `name = "{{project_name}}"`.

If you need help with the full template setup or resolving the `cargo-generate` error, please share your `cargo-generate.toml` or other relevant files!