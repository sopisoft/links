import { Hono } from "hono";
import { showRoutes } from "hono/dev";
import renderer from "./_renderer";
import notFound from "./routes/_404";
import onError from "./routes/_error";
import favicon from "./routes/favicon";
import index from "./routes/index";
import proxy from "./routes/proxy";
import tree from "./routes/tree";
import type { Bindings } from "./types";

const app = new Hono<{ Bindings: Bindings }>();

app.get("*", renderer);

app.route("/favicon", favicon);

app.route("/", index);
app.route("/tree", tree);

app.route("/proxy", proxy);

app.notFound(notFound);
app.onError(onError);

showRoutes(app);

export default app;
