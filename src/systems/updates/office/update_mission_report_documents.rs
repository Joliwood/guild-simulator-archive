use crate::{
    structs::missions::MissionReports,
    ui::rooms::office::mission_report_documents::{MissionReportTextTrigger, MissionReportTrigger},
};
use bevy::prelude::*;

pub fn update_mission_report_documents(
    mission_reports: Res<MissionReports>,
    mut query: Query<(&mut Node, &mut MissionReportTrigger)>,
    text_query: Single<Entity, (With<MissionReportTextTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    let mission_reports_number = mission_reports.0.len();

    for (mut node, _) in query.iter_mut() {
        if mission_reports_number > 0 {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }

    *writer.text(*text_query, 0) = format!("{}", mission_reports_number);
}
