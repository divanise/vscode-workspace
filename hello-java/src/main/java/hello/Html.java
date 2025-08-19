package hello;

import java.util.Map;

import static io.javalin.apibuilder.ApiBuilder.get;
import static io.javalin.apibuilder.ApiBuilder.path;
import io.javalin.http.Context;

class Html {

    static void routes(Services services) {
        var handler = new Html(services);

        path("/", () -> {
            get(handler::greeter);
        });
    }

    private final Services services;

    private Html(Services services) {
        this.services = services;
    }

    private void greeter(Context ctx) {
        var greeting = this.services.greet(ctx.queryParam("name"));

        ctx.header("content-type", "text/html; charset=utf-8");
        ctx.render("greeter.html", Map.of("greeting", greeting));
    }
}
