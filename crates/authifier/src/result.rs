#[derive(Serialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[serde(tag = "type")]
pub enum Error {
    #[cfg_attr(feature = "utoipa", schema(title = "IncorrectData"))]
    IncorrectData {
        with: &'static str,
    },
    #[cfg_attr(feature = "utoipa", schema(title = "DatabaseError"))]
    DatabaseError {
        operation: &'static str,
        with: &'static str,
    },
    #[cfg_attr(feature = "utoipa", schema(title = "InternalError"))]
    InternalError,
    #[cfg_attr(feature = "utoipa", schema(title = "OperationFailed"))]
    OperationFailed,

    #[cfg_attr(feature = "utoipa", schema(title = "RenderFail"))]
    RenderFail,
    #[cfg_attr(feature = "utoipa", schema(title = "MissingHeaders"))]
    MissingHeaders,
    #[cfg_attr(feature = "utoipa", schema(title = "CaptchaFailed"))]
    CaptchaFailed,
    #[cfg_attr(feature = "utoipa", schema(title = "BlockedByShield"))]
    BlockedByShield,

    #[cfg_attr(feature = "utoipa", schema(title = "InvalidSession"))]
    InvalidSession,
    #[cfg_attr(feature = "utoipa", schema(title = "UnverifiedAccount"))]
    UnverifiedAccount,
    #[cfg_attr(feature = "utoipa", schema(title = "UnknownUser"))]
    UnknownUser,

    #[cfg_attr(feature = "utoipa", schema(title = "EmailFailed"))]
    EmailFailed,
    #[cfg_attr(feature = "utoipa", schema(title = "InvalidToken"))]
    InvalidToken,
    #[cfg_attr(feature = "utoipa", schema(title = "MissingInvite"))]
    MissingInvite,
    #[cfg_attr(feature = "utoipa", schema(title = "InvalidInvite"))]
    InvalidInvite,
    #[cfg_attr(feature = "utoipa", schema(title = "InvalidCredentials"))]
    InvalidCredentials,

    #[cfg_attr(feature = "utoipa", schema(title = "CompromisedPassword"))]
    CompromisedPassword,
    #[cfg_attr(feature = "utoipa", schema(title = "ShortPassword"))]
    ShortPassword,
    #[cfg_attr(feature = "utoipa", schema(title = "Blacklisted"))]
    Blacklisted,
    #[cfg_attr(feature = "utoipa", schema(title = "LockedOut"))]
    LockedOut,

    #[cfg_attr(feature = "utoipa", schema(title = "TotpAlreadyEnabled"))]
    TotpAlreadyEnabled,
    #[cfg_attr(feature = "utoipa", schema(title = "DisallowedMFAMethod"))]
    DisallowedMFAMethod,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
pub type Success = Result<()>;
