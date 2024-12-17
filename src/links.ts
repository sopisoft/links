import type { Link } from "./types";

const links: Link[] = [
  {
    link: "https://twitter.com/sopi_dev",
    id: "sopi_dev",
    text: "Twitter",
    description: "いいねやリツイートをしています",
  },
  {
    link: "https://misskey.io/@gobo",
    id: "gobo",
    text: "Misskey.io",
    description: "リアクションシューティングをしています",
  },
  {
    link: "https://mixi.social/@sopi",
    id: "sopi",
    text: "mixi2",
    description: "いろんなコミュニティに参加しています",
  },
  {
    link: "https://annict.com/@sopi",
    id: "sopi",
    text: "Annict",
    description: "視聴したアニメを記録しています",
  },
  {
    link: "https://github.com/sopisoft",
    id: "sopisoft",
    text: "GitHub",
    description: "GitHub account.",
  },
  {
    link: "https://zenn.dev/sopi",
    id: "sopi",
    text: "Zenn",
    description: "あんまり投稿していません",
  },
  {
    link: "https://discord.com/users/915858613996761118",
    id: "sopi.exe",
    text: "Discord",
    description: "みるだけ",
  },
  {
    link: "https://bsky.app/profile/sopi.bsky.social",
    id: "sopi.bsky.social",
    text: " Bluesky",
    description: "なんにもしていません",
  },
] as const;

export { links };
