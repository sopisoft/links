/// <reference types="hono" />
/// <reference types="@cloudflare/workers-types" />

import {} from "hono";

declare module "hono" {
  interface ContextRenderer extends Function {
    (
      content: string | Promise<string>,
      props: {
        title: string;
        metas: {
          tag: "meta" | "link" | "script" | "style";
          props: { [key: string]: string };
        }[];
      }
    ): Response;
  }
}

type Bindings = {
  img: Fetcher;
};

interface Link {
  link: string;
  text: string;
  description: string;
}
