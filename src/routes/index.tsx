import { Hono } from "hono";

const app = new Hono();

app.get("/", (c) => {
  return c.redirect("/tree");
});

export default app;
