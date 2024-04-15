use serde::{Serialize, Deserialize};
use crate::params::*;

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseParams {
    CreateSession(session::ResponseCreateSessionParams),
    DeleteSession,
    RestoreSession(session::ResponseRestoreSessionParams),
    DebugCommand,
    ServerPing,
    CheckAlive,
    GetAnnounceMessageList(announcement::ResponseGetAnnounceMessageListParams),
    CreateSign(sign::ResponseCreateSignParams),
    CreateMatchAreaSign,
    UpdateSign,
    GetSignList(sign::ResponseGetSignListParams),
    GetMatchAreaSignList,
    RemoveSign,
    SummonSign,
    RejectSign,
    CreateBloodMessage(bloodmessage::ResponseCreateBloodMessageParams),
    RemoveBloodMessage,
    ReentryBloodMessage,
    GetBloodMessageList(bloodmessage::ResponseGetBloodMessageListParams),
    EvaluateBloodMessage,
    GetBloodMessageDetail,
    CreateBloodstain(bloodstain::ResponseCreateBloodstainParams),
    GetBloodstainList(bloodstain::ResponseGetBloodstainListParams),
    GetDeadingGhost(bloodstain::ResponseGetDeadingGhostParams),
    CreateGhostData(ghostdata::ResponseCreateGhostDataParams),
    GetGhostDataList(ghostdata::ResponseGetGhostDataListParams),
    UpdateLoginPlayerCharacter(character::ResponseUpdateLoginPlayerCharacterParams),
    UpdatePlayerStatus,
    BreakInTarget,
    GetBreakInTargetList(breakin::ResponseGetBreakInTargetListParams),
    RejectBreakInTarget,
    AllowBreakInTarget,
    Visit,
    GetVisitorList,
    RejectVisit,
    NotifyAreaEvent,
    JoinMultiplay,
    LeaveMultiplay,
    GetMatchDensity,
    GetPlayZoneIdList,
    RegisterCharacterLog,
    SelectCharacterLog,
    DieLog,
    UseMagicLog,
    UseGestureLog,
    UseItemLog,
    PurchaseItemLog,
    GetItemLog,
    DropItemLog,
    LeaveItemLog,
    SaleItemLog,
    CreateItemLog,
    SummonBuddyLog,
    KillEnemyLog,
    KillBossLog,
    GlobalEventLog,
    DiscoverMapPointLog,
    JoinMultiplayLog,
    LeaveMultiplayLog,
    CreateSignResultLog,
    SummonSignResultLog,
    BreakInResultLog,
    VisitResultLog,
    QuickMatchResultLog,
    QuickMatchEndLog,
    SystemOptionLog,
    SearchQuickMatch(quickmatch::ResponseSearchQuickMatchParams),
    RegisterQuickMatch,
    UnregisterQuickMatch,
    UpdateQuickMatch,
    JoinQuickMatch,
    AcceptQuickMatch,
    RejectQuickMatch,
    GrUploadPlayerEquipments,
    GrGetPlayerEquipments(player_equipments::ResponseGrGetPlayerEquipmentsParams),
    CreateMatchingTicket,
    PollMatchingTicket,
    DeleteMatchingTicket,
    CreateBattleSession,
    CreateRoom,
    UpdateRoom,
    DeleteRoom,
    GetRoom,
    GetRoomList,
    RegisterUGC(ugc::ResponseRegisterUGCParams),
    GetUGCSNSCodeList,
    GetUGC,
    DeleteUGC,
    SendQuickMatchStart,
    SendQuickMatchResult,
}
