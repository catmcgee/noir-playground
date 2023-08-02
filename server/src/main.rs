mod challenges;
use challenges::get_challenges;
use serde_derive::{ Deserialize, Serialize };
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::sync::Arc;
use tokio::fs;
use tokio::task;
use toml;
use uuid::Uuid;
use warp::reply::Json;
use warp::{ reject::Reject, Filter, Rejection };

// Structure to receive data
#[derive(Deserialize)]
struct ExecutionInput {
    code: String,
    challenge_id: u32,
    prover_inputs: std::collections::HashMap<String, String>,
}

use std::error::Error;
use std::fmt;

// Custom error type
#[derive(Debug)]
pub struct SimpleRejection(pub String);

impl fmt::Display for SimpleRejection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SimpleRejection {}
impl Reject for SimpleRejection {}

#[derive(Serialize, Deserialize, Debug)]
struct Challenge {
    id: u32,
    description: String,
    test_cases: Vec<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    let challenges = get_challenges();
    let challenges_for_execute = Arc::clone(&challenges);
    // Routes and CORS
    let execute = warp
        ::post()
        .and(warp::path("execute"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&challenges_for_execute)))
        .and_then(execute_code);

    let execute_test = warp
        ::post()
        .and(warp::path("execute_test"))
        .and(warp::body::json())
        .and_then(execute_test_code);

    let execute_check = warp
        ::post()
        .and(warp::path("execute_check"))
        .and(warp::body::json())
        .and_then(execute_check);

    let challenges_route = warp
        ::path("challenges")
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .and(warp::any().map(move || Arc::clone(&challenges)))
        .and_then(get_challenge);

    let cors = warp
        ::cors()
        .allow_any_origin()
        .allow_headers(vec!["Accept", "Content-Type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "HEAD", "OPTIONS"]);

    let routes = execute
        .or(execute_test)
        .or(challenges_route)
        .or(execute_check)
        .recover(handle_rejection)
        .with(cors);

    let server_port: u16 = std::env
        ::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number.");
    println!("Server is running on port: {}", server_port);
    warp::serve(routes).run(([0, 0, 0, 0], server_port)).await;
}

// Function to get challenge from ID
async fn get_challenge(
    id: u32,
    challenges: Arc<Vec<challenges::Challenge>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let challenge = challenges.iter().find(|c| c.id == id);
    match challenge {
        Some(challenge) => {
            let send_challenge = Challenge {
                id: challenge.id,
                description: challenge.description.clone(),
                test_cases: vec![], // Empty test cases when returning to the user
            };
            Ok(warp::reply::json(&send_challenge))
        }
        None => Err(warp::reject::not_found()),
    }
}

// Function to create a project directory for new Noir project
async fn create_project_dir() -> Result<PathBuf, Rejection> {
    // Get current directory
    let cwd = std::env
        ::current_dir()
        .map_err(|err| warp::reject::custom(SimpleRejection(err.to_string())))?;
    // Create new directory name using a random UUID
    let dir_name = format!("{}/tmp/noir_projects/{}", cwd.display(), Uuid::new_v4());
    let project_dir = Path::new(&dir_name);
    // Check if path "/tmp/noir_projects/" exists, if not it creates it
    if !Path::new("/tmp/noir_projects/").exists() {
        tokio::fs
            ::create_dir("/tmp/noir_projects/").await
            .map_err(|e| warp::reject::custom(SimpleRejection(e.to_string())))?;
    }
    // Create new project directory, or return an error if the creation failed
    match tokio::fs::create_dir_all(&project_dir).await {
        Ok(_) => println!("Successfully created directory {:?}", project_dir),
        Err(e) => {
            return Err(
                warp::reject::custom(SimpleRejection(format!("Failed to create dir: {}", e)))
            );
        }
    }
    // Sets up a new Noir project with nargo in the new project directory
    Command::new("nargo")
        .arg("new")
        .arg("project")
        .current_dir(&project_dir)
        .output()
        .map_err(|err| warp::reject::custom(SimpleRejection(err.to_string())))?;

    Ok(project_dir.to_path_buf())
}

// Function to run nargo commands in specified directory
async fn run_command(mut cmd_obj: Command, dir_buf: &PathBuf) -> Result<Output, Rejection> {
    // Clone dir_buf to use in the following spawn_blocking closure
    let dir_buf = dir_buf.clone();

    // Spawn and run command in separate thread
    let spawn_result = task::spawn_blocking(move || {
        cmd_obj.current_dir(&dir_buf);
        cmd_obj.output()
    }).await;

    let output: Output = match spawn_result {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => {
            return Err(warp::reject::custom(SimpleRejection(e.to_string())));
        }
        Err(_) => {
            return Err(warp::reject::custom(SimpleRejection("Failed to perform task".into())));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", &stderr);
        return Err(warp::reject::custom(SimpleRejection(stderr)));
    }

    Ok(output)
}

// Function to run nargo check on user submitted code
async fn execute_check(body: ExecutionInput) -> Result<Json, Rejection> {
    let project_dir = create_project_dir().await.map_err(|err| {
        warp::reject::custom(
            SimpleRejection(format!("Failed to create project directory: {:?}", err))
        )
    })?;
    println!("Project directory in execute function: {:?}", project_dir);

    let code_file_path = project_dir.join("project/src/main.nr");

    // Write code to src/main.nr file
    tokio::fs
        ::write(&code_file_path, &body.code).await
        .map_err(|e| {
            warp::reject::custom(SimpleRejection(format!("Failed to write to file: {}", e)))
        })?;

    let project_sub_dir = project_dir.join("project");

    let output = Command::new("nargo")
        .arg("check")
        .current_dir(&project_sub_dir)
        .output()
        .map_err(|err| warp::reject::custom(SimpleRejection(err.to_string())))?;

    if !output.status.success() {
        return Err(warp::reject::custom(SimpleRejection(String::from("Failed to execute check"))));
    }

    // Read the contents of Prover.toml
    let prover_file_path = project_dir.join("project/Prover.toml");
    let prover_content = tokio::fs
        ::read_to_string(&prover_file_path).await
        .expect("Failed to read Prover.toml");

    // Parse it into a JSON object
    let prover_toml: serde_json::Value = toml
        ::from_str(&prover_content)
        .map_err(|err| warp::reject::custom(SimpleRejection(err.to_string())))?;

    // Remove directory
    if let Err(e) = fs::remove_dir_all(&project_dir).await {
        println!("Removing directory");
        println!("{}", project_dir.display());
        println!("Failed to remove directory {:?}. Error: {}", project_dir, e);
    }

    // Return the parsed content
    Ok(warp::reply::json(&prover_toml))
}

// Function to run nargo test on user submitted code
async fn execute_test_code(body: ExecutionInput) -> Result<Json, Rejection> {
    // Check if "#[test]" exists in the client-side code:
    if !body.code.contains("#[test]") {
        return Ok(warp::reply::json(&"There are no tests to run"));
    }
    let project_dir = create_project_dir().await.map_err(|err| {
        warp::reject::custom(
            SimpleRejection(format!("Failed to create project directory: {:?}", err))
        )
    })?;

    let code_file_path = project_dir.join("project/src/main.nr");

    // Write user code to src/main.nr file
    tokio::fs
        ::write(&code_file_path, &body.code).await
        .map_err(|e| {
            warp::reject::custom(SimpleRejection(format!("Failed to write to file: {}", e)))
        })?;

    // Run nargo test
    let result = run_nargo_test(&project_dir.join("project")).await;

    // Remove directory
    if let Err(e) = fs::remove_dir_all(&project_dir).await {
        println!("Failed to remove directory {:?}. Error: {}", project_dir, e);
    }

    result
}

// Function to run nargo test, which is called by execute_test_code
async fn run_nargo_test(dir_buf: &PathBuf) -> Result<Json, Rejection> {
    let mut cmd_obj = Command::new("nargo");
    cmd_obj.arg("test").current_dir(&dir_buf);

    let mut cmd_obj = Command::new("nargo");
    cmd_obj.arg("test").current_dir(&dir_buf);
    let output = run_command(cmd_obj, &dir_buf).await?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    eprintln!("stdout: {}", stdout);
    eprintln!("stderr: {}", stderr);

    if output.status.success() {
        Ok(warp::reply::json(&format!("Tests pass!\nOutput: {}", stdout)))
    } else {
        Err(warp::reject::custom(SimpleRejection(format!("Tests failed:\n{}", stderr))))
    }
}
// Function to handle execution of user submitted code,
// which includes running pre-written test cases
// and running prover and verifier functions
async fn execute_code(
    body: ExecutionInput,
    challenges: Arc<Vec<challenges::Challenge>>
) -> Result<Json, Rejection> {
    println!("Received code: {}", body.code);

    // Find the challenge with the given ID
    let challenge_opt = challenges.iter().find(|&c| c.id == body.challenge_id);
    let challenge = match challenge_opt {
        Some(challenge) => challenge,
        None => {
            return Err(
                warp::reject::custom(
                    SimpleRejection(format!("No challenge found for the given ID"))
                )
            );
        }
    };

    // Combine user submitted code with test cases from the chosen challenge
    let combined_code = format!("{}\n{}", body.code, challenge.test_cases.join("\n"));

    let project_dir = create_project_dir().await.map_err(|err| {
        warp::reject::custom(
            SimpleRejection(format!("Failed to create project directory: {:?}", err))
        )
    })?;

    // Define the file paths for the Noir code and the Prover inputs
    let code_file_path = project_dir.join("project/src/main.nr");
    let prover_file_path = project_dir.join("project/Prover.toml");

    // Write the combined code to the src/main.nr file.
    tokio::fs
        ::write(&code_file_path, &combined_code).await
        .map_err(|e| {
            warp::reject::custom(SimpleRejection(format!("Failed to write to file: {}", e)))
        })?;

    // Write the Prover inputs to the Prover.toml file
    tokio::fs
        ::write(&prover_file_path, toml::to_string(&body.prover_inputs).unwrap()).await
        .map_err(|e| {
            warp::reject::custom(SimpleRejection(format!("Failed to write to Prover.toml: {}", e)))
        })?;

    // Run the nargo commands to test, prove and verify
    let result = run_nargo_commands(&project_dir.join("project")).await;

    // Remove directory
    if let Err(e) = fs::remove_dir_all(&project_dir).await {
        eprintln!("Failed to remove directory {:?}. Error: {}", project_dir, e);
    }

    // Return the result
    result
}

// Function to run all nargo commands for executing user submitted code
async fn run_nargo_commands(dir_buf: &PathBuf) -> Result<Json, Rejection> {
    let commands = vec![
        vec!["test"],
        vec!["check"],
        vec!["prove", "proof-1"],
        vec!["verify", "proof-1"]
    ];

    for command in commands {
        let mut cmd_obj = Command::new("nargo");
        for arg in &command {
            cmd_obj.arg(arg);
        }
        cmd_obj.current_dir(&dir_buf);

        let output = run_command(cmd_obj, &dir_buf).await?;

        if command[0] == "prove" {
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            if output.status.success() {
                if stdout.contains("FAILED") {
                    return Err(warp::reject::custom(SimpleRejection(String::from("Prove failed"))));
                }
            } else {
                return Err(
                    warp::reject::custom(SimpleRejection(String::from("Nargo command failed")))
                );
            }
        }
    }

    Ok(warp::reply::json(&"Success!"))
}

// Function to handle routing errors
async fn handle_rejection(
    err: warp::Rejection
    // If the error can be handled as a SimpleRejection, we respond with an ErrorResponse
    // containing the SimpleRejection's error message and 505 HTTP status
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    if let Some(e) = err.find::<SimpleRejection>() {
        let error = ErrorResponse {
            message: e.0.clone(),
        };
        return Ok(
            warp::reply::with_status(
                warp::reply::json(&error),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR
            )
        );
    }

    // If the error cannot be handled as a SimpleRejection, then return a new ErrorResponse
    // indicating that the requested resource was not found, along with "Not Found" HTTP status code
    let error = ErrorResponse {
        message: "Fot found".into(),
    };
    Ok(warp::reply::with_status(warp::reply::json(&error), warp::http::StatusCode::NOT_FOUND))
}
