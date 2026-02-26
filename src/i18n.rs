use crate::links::LinkId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lang {
    En,
    Ja,
}

impl Lang {
    pub const fn tag(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Ja => "ja",
        }
    }
}

pub const fn title(lang: Lang) -> &'static str {
    match lang {
        Lang::En | Lang::Ja => "sopi link tree",
    }
}

pub const fn description(lang: Lang, id: LinkId) -> &'static str {
    match lang {
        Lang::En => description_en(id),
        Lang::Ja => description_ja(id),
    }
}

const fn description_en(id: LinkId) -> &'static str {
    match id {
        LinkId::Twitter => "I like and retweet.",
        LinkId::Misskey => "I enjoy reaction shooting.",
        LinkId::Mixi => "I participate in various communities.",
        LinkId::Annict => "I record the anime I watched.",
        LinkId::Github => "GitHub account.",
        LinkId::Discord => "Used mostly for viewing.",
        LinkId::Bluesky => "I do nothing there.",
    }
}

const fn description_ja(id: LinkId) -> &'static str {
    match id {
        LinkId::Twitter => "いいねやリツイートをしています",
        LinkId::Misskey => "リアクションシューティングをしています",
        LinkId::Mixi => "いろんなコミュニティに参加しています",
        LinkId::Annict => "視聴したアニメを記録しています",
        LinkId::Github => "GitHub のアカウントです",
        LinkId::Discord => "見るだけ",
        LinkId::Bluesky => "何もしていません",
    }
}
