//! Telegram Bot API methods.

pub use telegram_bot_fork_raw::{
    AnswerCallbackQuery, DeleteMessage, EditMessageCaption, EditMessageLiveLocation,
    EditMessageReplyMarkup, EditMessageText, ForwardMessage, GetChat, GetChatAdministrators,
    GetChatMember, GetChatMembersCount, GetFile, GetMe, GetUpdates, GetUserProfilePhotos,
    KickChatMember, LeaveChat, PinChatMessage, RestrictChatMember, SendAudio, SendChatAction,
    SendContact, SendDocument, SendLocation, SendMessage, SendVenue, StopMessageLiveLocation,
    UnbanChatMember, UnpinChatMessage, ExportChatInviteLink, CanExportChatInviteLink,
};
