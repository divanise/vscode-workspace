package hello;

import io.javalin.Javalin;

public class App {

    public static void main(String[] args) throws Exception {
        Javalin
                .create(config -> {
                    config.showJavalinBanner = false;
                })
                .get("/", ctx -> ctx.result("Hello World"))
                .start(8080);
    }
}
