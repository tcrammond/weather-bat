# get-weather

The API handler for weather-bat. This runs as a [Netlify Function](https://docs.netlify.com/functions/overview/).

## Running locally

Requirements: [Netlify CLI](https://docs.netlify.com/cli/get-started/) and an [OpenWeather](https://openweathermap.org/api) API key.

From the root of the project:

```shell
# Set the Netlify environment variable for your OpenWeather API key
netlify env:set WEATHER_KEY XXXXXXXXXX

# Run API functions
netlify-dev
```

This will run all API functions at `http://localhost:8888/.netlify/functions..`. The CLI will use the correct URL automatically when using `cargo run`.

You may access the `get-weather` endpoint in your browser at `http://localhost:8888/.netlify/functions/get-weather?lat=XXX&lon=XXX`. You'll need to specify a latitude and longitude.
