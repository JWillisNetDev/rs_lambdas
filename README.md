# Hello, World!

## Packaging your Lambda Function as a ZIP Archive

1. Using pip, install the "cargo-lambda" tool: `pip3 install cargo-lambda`
1. You can create a new package already in the correct Lambda structure using `cargo lambda new {fn-name}`, where `{fn-name}` is any valid project name for your Lambda.
1. Run the `build` subcommand to compile the Lambda function code in release mode: `cargo lambda build --release`

## Deploying your Lambda Function using `cargo lambda`

1. Use the `deploy` subcommand to deploy your compiled binary as a Lambda function: `cargo lambda deploy {fn-name}`

## Testing your deployed Lambda Function using `cargo lambda`

1. Use the `invoke` subcommand to test your function with an appropriate payload: `cargo lambda invoke --remote --data-askii '{"command": "Hello world"}' {{fn-name}}` where `{{fn-name}}` is the name of your deployed function.