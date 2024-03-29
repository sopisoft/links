import { Hono } from "hono";
import { cache } from "hono/cache";
const app = new Hono();

app.get(
  "/",
  cache({
    cacheName: "proxy",
    cacheControl: `max-age=${60 * 60 * 24 * 7}`,
    wait: true,
  })
);

app.all("/", async (c) => {
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

export default app;
