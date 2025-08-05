package hello;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Map;
import java.util.Objects;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

import org.apache.logging.log4j.Level;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.apache.logging.log4j.core.config.Configurator;

import com.hubspot.jinjava.Jinjava;

import io.javalin.Javalin;

public class App {

    private static final Logger LOGGER = LogManager.getLogger();

    private static final ConcurrentMap<String, String> TEMPLATES = new ConcurrentHashMap<>();
    private static final Jinjava JINJAVA;

    static {
        JINJAVA = new Jinjava();
        JINJAVA.setResourceLocator((fullName, charset, interperter) -> {
            return locateTemplate(fullName);
        });
    }

    public static void main(String[] args) throws Exception {
        var config = Config.load();

        Configurator.setRootLevel(Level.valueOf(config.logLevel()));

        LOGGER.info("Loaded configuration: {}", config);

        Javalin
                .create(javalinConfig -> {
                    javalinConfig.showJavalinBanner = false;
                    javalinConfig.useVirtualThreads = true;

                    javalinConfig.jetty.defaultHost = config.httpHost();
                    javalinConfig.jetty.defaultPort = config.httpPort();

                    javalinConfig.fileRenderer((filePath, model, ctx) -> {
                        return JINJAVA.render(locateTemplate(filePath), model);
                    });
                })
                .get("/", ctx -> {
                    var name = Objects.requireNonNullElse(ctx.queryParam("name"), "World");

                    ctx.header("content-type", "text/html; charset=utf-8");
                    ctx.render("greeter.html", Map.of("name", name));
                })
                .start();
    }

    private static String locateTemplate(String name) {
        return TEMPLATES.computeIfAbsent(name, (key) -> {
            try {
                var resourceName = "templates/" + key;
                var stream = ClassLoader.getSystemResourceAsStream(resourceName);
                return new String(stream.readAllBytes(), StandardCharsets.UTF_8);
            } catch (IOException | RuntimeException e) {
                throw new RuntimeException("Loading template '" + key + "' failed", e);
            }
        });
    }
}
