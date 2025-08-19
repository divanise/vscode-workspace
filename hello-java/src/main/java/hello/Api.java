package hello;

import java.util.Map;

import com.fasterxml.jackson.annotation.JsonProperty;

import static io.javalin.apibuilder.ApiBuilder.path;
import static io.javalin.apibuilder.ApiBuilder.post;
import io.javalin.http.Context;

class Api {

    static void routes(Services services) {
        var handler = new Api(services);

        path("/api/v1", () -> {
            path("/greet", () -> {
                post(handler::greet);
            });
        });
    }

    private final Services services;

    private Api(Services services) {
        this.services = services;
    }

    private static class GreetArgs {

        @JsonProperty(value = "name", required = false)
        String name;
    }

    private void greet(Context ctx) {
        var args = ctx.bodyAsClass(GreetArgs.class);
        var greeting = this.services.greet(args.name);

        ctx.json(Map.of("greeting", greeting));
    }
}
