import type { ErrorHandler } from "hono";

const handler: ErrorHandler = (e, c) => {
  return c.render(<h1>Error! {e.message}</h1>, {
    title: "Error",
    metas: [
      {
        tag: "meta",
        props: {
          name: "description",
          content: "An error has occurred",
        },
      },
    ],
  });
};

export default handler;
