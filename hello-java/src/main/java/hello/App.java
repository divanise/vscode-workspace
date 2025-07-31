package hello;

import io.javalin.Javalin;

public class App {

    public static void main(String[] args) throws Exception {
        Javalin
                .create(config -> {
                    config.showJavalinBanner = false;
                })
                .get("/", ctx -> {
                    var loader = ClassLoader.getSystemResourceAsStream("html/index.html");
                    ctx.writeSeekableStream(loader, "text/html; charset=utf-8");
                })
                .start(8080);
    }
}
