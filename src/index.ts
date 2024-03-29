import { Hono } from "hono";
import { showRoutes } from "hono/dev";
import renderer from "./_renderer";
import notFound from "./routes/_404";
import onError from "./routes/_error";
import index from "./routes/index";
import tree from "./routes/tree";
import type { Bindings } from "./types";

const app = new Hono<{ Bindings: Bindings }>();

app.get("*", renderer);

app.route("/", index);
app.route("/tree", tree);

app.get("/favicon", async (c) => {
  const base = "https://img.sopi.workers.dev/";
  const path = "api/emoji/v1/🐢.png?provider=fluent&size=32";
  const url = new URL(path, base);
  const req = new Request(url);

  const res = await c.env.img.fetch(req);
  const png = await res.arrayBuffer();

  return c.newResponse(png, {
    headers: {
      "Content-Type": "image/png",
    },
  });
});

app.all("/proxy", async (c) => {
  const url = c.req.query("url");
  if (!url) return c.text("Url is required", { status: 400 });

  const req = new Request(url);
  const res = await fetch(req);
  const buf = await res.arrayBuffer();

  return c.newResponse(new Uint8Array(buf), {
    headers: {
      "Content-Type": res.headers.get("Content-Type") ?? "text/plain",
    },
  });
});

app.notFound(notFound);
app.onError(onError);

showRoutes(app);

export default app;
