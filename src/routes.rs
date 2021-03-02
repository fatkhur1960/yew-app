//! You can register routes here.
//! Note: The route definition need exact name of Component View

register_routes!([
    route("/", HomeView),
    route("/about", AboutView),
    route("/projects", BlogView),
    route("/project/{name}", ContactView)
]);
