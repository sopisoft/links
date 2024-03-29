import { css } from "hono/css";
import { Suspense } from "hono/jsx";
import type { Link } from "../types";

async function Favicon(props: { url: string }) {
  const { url } = props;
  const host = new URL(url).host;

  const favicon_url = `https://www.google.com/s2/favicons?sz=${256}&domain=${host}`;
  return await fetch(favicon_url)
    .then(() => {
      return (
        <img
          src={`/proxy?url=${encodeURIComponent(favicon_url)}`}
          alt={`${host} favicon`}
          class={css`
                padding: 0.2rem;
                height: 2rem;
                max-width: 2rem;
                max-height: 2rem;
                aspect-ratio: 1/1;
                border-radius: 0.5rem;
                overflow: hidden;
                object-fit: cover;
              `}
        />
      );
    })
    .catch(() => {
      return (
        <div
          class={css`
                display: flex;
                align-items: center;
                justify-content: center;
                width: 2rem;
                height: 2rem;
                padding: 0.2rem;
              `}
        >
          {host[0].toUpperCase()}
        </div>
      );
    });
}

async function LinkItem(Link: Link) {
  const { text, description, link } = Link;

  return (
    <a
      href={link}
      target="_blank"
      rel="noopener noreferrer"
      class={css`
          display: flex;
          flex-direction: row;
          align-items: center;
          height: 5rem;
          padding: 0rem 1rem;
          background: #e3e2e28f;
          border-radius: 1rem;
          &:hover {
            background: #e3e2e2;
          }
        `}
    >
      <Suspense
        fallback={
          <div
            class={css`
                display: flex;
                align-items: center;
                justify-content: center;
                width: 2rem;
                height: 2rem;
                padding: 0.2rem;
              `}
          >
            ...
          </div>
        }
      >
        <Favicon url={link} />
      </Suspense>
      <div
        class={css`
            display: flex;
            flex-direction: column;
            margin-left: 1rem;
          `}
      >
        <span
          class={css`
            font-weight: bold;
            font-size: 1.5rem;
          `}
        >
          {text}
        </span>
        <span
          class={css`
            font-size: 1rem;
          `}
        >
          {description}
        </span>
      </div>
    </a>
  );
}

export default LinkItem;
