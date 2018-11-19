//! Telegram bot prelude.
//!
//! This module re-exports request builder traits from telegram-bot-raw.

pub use telegram_bot_fork_raw::{
    CanAnswerCallbackQuery, CanDeleteMessage, CanEditMessageCaption, CanEditMessageLiveLocation,
    CanEditMessageReplyMarkup, CanEditMessageText, CanForwardMessage, CanGetChat,
    CanGetChatAdministrators, CanGetChatMemberForChat, CanGetChatMemberForUser,
    CanGetChatMembersCount, CanGetFile, CanGetUserProfilePhotos, CanKickChatMemberForChat,
    CanKickChatMemberForUser, CanLeaveChat, CanPinMessage, CanReplySendAudio, CanReplySendContact,
    CanReplySendLocation, CanReplySendMessage, CanReplySendVenue, CanSendAudio, CanSendChatAction,
    CanSendContact, CanSendLocation, CanSendMessage, CanSendVenue, CanStopMessageLiveLocation,
    CanUnbanChatMemberForChat, CanUnbanChatMemberForUser, CanUnpinMessage, ToReplyRequest,
    ToRequest,
};
