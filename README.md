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
### Design Details
1. The user runs the weather CLI application and inputs the required arguments.
1. WeatherCliArgs struct holds the input arguments and passes it to the appropriate subcommand (configure, get, info, reset) based on the user's input.
    * ConfigureArgs struct holds the provider name, key, and value of the settings for the configure sub-command.
    * GetWeatherArgs struct holds the address of the location and the date (optional) for the get sub-command.
1. Provider Interface: The selected provider will implement the WeatherProvider trait, which defines the interface for a weather provider. The get_weather method in the trait will be called to retrieve the weather information for the given location and date.

1. Provider Implementation: Each weather provider will have its own implementation of the WeatherProvider trait, which will provide the weather information by calling its own API or parsing its data.

1. ProviderManagerBuilder struct used to build the ProviderManager. It holds two HashMaps: providers and builders. The providers HashMap holds instances of weather providers, while the builders HashMap holds functions to build weather providers.

1. ProviderManager struct is the main component that manages the weather providers, both pre-existing and newly built. It has two HashMaps: providers and builders. The providers HashMap holds instances of weather providers, while the builders HashMap holds functions to build weather providers. The ProviderManager provides methods to retrieve a list of all providers, check if a provider is supported, and retrieve a reference to a specific provider.

1. Weather Information: The weather information will be returned as a Weather struct, which will contain details such as temperature, condition, etc.

1. Error Handling: If there is an error while retrieving the weather information, the provider will return it's own error, which will be handled by the main application in error_to_user_output function and displayed to the user.

1. Output: The retrieved weather information will be displayed to the user, based on the user's preference. The output format can also be specified through command-line arguments or a configuration file.