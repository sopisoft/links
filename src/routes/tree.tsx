import { Hono } from "hono";
import { cache } from "hono/cache";
import { css } from "hono/css";
import LinkItem from "../components/link_item";
import { links } from "../links";
import type { Bindings } from "../types";

const app = new Hono<{ Bindings: Bindings }>();

app.get(
  "*",
  cache({
    cacheName: "links",
    cacheControl: `max-age=${60 * 60 * 24 * 7 * 3}`,
    wait: true,
  })
);

app.get("/", (c) => {
  return c.render(
    <div
      class={css`
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        width: min(100%, 32rem);
        margin: 0;
        padding: 1rem min(1rem, 5%);
        border-radius: 1rem;
      `}
    >
      <h1>Links Tree</h1>
      {links.map((link) => (
        // biome-ignore lint/correctness/useJsxKeyInIterable: <explanation>
        <LinkItem {...link} />
      ))}
    </div>,
    {
      title: "Links",
      metas: [
        {
          tag: "meta",
          props: {
            name: "description",
            content: "sopi's social links tree",
          },
        },
      ],
    }
  );
});

export default app;
