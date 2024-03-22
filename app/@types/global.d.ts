/// <reference types="hono" />
/// <reference types="vite/client" />
/// <reference types="@cloudflare/workers-types" />

import {} from "hono";

declare module "hono" {
  type ContextRenderer = (
    content: string | Promise<string>,
    props: {
      title: string;
      metas: {
        tag: "meta" | "link" | "script" | "style";
        props: { [key: string]: string };
      }[];
    }
  ) => Response;
}

interface Link {
  link: string;
  text: string;
  description: string;
}
