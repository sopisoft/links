#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LinkId {
    Twitter,
    Misskey,
    Mixi,
    Annict,
    Github,
    Discord,
    Bluesky,
}

impl LinkId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Twitter => "twitter",
            Self::Misskey => "misskey",
            Self::Mixi => "mixi",
            Self::Annict => "annict",
            Self::Github => "github",
            Self::Discord => "discord",
            Self::Bluesky => "bluesky",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "twitter" => Some(Self::Twitter),
            "misskey" => Some(Self::Misskey),
            "mixi" => Some(Self::Mixi),
            "annict" => Some(Self::Annict),
            "github" => Some(Self::Github),
            "discord" => Some(Self::Discord),
            "bluesky" => Some(Self::Bluesky),
            _ => None,
        }
    }

    pub const fn href(self) -> &'static str {
        match self {
            Self::Twitter => "https://x.com/sopi_dev",
            Self::Misskey => "https://misskey.io/@gobo",
            Self::Mixi => "https://mixi.social/@sopi",
            Self::Annict => "https://annict.com/@sopi",
            Self::Github => "https://github.com/sopisoft",
            Self::Discord => "https://discord.com/users/915858613996761118",
            Self::Bluesky => "https://bsky.app/profile/sopi.bsky.social",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Link {
    pub id: LinkId,
    pub text: &'static str,
    pub handle: &'static str,
    pub href: &'static str,
}

pub const LINKS: [Link; 7] = [
    Link {
        id: LinkId::Twitter,
        text: "Twitter",
        handle: "@sopi_dev",
        href: "https://x.com/sopi_dev",
    },
    Link {
        id: LinkId::Misskey,
        text: "Misskey",
        handle: "@gobo",
        href: "https://misskey.io/@gobo",
    },
    Link {
        id: LinkId::Mixi,
        text: "mixi",
        handle: "@sopi",
        href: "https://mixi.social/@sopi",
    },
    Link {
        id: LinkId::Annict,
        text: "Annict",
        handle: "@sopi",
        href: "https://annict.com/@sopi",
    },
    Link {
        id: LinkId::Github,
        text: "GitHub",
        handle: "@sopisoft",
        href: "https://github.com/sopisoft",
    },
    Link {
        id: LinkId::Discord,
        text: "Discord",
        handle: "@sopi_exe",
        href: "https://discord.com/users/915858613996761118",
    },
    Link {
        id: LinkId::Bluesky,
        text: "Bluesky",
        handle: "@sopi_bsky",
        href: "https://bsky.app/profile/sopi.bsky.social",
    },
];
