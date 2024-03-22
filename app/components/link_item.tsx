import type { SummalyResult } from "@misskey-dev/summaly/built/summary";
import { css } from "hono/css";
import { Suspense } from "hono/jsx";
import { textStyle } from "../style";

async function Favicon(props: { url: string }) {
  const { url } = props;
  const host = new URL(url).host;

  const api_base = "https://summerflare.sopi.workers.dev/";
  const api_url = new URL("/url", api_base);
  api_url.searchParams.append("url", new URL(`https://${host}`).toString());
  const summalyResult = await fetch(api_url.toString());
  const summaly: SummalyResult = await summalyResult.json();
  const favicon =
    summaly?.icon ?? `https://www.google.com/s2/favicons?domain=${host}`;

  return (
    <img
      src={favicon}
      alt={`${summaly?.sitename ?? host} favicon`}
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
