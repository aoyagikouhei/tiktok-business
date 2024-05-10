pub enum TiktokScope {
    CommentList,
    CommentListManage,
    ResearchAdlibBasic,
    ResearchDataBasic,
    UserAccountType,
    UserInfoBasic,
    UserInfoProfile,
    UserInfoStats,
    UserInfoUsername,
    UserInsights,
    VideoInsights,
    VideoList,
    VideoPublish,
    VideoUpload,
}

impl TiktokScope {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CommentList,
            Self::CommentListManage,
            Self::ResearchAdlibBasic,
            Self::ResearchDataBasic,
            Self::UserAccountType,
            Self::UserInfoBasic,
            Self::UserInfoProfile,
            Self::UserInfoUsername,
            Self::UserInfoStats,
            Self::UserInsights,
            Self::VideoInsights,
            Self::VideoList,
            Self::VideoPublish,
            Self::VideoUpload,
        ]
    }

    pub fn tiktok_accounts() -> Vec<Self> {
        vec![
            Self::CommentList,
            Self::CommentListManage,
            Self::UserAccountType,
            Self::UserInfoBasic,
            Self::UserInfoUsername,
            Self::UserInfoStats,
            Self::UserInsights,
            Self::VideoInsights,
            Self::VideoList,
            Self::VideoPublish,
        ]
    }
}

impl std::fmt::Display for TiktokScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CommentList => write!(f, "comment.list"),
            Self::CommentListManage => write!(f, "comment.list.manage"),
            Self::ResearchAdlibBasic => write!(f, "research.adlib.basic"),
            Self::ResearchDataBasic => write!(f, "research.data.basic"),
            Self::UserAccountType => write!(f, "user.account.type"),
            Self::UserInfoBasic => write!(f, "user.info.basic"),
            Self::UserInfoUsername => write!(f, "user.info.username"),
            Self::UserInfoProfile => write!(f, "user.info.profile"),
            Self::UserInfoStats => write!(f, "user.info.stats"),
            Self::UserInsights => write!(f, "user.insights"),
            Self::VideoInsights => write!(f, "video.insights"),
            Self::VideoList => write!(f, "video.list"),
            Self::VideoPublish => write!(f, "video.publish"),
            Self::VideoUpload => write!(f, "video.upload"),
        }
    }
}