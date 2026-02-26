pub const ICON_SIZE: u32 = 128;
pub const OGP_WIDTH: u32 = 1200;
pub const OGP_HEIGHT: u32 = 630;

pub const ICON_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" role="img">
  <rect width="128" height="128" rx="32" fill="#050609" />
  <text x="50%" y="54%" text-anchor="middle" font-family="Inter, system-ui, sans-serif" font-size="64" fill="#fff" font-weight="600" dy=".1em">S</text>
</svg>
"##;

pub const OGP_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 630" role="img">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="#0f172a" />
      <stop offset="100%" stop-color="#111827" />
    </linearGradient>
  </defs>
  <rect width="1200" height="630" rx="48" fill="url(#bg)" />
  <g font-family="Inter, system-ui, sans-serif" fill="#fff" text-anchor="middle">
    <text x="50%" y="50%" font-size="72" font-weight="700" text-anchor="middle">
      sopi link tree
    </text>
  </g>
</svg>
"##;
