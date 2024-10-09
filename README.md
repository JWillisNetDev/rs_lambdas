# Hello, World!

## Packaging your Lambda Function as a ZIP Archive

1. Using pip, install the "cargo-lambda" tool: `pip3 install cargo-lambda`
1. You can create a new package already in the correct Lambda structure using `cargo lambda new {fn-name}`, where `{fn-name}` is any valid project name for your Lambda.
1. Run the `build` subcommand to compile the Lambda function code in release mode: `cargo lambda build --release`

## Deploying your Lambda Function using `cargo lambda`

1. Use the `deploy` subcommand to deploy your compiled binary as a Lambda function: `cargo lambda deploy {fn-name}`

## Testing your deployed Lambda Function using `cargo lambda`

1. Use the `invoke` subcommand to test your function with an appropriate payload: `cargo lambda invoke --remote --data-ascii '{"command": "Hello world"}' {{fn-name}}` where `{{fn-name}}` is the name of your deployed function.

## Troubleshooting

### Windows is Failing to Run the Build-Script for a Crate

This probably means that Windows lacks a Unix tool necessary for a crate's build script in your project. `cargo lambda build` will, by default, use zigbuilder for compiling code which can sometimes suffer from there build-script issues.

The solution is to switch our compiler to a more aggressive option with `cross`

1. Install `cross` using `cargo install cross`
    - `cross` is a project which activates activates and builds your Rust code on more consistent Linux containers. Unfortunately, this comes at the cost of speed.
1. Run the Docker daemon (service) and confirm Docker is running using `docker ps -a`
1. When building and packaging your Lambda function source code, use `cargo lambda build --release --compiler cross` instead of allowing the compiler flag to fall-through to its default value.