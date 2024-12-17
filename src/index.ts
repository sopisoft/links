import { Hono } from "hono";
import { csrf } from "hono/csrf";
import { secureHeaders } from "hono/secure-headers";
import renderer from "./_renderer";
import notFound from "./routes/_404";
import onError from "./routes/_error";
import favicon from "./routes/favicon";
import index from "./routes/index";
import tree from "./routes/tree";
import type { Bindings } from "./types";

const app = new Hono<{ Bindings: Bindings }>();
app.use(csrf());
app.use(secureHeaders());

app.get("*", renderer);
app.get("/robots.txt", (c) => {
  let txt = "";
  txt += "User-agent: *\n";
  txt += "Allow: /tree\n";
  txt += "Disallow: /\n";

  return c.text(txt);
});

app.route("/favicon", favicon);

app.route("/", index);
app.route("/tree", tree);

app.notFound(notFound);
app.onError(onError);

export default app;
