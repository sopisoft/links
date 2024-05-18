import { Style, css } from "hono/css";
import { jsxRenderer } from "hono/jsx-renderer";

const jsx = jsxRenderer(
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
          <link rel="icon" href="/favicon" />
          {metas?.map((meta) => (
            // biome-ignore lint/correctness/useJsxKeyInIterable: <explanation>
            <meta.tag {...meta.props} />
          ))}
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
                font-family: "Noto Sans JP", sans-serif;
                font-optical-sizing: auto;
                font-weight: "normal";
                font-style: "normal";
                font-size: "normal";
                line-height: 1.5;
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

export default jsx;
