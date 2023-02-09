# Weather CLI
A Command-Line Interface (CLI) tool for fetching weather information from various providers.
## Prerequisites
To use this CLI, you need to have Rust installed on your system. You can install it from [here](https://www.rust-lang.org/tools/install).

## Installation 
Download the source code.
```
git clone https://github.com/vadymkurinnyi/weather-cli.git
```
Change the current working directory to the "weather-cli" directory, which contains the source code for the project.
```
cd weather-cli
```
Build the application in release mode.
```
cargo build --release
```
Change to the "weather" directory.
```
cd weather
```
Install the application. The --path flag specifies the current directory, which contains the built application. This installs the application globally on your system, so you can run it from the command line.
```
cargo install --path <current-directory>
```
## Usage
The CLI has four main sub-commands: configure, get, info, and reset

## Configure
The configure sub-command allows you to configure the weather provider you want to use. You can also set or update the API key (if required by the provider).
```
weather configure <provider_name> [<api_key>] [<api_key_value>]
```
### Supported providers
| Name          |      API      |
| ------------- |:-------------:|
| weather-api   | https://www.weatherapi.com/docs/ |
| open-weather  | https://openweathermap.org/api |
## Get
The get sub-command is used to get weather information for a specific location. You can provide the location either as an address or as a set of coordinates. You can also provide a date to get the weather information for that date (optional).

```
weather get <location> [<date>]
```
## Info
The info sub-command is used to get information about the current weather provider and its settings.
```
weather info
```
## Reset
The reset sub-command is used to reset the weather provider and its settings to the default.
```
weather reset
```
## Examples
#### Set provider
```
weather configure open-weather
```
#### Set provider API key
```
weather configure open-weather apiKey 298a5d93dc8d9956ae0404b7fbe46ec3
```
#### Get weather today
```
weather get London 
```
#### Get weather for date
```
weather get Lviv 2023-02-10 
```