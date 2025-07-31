package hello;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.Objects;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.teesoft.jackson.dataformat.toml.TOMLMapper;

record Config(
        @JsonProperty(value = "log_level", required = false)
        String logLevel,
        @JsonProperty(value = "http_host", required = false)
        String httpHost,
        @JsonProperty(value = "http_port", required = false)
        Integer httpPort
        ) {

    static Config load() throws IOException {
        var defaults = defaults();

        try {
            var mapper = new TOMLMapper();
            var config = mapper.readValue(new File("config.toml"), Config.class);

            return new Config(
                    Objects.requireNonNullElse(config.logLevel, defaults.logLevel()),
                    Objects.requireNonNullElse(config.httpHost, defaults.httpHost()),
                    Objects.requireNonNullElse(config.httpPort, defaults.httpPort())
            );
        } catch (FileNotFoundException e) {
            return defaults;
        }
    }

    static Config defaults() {
        return new Config("info", "0.0.0.0", 8080);
    }
}
