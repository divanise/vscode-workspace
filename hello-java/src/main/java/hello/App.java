package hello;

import org.apache.logging.log4j.Level;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.apache.logging.log4j.core.config.Configurator;

import io.javalin.Javalin;

public class App {

    private static final Logger LOGGER = LogManager.getLogger();

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
                })
                .get("/", ctx -> {
                    var loader = ClassLoader.getSystemResourceAsStream("html/index.html");
                    ctx.writeSeekableStream(loader, "text/html; charset=utf-8");
                })
                .start();
    }
}
