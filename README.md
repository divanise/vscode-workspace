A simple template for a VS Code development container, consisting of a workspace, a PostgreSQL instance, and a Redis instance. Data is persisted in Docker volumes, and the image is initialized via a collection of shell scripts in `/usr/local/etc` rather than directly in the `Dockerfile`.
