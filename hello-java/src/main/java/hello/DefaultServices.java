package hello;

import java.util.Objects;

class DefaultServices implements Services {

    @Override
    public String greet(String name) {
        name = Objects.requireNonNullElse(name, "world");

        return "Hello, " + name + "!";
    }
}
