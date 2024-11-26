Command and API Call Templates for catsh

This documentation explains how to use the provided templates to create new commands, subcommands, and API calls in the catsh CLI application.

Adding a New Command

1. Define the Command in main.rs
Add a new entry to the Commands enum.
Define subcommands for the new command, if needed, using an enum or struct.
Example:

#[derive(Debug, Subcommand)]
pub enum Commands {
    NewCommand {
        #[command(subcommand)]
        subcommand: NewCommandSubcommands,
    },
    // Existing commands...
}

#[derive(Debug, Subcommand)]
pub enum NewCommandSubcommands {
    ActionOne,
    ActionTwo {
        #[arg(help = "Parameter for ActionTwo")]
        param: String,
    },
}
2. Implement the Command Logic
Add a handler function for the new command in main.rs. The handler should:

Load configuration.
Authenticate to get a valid token.
Use the appropriate API calls based on the subcommand.
Example:

fn handle_new_command(subcommand: NewCommandSubcommands) {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        let config = match config::load_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                return;
            }
        };

        let token = match api::auth::authenticate(&config).await {
            Ok(t) => t,
            Err(e) => {
                error!("Authentication failed: {}", e);
                return;
            }
        };

        match subcommand {
            NewCommandSubcommands::ActionOne => {
                println!("Performing ActionOne...");
                // Call appropriate API function
            }
            NewCommandSubcommands::ActionTwo { param } => {
                println!("Performing ActionTwo with parameter: {}", param);
                // Call appropriate API function
            }
        }
    });
}
3. Integrate the Command into the REPL
Update the REPL handler in main.rs to include the new command:

match cli.command {
    Commands::NewCommand { subcommand } => handle_new_command(subcommand),
    // Other commands...
}
Making API Calls

Use the provided API call templates in templates.rs for different HTTP methods. These templates handle:

Authentication (using a valid token).
Token expiration (re-authentication if necessary).
Error handling and response parsing.
Supported API Call Types
GET Request
Use the api_get function to fetch data:

let response = api_get::<ResponseType>(&client, &config, &token, "endpoint").await?;
println!("Response: {:?}", response);
Example:

let devices = api_get::<Vec<Device>>(&client, &config, &token, "network-devices").await?;
POST Request
Use the api_post function to send data:

let payload = RequestType { field: "value" };
let response = api_post::<ResponseType, RequestType>(&client, &config, &token, "endpoint", &payload).await?;
println!("Response: {:?}", response);
Example:

let new_device = NewDevice { name: "Router1" };
let result = api_post::<Device, NewDevice>(&client, &config, &token, "network-device/add", &new_device).await?;
PUT Request
Use the api_put function to update data:

let payload = RequestType { field: "new_value" };
let response = api_put::<ResponseType, RequestType>(&client, &config, &token, "endpoint", &payload).await?;
println!("Response: {:?}", response);
Example:

let update_device = UpdateDevice { id: "12345", name: "UpdatedRouter" };
let result = api_put::<Device, UpdateDevice>(&client, &config, &token, "network-device/update", &update_device).await?;
DELETE Request
Use the api_delete function to delete data:

let response = api_delete::<ResponseType>(&client, &config, &token, "endpoint").await?;
println!("Response: {:?}", response);
Example:

let result = api_delete::<DeleteResponse>(&client, &config, &token, "network-device/delete/12345").await?;
Example Data Structures

Define the data structures for request payloads (Serialize) and response types (Deserialize).

Request Example:
#[derive(Serialize)]
pub struct NewDevice {
    pub name: String,
}
Response Example:
#[derive(Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    // Additional fields...
}
Extending the Templates

Adding More Subcommands:
Add new subcommands to the appropriate enum.
Create corresponding handler logic and tie it to the REPL.
Adding New API Methods:
Extend the API client with additional utility functions if needed.
Follow the same structure for authentication and error handling.
Best Practices

Error Handling: Use the anyhow crate to handle and propagate errors effectively.
Authentication: Always check token validity before making API calls.
Logging: Use log for debugging and error messages.
Consistency: Use the provided templates for consistent command and API integration.
Example Command Workflow

Define the command in main.rs:
Commands::ExampleCommand { subcommand }
Implement the handler in main.rs:
fn handle_example_command(subcommand: ExampleSubcommands) { ... }
Create API calls in a separate module:
pub async fn fetch_example(...) -> Result<ResponseType> { ... }
Tie it all together in the REPL:
match cli.command {
    Commands::ExampleCommand { subcommand } => handle_example_command(subcommand),
    ...
}
With this structure, you can easily extend catsh with new commands and integrate API calls into the REPL.
