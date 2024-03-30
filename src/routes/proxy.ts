import { Hono } from "hono";
import { cache } from "hono/cache";
import type { Bindings } from "../types";

const app = new Hono<{ Bindings: Bindings }>();

app.get(
  "/",
  cache({
    cacheName: "proxy",
    cacheControl: `max-age=${60 * 60 * 24 * 7 * 3}`,
    wait: true,
  })
);

app.all("/", async (c) => {
  const url = c.req.query("url");
  if (!url) return c.text("Url is required", { status: 400 });

  const req = new Request(url);
  const res = await fetch(req);
  const buffer = await res.arrayBuffer();

  if (res.ok && buffer.byteLength > 0) {
    const headers = new Headers(res.headers);
    headers.set("cache-control", `max-age=${60 * 60 * 24 * 7}`);

    const response = new Response(buffer, {
      status: res.status,
      statusText: res.statusText,
      headers,
    });

    return response;
  }
});

export default app;
