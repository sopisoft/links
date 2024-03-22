import { css } from "hono/css";
import { createRoute } from "honox/factory";
import LinkItem from "../components/link_item";
import { links } from "../links";

export default createRoute((c) => {
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
        font-family: "Noto Sans JP", sans-serif;
        font-optical-sizing: auto;
        font-weight: "normal";
        font-style: "normal";
        font-size: "normal";
        line-height: 1.5;
      `}
    >
      <h1
        class={css`
          font-size: 2rem;
          font-weight: bold;
        `}
      >
        Links Tree
      </h1>
      {links.map((link) => (
        <LinkItem {...link} />
      ))}
    </div>,
    {
      title: "Links Tree",
      metas: [
        {
          tag: "meta",
          props: {
            name: "description",
            content: "My Links",
          },
        },
      ],
    }
  );
});
