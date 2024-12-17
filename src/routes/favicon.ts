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

app.get("/get", async (c) => {
  const target = c.req.query("url");
  if (!target) {
    return c.text("url query is required", { status: 400 });
  }

  const base = "https://summerflare.sopi.workers.dev/";
  const path = "url";
  const url = new URL(path, base);
  url.searchParams.set("url", encodeURI(target));
  const req = new Request(url);

  const res = await c.env.summerflare.fetch(req);
  const icon = await res
    .json()
    .then((json: unknown) => (json as { icon: string }).icon);
  const ireq = new Request(icon);
  const ires = await fetch(ireq);
  const ibuffer = await ires.arrayBuffer();

  if (ires.ok && ibuffer.byteLength > 0) {
    const headers = new Headers(ires.headers);
    headers.set("cache-control", `max-age=${60 * 60 * 24 * 7}`);

    const response = new Response(ibuffer, {
      status: ires.status,
      statusText: ires.statusText,
      headers,
    });

    return response;
  }
});

export default app;
