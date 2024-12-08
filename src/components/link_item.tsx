import { css } from "hono/css";
import type { Link } from "../types";

function LinkItem(Link: Link) {
  const { link, id, text, description } = Link;

  return (
    <a
      href={link}
      target="_blank"
      rel="noopener noreferrer"
      class={css`
        display: flex;
        flex-direction: row;
        align-items: center; 
        padding: 0.5rem 1rem;
        background: rgba(247, 247, 247, 0.8);
        border-radius: 1rem;
        text-decoration : none;
        border : 1px solid rgba(0, 0, 0, 0.1);
        &:hover {
          background: rgba(247, 247, 247, 1);
        }
      `}
    >
      <img
        src={`/proxy?url=${encodeURIComponent(`https://www.google.com/s2/favicons?sz=${128}&domain=${new URL(link).host}`)}`}
        alt={`${new URL(link).host} favicon`}
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
      <div
        class={css`
          display: flex;
          flex-direction: column;
          margin-left: 1rem;
        `}
      >
        <span
          class={css`
            display: flex;
            flex-direction: row;
            align-items: baseline;
            gap : 0.5rem;
            font-weight: bold;
            font-size: 1.5rem;
          `}
        >
          {text}
          <span
            class={css`
            font-size: 0.8rem;
          `}
          >
            ( {id} )
          </span>
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
