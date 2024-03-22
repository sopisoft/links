import type { NotFoundHandler } from "hono";

const handler: NotFoundHandler = (c) => {
  return c.redirect("/tree");
};

export default handler;
