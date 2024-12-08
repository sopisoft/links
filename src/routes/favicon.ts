import { Hono } from "hono";
import { cache } from "hono/cache";
import type { Bindings } from "../types";

const app = new Hono<{ Bindings: Bindings }>();

app.get(
  "/",
  cache({
    cacheName: "favicon",
    cacheControl: `max-age=${60 * 60 * 24 * 7 * 3}`,
    wait: true,
  }),
);

app.get("/", async (c) => {
  const base = "https://img_emoji.sopi.workers.dev/";
  const path = "api/emoji/v1/🐢.png?provider=fluent&size=32";
  const url = new URL(path, base);
  const req = new Request(url);

  const res = await c.env.img_emoji.fetch(req);

  return res;
});

export default app;
