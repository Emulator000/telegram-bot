use requests::*;
use types::*;

/// Use this method to get a list of administrators in a chat.
/// If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ExportChatInviteLink {
    chat_id: ChatRef,
}

impl Request for ExportChatInviteLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<String>;

    fn serialize(&self) -> Result<HttpRequest> {
        Self::Type::serialize(RequestUrl::method("exportChatInviteLink"), self)
    }
}

impl ExportChatInviteLink {
    pub fn new<C>(chat: C) -> Self
    where
        C: ToChatRef,
    {
        ExportChatInviteLink {
            chat_id: chat.to_chat_ref(),
        }
    }
}

/// Get a list of administrators in a chat.
pub trait CanExportChatInviteLink {
    fn export_chat_invite_link(&self) -> ExportChatInviteLink;
}

impl<C> CanExportChatInviteLink for C
where
    C: ToChatRef,
{
    fn export_chat_invite_link(&self) -> ExportChatInviteLink {
        ExportChatInviteLink::new(self)
    }
}
