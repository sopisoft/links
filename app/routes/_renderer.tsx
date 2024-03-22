import { Style, css } from "hono/css";
import { jsxRenderer } from "hono/jsx-renderer";
import { Script } from "honox/server";

export default jsxRenderer(
  ({ children, title, metas }) => {
    return (
      <html lang="ja-JP">
        <head>
          <meta charset="utf-8" />
          <meta
            name="viewport"
            content="width=device-width, initial-scale=1.0"
          />
          <title>{title}</title>
          <link
            rel="icon"
            href="data:image/svg+xml,
          <svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22>
            <text x=%2250%%22 y=%2250%%22 style=%22dominant-baseline:central;text-anchor:middle;font-size:90px;%22>
              🐢
            </text>
          </svg>"
          />
          {metas?.map((meta, i) => (
            <meta.tag key={i} {...meta.props} />
          ))}
          <Script src="/app/client.ts" />
          <link rel="preconnect" href="https://fonts.googleapis.com" />
          <link
            rel="preconnect"
            href="https://fonts.gstatic.com"
            crossorigin="anonymous"
          />
          <link
            href="https://fonts.googleapis.com/css2?family=Noto+Sans+JP:wght@100..900&display=swap"
            rel="stylesheet"
          />
          <Style>
            {css`
              body {
                display: flex;
                justify-content: center;
                width: 100svw;
                height: 100svh;
                min-height: 100svh;
                margin: 0;
                padding: 0;
                box-sizing: border-box;
              }
            `}
          </Style>
        </head>
        <body>{children}</body>
      </html>
    );
  },
  {
    docType: true,
    stream: true,
  }
);
